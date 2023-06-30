#![no_std]
#![no_main]

pub mod syscon;
pub mod uart;

extern "C" {
    /* Extern Assembly functions */

    pub fn halt() -> !;

    /* Extern C functions */

    pub fn call_c_from_rust();
}

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    uart::uart_put_str("Entered panic handler!");
    uart::uart_put_nl();

    if let Some(location) = info.location() {
        uart::uart_put_str("Panicked at: ");
        uart::uart_put_str(location.file());
        uart::uart_put_c_uchar(b':');
        uart::uart_put_uint(location.line() as usize);
        uart::uart_put_nl();
    }

    unsafe {
        halt();
    }
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    uart::uart_init();

    uart::uart_put_str("Hello, World from Rust!");
    uart::uart_put_nl();

    unsafe {
        call_c_from_rust();

        syscon::poweroff();
    }
}
