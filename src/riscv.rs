use core::arch::asm;

pub fn wfi() {
    unsafe {
        asm!("wfi", options(nomem, nostack));
    }
}

pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) hart_id, options(nomem, nostack));
    }
    hart_id
}
