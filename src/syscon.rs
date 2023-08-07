use crate::{asm, uart};

const SYSCON_ADDRESS: *mut u32 = 0x10_0000 as *mut u32;

#[no_mangle]
pub extern "C" fn poweroff() -> ! {
    const SYSCON_POWEROFF: u32 = 0x5555;

    uart::uart_put_str("Powering off...");
    uart::uart_put_nl();

    unsafe {
        SYSCON_ADDRESS.write_volatile(SYSCON_POWEROFF);

        asm::halt();
    }
}
