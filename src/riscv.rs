use core::arch::asm;

pub fn wfi() {
    unsafe {
        asm!("wfi", options(nomem, nostack));
    }
}

pub fn mhartid() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) hart_id, options(nomem, nostack));
    }
    hart_id
}

pub fn mscratch() -> usize {
    let mscratch: usize;
    unsafe {
        asm!("csrr {}, mscratch", out(reg) mscratch, options(nomem, nostack));
    }
    mscratch
}

pub mod mstatus {
    use super::*;

    pub fn get() -> usize {
        let mstatus: usize;
        unsafe {
            asm!("csrr {}, mstatus", out(reg) mstatus, options(nomem, nostack));
        }
        mstatus
    }

    pub fn initialize_fs_and_vs() {
        const FS_OFFSET: usize = 13;
        const VS_OFFSET: usize = 9;
        unsafe {
            asm!("csrw mstatus, {}", in(reg) get() | (0b01 << FS_OFFSET) | (0b01 << VS_OFFSET), options(nomem, nostack));
        }
    }
}
