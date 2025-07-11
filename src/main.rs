#![no_std]
#![no_main]

extern crate alloc;

#[unsafe(link_section = ".srodata")]
#[unsafe(no_mangle)]
pub static K_STACK_SIZE_PER_HART_BYTES: usize = 256 * 1024;

#[unsafe(link_section = ".srodata")]
#[unsafe(no_mangle)]
pub static K_TLS_SIZE_PER_HART_BYTES: usize = 4 * 1024;

core::arch::global_asm!(core::include_str!("./asm/boot.S"));
core::arch::global_asm!(core::include_str!("./asm/trap.S"));

mod memory;
mod system_control;
mod uart;
mod wasm;

#[unsafe(no_mangle)]
pub extern "C" fn k_hart_halt() -> ! {
    loop {
        riscv::asm::wfi();
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        uart::io_lock_acquire();

        uart::write_unsafe::<false>(format_args!(
            "Hart {} panicked at: ",
            riscv::register::mhartid::read()
        ));

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
    use core::ffi::c_void;

    unsafe extern "C" {
        #[link_name = "__bss_start"]
        unsafe static BSS_START: c_void;
        #[link_name = "__bss_end"]
        unsafe static BSS_END: c_void;
    }

    unsafe {
        let mut ptr = (&BSS_START as *const c_void) as usize;
        let end = (&BSS_END as *const c_void) as usize;

        assert!(ptr <= end);

        while ptr <= end {
            (ptr as *mut u8).write_volatile(0);
            ptr += 1;
        }
    }
}

/// SAFETY: Initialized by `parse_device_tree`
mod fdtb_variables {
    use spin::Once;

    pub static NUM_HARTS: Once<usize> = Once::new();
    pub static MEMORY_SIZE_BYTES: Once<usize> = Once::new();
}

/// Initializes device tree variables from FDTB stored in big-endian format
#[cfg(target_endian = "little")]
fn parse_device_tree(fdtb_ptr: usize) {
    let fdtb = unsafe { fdt::Fdt::from_ptr(fdtb_ptr as *const u8) }.expect("Invalid FDTB pointer!");

    fdtb_variables::NUM_HARTS.call_once(|| fdtb.cpus().count());
    fdtb_variables::MEMORY_SIZE_BYTES.call_once(|| {
        fdtb.memory()
            .regions()
            .map(|region| region.size.unwrap_or(0))
            .sum()
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn k_main() -> ! {
    use riscv::register::{mhartid, mscratch, mstatus};

    unsafe {
        mstatus::set_mpp(mstatus::MPP::Machine);

        mstatus::set_fs(mstatus::FS::Initial);
        mstatus::set_vs(mstatus::VS::Initial);
    }

    const INIT_HART_ID: usize = 0;
    if mhartid::read() != INIT_HART_ID {
        // Halt if not init hart
        k_hart_halt();
    }

    unsafe {
        mstatus::clear_mie();
    }

    zero_bss();
    parse_device_tree(mscratch::read());

    unsafe {
        memory::init();
    }

    // TODO: Setup TLS + MIE (interrupts)

    // TODO: Kernel

    system_control::k_poweroff();
}
