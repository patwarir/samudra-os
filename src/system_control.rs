use crate::k_println;

const SYSTEM_CONTROL_ADDRESS: *mut u32 = 0x100000 as *mut u32;

#[unsafe(no_mangle)]
pub extern "C" fn k_poweroff() -> ! {
    k_println!("Powering off...");

    const SYSTEM_CONTROL_POWER_SIGNAL: u32 = 0x5555;
    unsafe {
        SYSTEM_CONTROL_ADDRESS.write_volatile(SYSTEM_CONTROL_POWER_SIGNAL);
    }

    crate::k_hart_halt();
}
