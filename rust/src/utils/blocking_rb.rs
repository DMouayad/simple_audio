// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of 
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

use std::sync::{Mutex, atomic::AtomicUsize, Arc, Condvar};

#[derive(Clone)]
pub struct BlockingRb<T>
{
    size: usize,
    num_values: Arc<AtomicUsize>,
    buf: Arc<Mutex<Vec<T>>>,
    read_pos: Arc<AtomicUsize>,
    write_pos: Arc<AtomicUsize>,
    producer_events: Arc<(Mutex<Event>, Condvar)>,
}

impl<T: Copy + Clone + Default> BlockingRb<T>
{
    pub fn new(size: usize) -> Self
    {
        BlockingRb
        {
            size,
            num_values: Arc::new(AtomicUsize::new(0)),
            buf: Arc::new(Mutex::new(vec![T::default(); size])),
            read_pos: Arc::new(AtomicUsize::new(0)),
            write_pos: Arc::new(AtomicUsize::new(0)),
            producer_events: Arc::new((Mutex::new(Event::None), Condvar::new()))
        }
    }

    /// Returns the number of free spaces in the ring buffer.
    fn num_free(&self) -> usize
    {
        let num_values = self.num_values.load(std::sync::atomic::Ordering::SeqCst);
        self.size - num_values
    }

    fn is_full(&self) -> bool
    {
        let num_values = self.num_values.load(std::sync::atomic::Ordering::SeqCst);
        num_values == self.size
    }

    fn is_empty(&self) -> bool
    {
        let num_values = self.num_values.load(std::sync::atomic::Ordering::SeqCst);
        num_values == 0
    }

    // ---------------------------------
    //             PRODUCER
    // ---------------------------------

    /// Blocks the thread until there is space in the
    /// buffer to write to. This operation can be cancelled
    /// by calling `cancel`.
    /// 
    /// Returns the number of items written.
    /// Returns `None` if the given slice is empty
    /// or the operation was cancelled.
    pub fn write(&self, slice: &[T]) -> Option<usize>
    {
        if slice.is_empty() { return None; }

        let num_free = self.num_free();
        // Block if the buffer doesn't have space for the slice.
        if num_free < slice.len() || self.is_full()
        {
            // Wait for the event to tell us that there free space
            // available or that the operation should be cancelled.
            let (mutex, cvar) = &*self.producer_events;
            let mut event = mutex.lock().unwrap();
            event = cvar.wait(event).unwrap();

            match *event
            {
                Event::CancelWrite => return None,
                Event::FreeSpace => (),
                _ => panic!("This event is not supported by `write()`.")
            }
        }

        let mut buf = self.buf.lock().unwrap();

        // Write as much of the given slice as possible.
        // If the slice is larger than the buffer, then write until
        // the buffer size.
        let count = slice.len().min(num_free);

        let write_pos = self.write_pos.load(std::sync::atomic::Ordering::SeqCst);

        // The data can fit in line in the buffer.
        if write_pos + count < self.size
        {
            buf[write_pos..write_pos + count]
                .copy_from_slice(&slice[..count]);
        }
        // The data is towards the end of the buffer and
        // needs to be wrapped.
        else
        {
            // How much data can be written before wrapping.
            let num_end = self.size - write_pos;
            buf[write_pos..].copy_from_slice(&slice[..num_end]);
            buf[..count - num_end].copy_from_slice(&slice[num_end..count]);
        }

        let write_pos = (write_pos + count) % self.size;
        self.write_pos.store(write_pos, std::sync::atomic::Ordering::SeqCst);
        self.num_values.fetch_add(count, std::sync::atomic::Ordering::SeqCst);

        Some(count)
    }

    /// Cancels the current write operation.
    pub fn cancel_write(&self)
    {
        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::CancelWrite;
        cvar.notify_all();
    }

    // ---------------------------------
    //             CONSUMER
    // ---------------------------------

