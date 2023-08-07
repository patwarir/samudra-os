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

fn wasm_test() {
    let wasm = include_bytes!("../wasm/example-hello-world/target/wasm32-unknown-unknown/release/example_hello_world.wasm");

    let mut module = wasm::KernelWasmModule::new((), &wasm[..]);

    module.define("get_version", || {
        return 20230807_1;
    });

    module.define("host_hello", |value: i32| {
        uart::uart_put_str("Hello, Wasm World! We got: ");
        uart::uart_put_sint(value.try_into().unwrap());
        uart::uart_put_nl();
    });

    let ret: u32 = module.run("module_init", (0u32, 0u32));

    uart::uart_put_str("Returns: ");
    uart::uart_put_uint(ret.try_into().unwrap());
    uart::uart_put_nl();
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    mem::page::init();
    mem::page::print_mem_values();

    wasm_test();

    syscon::poweroff();
}
