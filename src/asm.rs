use core::ffi::c_void;

extern "C" {
    pub static K_MEMORY_HEAP_START: *mut c_void;
    pub static K_MEMORY_HEAP_END: *mut c_void;

    pub fn halt() -> !;
}
