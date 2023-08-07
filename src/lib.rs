#![no_std]
#![no_main]

pub mod asm;
pub mod math;

extern crate alloc;
extern crate libm;
extern crate wasmi;

pub mod mem;
pub mod syscon;
pub mod uart;
pub mod wasm;

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    uart::uart_put_str("Entered panic handler!");
    uart::uart_put_nl();

    if let Some(location) = info.location() {
        uart::uart_put_str("Panicked at: ");
        uart::uart_put_str(location.file());
        uart::uart_put_str(":");
        uart::uart_put_uint(location.line() as usize);
        uart::uart_put_nl();
    }

    syscon::poweroff();
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    mem::page::print_mem_values();

    syscon::poweroff();
}
