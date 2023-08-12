#![no_main]

use std::ffi::c_uchar;

const VERSION: u32 = 20230807_1;

extern "C" {
    pub fn get_version() -> u32;
    pub fn host_hello(value: i32);
    pub fn mut_counter();
}

#[no_mangle]
pub extern "C" fn module_init(argc: u32, argv: *const *const c_uchar) -> u32 {
    unsafe {
        if argc != 0 {
            return 1;
        }

        if get_version() != VERSION {
            return 1;
        }

        host_hello(-23);

        for _ in 0..5 {
            mut_counter();
        }

        0
    }
}
