#![no_std]
#![no_main]

#[no_mangle]
pub static K_STACK_SIZE_PER_HART: usize = 256 * 1024;

core::arch::global_asm!(core::include_str!("./asm/boot.S"));
core::arch::global_asm!(core::include_str!("./asm/trap.S"));

mod concurrency;
mod memory;
mod riscv;
mod system_control;
mod uart;

#[no_mangle]
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
        let mut ptr = core::ptr::addr_of_mut!(BSS_START) as usize;
        let end = core::ptr::addr_of_mut!(BSS_END) as usize;

        assert!(ptr <= end);

        while ptr <= end {
            (ptr as *mut u8).write_volatile(0);
            ptr += 1;
        }
    }
}

#[cfg(target_endian = "little")]
fn device_tree(fdtb_ptr: usize) {
    unsafe {
        assert_ne!(fdtb_ptr, 0);

        let ptr = fdtb_ptr as *const u32;

        // The FDTB is stored in big-endian format

        const FDTB_MAGIC: u32 = 0xD00DFEED;
        let magic = ptr.add(0).read_volatile().swap_bytes();
        assert_eq!(magic, FDTB_MAGIC);

        const FDTB_VERSION: u32 = 17;
        let version = ptr.add(5).read_volatile().swap_bytes();
        assert_eq!(version, FDTB_VERSION);
    }
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    if riscv::mhartid() != 0 {
        // Halt if not init hart
        k_hart_halt();
    }

    riscv::mstatus::initialize_fs_and_vs();

    // TODO: Setup MPP, MIE and TLS

    zero_bss();

    let fdtb_ptr = riscv::mscratch::get();
    device_tree(fdtb_ptr);

    // TODO: Kernel

    system_control::k_poweroff();
}
