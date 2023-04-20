#![no_std]
#![no_main]

pub mod uart;

extern "C" {
    /* Extern Assembly functions */

    pub fn halt() -> !;

    /* Extern C functions */

    pub fn call_me_from_rust();

    pub fn c_calls_halt() -> !;
}

#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    uart::uart_put_str("Entered panic handler!\r\n");

    unsafe { halt(); }
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    uart::uart_init();

    uart::uart_put_str("Hello, World from Rust!\r\n");

    unsafe { call_me_from_rust(); }

    unsafe { c_calls_halt(); }
}