    /// Reads from the ring buffer and fills the given slice
    /// with as much data as possible.
    /// 
    /// Returns the number of items written.
    /// Returns `None` if the given slice is empty
    /// or the buffer is empty.
    pub fn read(&self, slice: &mut [T]) -> Option<usize>
    {
        if slice.is_empty() || self.is_empty() { return None; }

        let buf = self.buf.lock().unwrap();

        // Fill as much of the slice as possible.
        // If the slice is larger than the buffer, then read until
        // the buffer size.
        let count = slice.len().min(self.size);

        let read_pos = self.read_pos.load(std::sync::atomic::Ordering::SeqCst);

        // The data can be read in line from the buffer.
        if read_pos + count < self.size
        {
            slice[..count].copy_from_slice(
                &buf[read_pos..read_pos + count]);
        }
        // The read position is towards the end of the buffer and
        // needs to be wrapped.
        else
        {
            // How much data can be written before wrapping.
            let num_end = self.size - read_pos;
            slice[..num_end].copy_from_slice(&buf[read_pos..]);
            slice[num_end..count].copy_from_slice(&buf[..count - num_end]);
        }

        self.read_pos.store((read_pos + count) % self.size, std::sync::atomic::Ordering::SeqCst);
        
        let num_values = self.num_values.load(std::sync::atomic::Ordering::SeqCst);
        self.num_values.store(num_values.checked_sub(count).unwrap_or(0), std::sync::atomic::Ordering::SeqCst);

        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::FreeSpace;
        cvar.notify_all();

        Some(count)
    }

    /// Sets the read position to the write position.
    /// This lets the consumer skip reading all the data
    /// in between in case it is useless.
    pub fn skip_all(&self)
    {
        let write_pos = self.write_pos.load(std::sync::atomic::Ordering::SeqCst);
        self.read_pos.store(write_pos, std::sync::atomic::Ordering::SeqCst);

        // This method basically "reads" until the write position.
        // When reading, the following has to be done.
        self.num_values.store(0, std::sync::atomic::Ordering::SeqCst);

        let (mutex, cvar) = &*self.producer_events;
        *mutex.lock().unwrap() = Event::FreeSpace;
        cvar.notify_all();
    }
}

/// Ring buffer events.
#[derive(Clone, Copy)]
enum Event
{
    None,
    /// There is free space in the buffer (sent after the buffer was read).
    FreeSpace,
    /// The write operation has been cancelled.
    CancelWrite
}

mod tests
{
    /// Expected output:
    /// [1, 2, 3, 4, 5, 6, 7, 0, 0, 0]
    #[test]
    fn test_write()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 5);

        let data = vec![6, 7];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 3);
    }

    /// Expected output:
    /// [11, 12, 3, 4, 5, 6, 7, 8, 9, 10]
    /// 
    /// *Thread Blocked*
    #[test]
    fn test_write_wrap()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();
        let reader = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 5);

        let mut read_buf = vec![0; 2];
        let _ = reader.read(&mut read_buf);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 7);

        let data = vec![6, 7, 8, 9, 10, 11, 12];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 0);

        // This should block to prevent overwriting.
        let data = vec![13, 14, 15];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
    }

    #[test]
    fn test_read()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();
        let reader = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 5);

        let mut read_buf = vec![0; 2];
        let _ = reader.read(&mut read_buf);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 7);

        let mut read_buf = vec![0; 2];
        let _ = reader.read(&mut read_buf);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 9);
    }

    #[test]
    fn test_read_wrap()
    {
        let rb = crate::utils::blocking_rb::BlockingRb::<u32>::new(10);
        let writer = rb.clone();
        let reader = rb.clone();

        let data = vec![1, 2, 3, 4, 5];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 5);

        let mut read_buf = vec![0; 5];
        let _ = reader.read(&mut read_buf);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 10);

        let data = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let _ = writer.write(&data);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 0);

        let mut read_buf = vec![0; 7];
        let _ = reader.read(&mut read_buf);
        println!("{:?}", *rb.buf.lock().unwrap());
        assert!(rb.num_free() == 7);
    }
}