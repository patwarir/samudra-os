unsafe extern "C" {
    #[link_name = "__memory_start"]
    unsafe static mut MEMORY_START: core::ffi::c_void;
}
