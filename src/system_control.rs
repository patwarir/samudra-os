use crate::uart;

const SYSTEM_CONTROL_ADDRESS: *mut u32 = 0x100000 as *mut u32;

#[no_mangle]
pub extern "C" fn k_poweroff() -> ! {
    uart::uart_put_str("Powering off...");
    uart::uart_put_nl();

    const SYSTEM_CONTROL_POWER_SIGNAL: u32 = 0x5555;
    unsafe {
        SYSTEM_CONTROL_ADDRESS.write_volatile(SYSTEM_CONTROL_POWER_SIGNAL);
        crate::k_halt();
    }
}
