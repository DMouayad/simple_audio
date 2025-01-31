use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_new__static_method__Player(
    port_: i64,
    actions: *mut wire_list_media_control_action,
    dbus_name: *mut wire_uint_8_list,
    hwnd: *mut i64,
) {
    wire_new__static_method__Player_impl(port_, actions, dbus_name, hwnd)
}

#[no_mangle]
pub extern "C" fn wire_dispose__static_method__Player(port_: i64) {
    wire_dispose__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_playback_state_stream__static_method__Player(port_: i64) {
    wire_playback_state_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_progress_state_stream__static_method__Player(port_: i64) {
    wire_progress_state_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_callback_stream__static_method__Player(port_: i64) {
    wire_callback_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_playing__method__Player(port_: i64, that: *mut wire_Player) {
    wire_is_playing__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_has_preloaded__method__Player(port_: i64, that: *mut wire_Player) {
    wire_has_preloaded__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_get_progress__method__Player(port_: i64, that: *mut wire_Player) {
    wire_get_progress__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_open__method__Player(
    port_: i64,
    that: *mut wire_Player,
    path: *mut wire_uint_8_list,
    autoplay: bool,
) {
    wire_open__method__Player_impl(port_, that, path, autoplay)
}

#[no_mangle]
pub extern "C" fn wire_preload__method__Player(
    port_: i64,
    that: *mut wire_Player,
    path: *mut wire_uint_8_list,
) {
    wire_preload__method__Player_impl(port_, that, path)
}

#[no_mangle]
pub extern "C" fn wire_play_preload__method__Player(port_: i64, that: *mut wire_Player) {
    wire_play_preload__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_play__method__Player(port_: i64, that: *mut wire_Player) {
    wire_play__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_pause__method__Player(port_: i64, that: *mut wire_Player) {
    wire_pause__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_stop__method__Player(port_: i64, that: *mut wire_Player) {
    wire_stop__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_loop_playback__method__Player(
    port_: i64,
    that: *mut wire_Player,
    should_loop: bool,
) {
    wire_loop_playback__method__Player_impl(port_, that, should_loop)
}

#[no_mangle]
pub extern "C" fn wire_set_volume__method__Player(port_: i64, that: *mut wire_Player, volume: f32) {
    wire_set_volume__method__Player_impl(port_, that, volume)
}

#[no_mangle]
pub extern "C" fn wire_seek__method__Player(port_: i64, that: *mut wire_Player, seconds: u64) {
    wire_seek__method__Player_impl(port_, that, seconds)
}

#[no_mangle]
pub extern "C" fn wire_set_metadata__method__Player(
    port_: i64,
    that: *mut wire_Player,
    metadata: *mut wire_Metadata,
) {
    wire_set_metadata__method__Player_impl(port_, that, metadata)
}

#[no_mangle]
pub extern "C" fn wire_normalize_volume__method__Player(
    port_: i64,
    that: *mut wire_Player,
    should_normalize: bool,
) {
    wire_normalize_volume__method__Player_impl(port_, that, should_normalize)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_Controls() -> wire_Controls {
    wire_Controls::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i64_0(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_metadata_0() -> *mut wire_Metadata {
    support::new_leak_box_ptr(wire_Metadata::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_player_0() -> *mut wire_Player {
    support::new_leak_box_ptr(wire_Player::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_media_control_action_0(len: i32) -> *mut wire_list_media_control_action {
    let wrap = wire_list_media_control_action {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_Controls(ptr: *const c_void) {
    unsafe {
        Arc::<Controls>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_Controls(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Controls>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Controls>> for wire_Controls {
    fn wire2api(self) -> RustOpaque<Controls> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Metadata> for *mut wire_Metadata {
    fn wire2api(self) -> Metadata {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Metadata>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Player> for *mut wire_Player {
    fn wire2api(self) -> Player {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Player>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Vec<MediaControlAction>> for *mut wire_list_media_control_action {
    fn wire2api(self) -> Vec<MediaControlAction> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Metadata> for wire_Metadata {
    fn wire2api(self) -> Metadata {
        Metadata {
            title: self.title.wire2api(),
            artist: self.artist.wire2api(),
            album: self.album.wire2api(),
            art_uri: self.art_uri.wire2api(),
            art_bytes: self.art_bytes.wire2api(),
        }
    }
}

impl Wire2Api<Player> for wire_Player {
    fn wire2api(self) -> Player {
        Player {
            controls: self.controls.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Controls {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_media_control_action {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Metadata {
    title: *mut wire_uint_8_list,
    artist: *mut wire_uint_8_list,
    album: *mut wire_uint_8_list,
    art_uri: *mut wire_uint_8_list,
    art_bytes: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Player {
    controls: wire_Controls,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Controls {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_Metadata {
    fn new_with_null_ptr() -> Self {
        Self {
            title: core::ptr::null_mut(),
            artist: core::ptr::null_mut(),
            album: core::ptr::null_mut(),
            art_uri: core::ptr::null_mut(),
            art_bytes: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_Metadata {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_Player {
    fn new_with_null_ptr() -> Self {
        Self {
            controls: wire_Controls::new_with_null_ptr(),
        }
    }
}

impl Default for wire_Player {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
