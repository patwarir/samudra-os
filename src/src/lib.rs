#![no_std]
#![no_main]

pub mod uart;

extern "C" {
    /* Extern Assembly functions */

    pub fn halt() -> !;

    /* Extern C functions */

    pub fn call_me_from_rust();

    pub fn c_calls_halt() -> !;

    pub fn c_raises_interrupt();
}

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    uart::uart_put_str("Entered panic handler!\r\n");

    if let Some(location) = info.location() {
        uart::uart_put_str("Panicked at: ");
        uart::uart_put_str(location.file());
        uart::uart_put_c_uchar(b':');
        uart::uart_put_uint(location.line() as usize);
        uart::uart_put_str(uart::NEWLINE);
    }

    unsafe { halt(); }
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    uart::uart_init();

    uart::uart_put_str("Hello, World from Rust!\r\n");

    unsafe { call_me_from_rust(); }

    // Enters panic handler
    // let i1 = Some(1isize);
    // let i2 = Some(0isize);
    // uart::uart_put_sint(i1.unwrap() / i2.unwrap());

    // Enters trap vector
    // unsafe { c_raises_interrupt(); }

    unsafe { c_calls_halt(); }
}
