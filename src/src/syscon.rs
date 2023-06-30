use crate::uart;

const SYSCON_ADDRESS: *mut u32 = 0x10_0000 as *mut u32;

extern "C" {
    /* Extern Assembly functions */

    pub fn halt() -> !;
}

#[no_mangle]
pub extern "C" fn poweroff() -> ! {
    const SYSCON_POWEROFF: u32 = 0x5555;
    unsafe {
        uart::uart_put_str("Powering off...");
        uart::uart_put_nl();

        SYSCON_ADDRESS.write_volatile(SYSCON_POWEROFF);

        halt();
    }
}

#[no_mangle]
pub extern "C" fn reboot() -> ! {
    const SYSCON_REBOOT: u32 = 0x7777;
    unsafe {
        uart::uart_put_str("Rebooting...");
        uart::uart_put_nl();

        SYSCON_ADDRESS.write_volatile(SYSCON_REBOOT);

        halt();
    }
}
