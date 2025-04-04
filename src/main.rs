#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub static K_STACK_SIZE_PER_HART_BYTES: usize = 256 * 1024;

core::arch::global_asm!(core::include_str!("./asm/boot.S"));
core::arch::global_asm!(core::include_str!("./asm/trap.S"));

mod concurrency;
mod memory;
mod riscv;
mod system_control;
mod uart;

#[unsafe(no_mangle)]
pub extern "C" fn k_hart_halt() -> ! {
    loop {
        riscv::wfi();
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        uart::io_lock_acquire();

        uart::write_unsafe::<false>(format_args!("Hart {} panicked at: ", riscv::mhartid()));

        if let Some(location) = info.location() {
            uart::write_unsafe::<true>(format_args!("{}:{}", location.file(), location.line()));
        } else {
            uart::write_unsafe::<true>(format_args!("unknown location"));
        }

        uart::write_unsafe::<true>(format_args!("{}", info.message()));

        uart::io_lock_release();
    }

    system_control::k_poweroff();
}

fn zero_bss() {
    unsafe extern "C" {
        #[link_name = "__bss_start"]
        unsafe static mut BSS_START: core::ffi::c_void;
        #[link_name = "__bss_end"]
        unsafe static mut BSS_END: core::ffi::c_void;
    }

    unsafe {
        let mut ptr = &raw mut BSS_START as usize;
        let end = &raw mut BSS_END as usize;

        assert!(ptr <= end);

        while ptr <= end {
            (ptr as *mut u8).write_volatile(0);
            ptr += 1;
        }
    }
}

/// SAFETY: Initialized by `parse_device_tree`
mod fdtb_variables {
    use crate::concurrency::OnceLock;

    pub static NUM_HARTS: OnceLock<usize> = OnceLock::new();
    pub static MEMORY_SIZE_BYTES: OnceLock<usize> = OnceLock::new();
}

/// Initializes device tree variables from FDTB stored in big-endian format
#[cfg(target_endian = "little")]
fn parse_device_tree(fdtb_ptr: usize) {
    use fdt::Fdt;

    let fdtb = unsafe { Fdt::from_ptr(fdtb_ptr as *const u8) }.expect("Invalid FDTB pointer!");

    fdtb_variables::NUM_HARTS
        .set(fdtb.cpus().count())
        .expect("Failed to set NUM_HARTS!");

    fdtb_variables::MEMORY_SIZE_BYTES
        .set(
            fdtb.memory()
                .regions()
                .map(|region| region.size.unwrap_or(0))
                .sum(),
        )
        .expect("Failed to set MEMORY_SIZE_BYTES!");
}

#[unsafe(no_mangle)]
pub extern "C" fn k_main() -> ! {
    if riscv::mhartid() != 0 {
        // Halt if not init hart
        k_hart_halt();
    }

    riscv::mstatus::initialize_fs_and_vs();

    // TODO: Setup MPP, MIE and TLS

    zero_bss();

    let fdtb_ptr = riscv::mscratch::get();
    parse_device_tree(fdtb_ptr);

    // TODO: Kernel

    system_control::k_poweroff();
}
