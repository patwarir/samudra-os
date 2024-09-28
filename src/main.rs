#![no_std]
#![no_main]

core::arch::global_asm!(core::include_str!("./asm/boot.S"));
core::arch::global_asm!(core::include_str!("./asm/trap.S"));

#[no_mangle]
pub static STACK_SIZE_PER_HART: usize = 256 * 1024;

mod concurrency;
mod memory;
mod riscv;
mod system_control;
mod uart;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    print!("Hart {} panicked at: ", riscv::mhartid());
    if let Some(location) = info.location() {
        println!("{}:{}", location.file(), location.line());
    } else {
        println!("unknown location");
    }

    println!("{}", info.message());

    system_control::k_poweroff();
}

#[no_mangle]
pub extern "C" fn k_halt() -> ! {
    loop {
        riscv::wfi();
    }
}

fn zero_bss() {
    extern "C" {
        #[link_name = "__bss_start"]
        static mut BSS_START: core::ffi::c_void;
        #[link_name = "__bss_end"]
        static mut BSS_END: core::ffi::c_void;
    }

    unsafe {
        let bss_start = core::ptr::addr_of_mut!(BSS_START).cast::<u8>();
        let bss_end = core::ptr::addr_of_mut!(BSS_END).cast::<u8>();

        assert!(bss_end >= bss_start);

        let mut ptr = bss_start;
        while ptr <= bss_end {
            ptr.write_volatile(0);
            ptr = ptr.add(1);
        }
    }
}

#[cfg(target_endian = "little")]
fn device_tree(fdtb_ptr: usize) {
    unsafe {
        let ptr = (fdtb_ptr as *const u8).cast::<u32>();

        let magic = ptr.add(0).read_volatile().swap_bytes();
        assert_eq!(magic, 0xD00DFEED);

        let version = ptr.add(5).read_volatile().swap_bytes();
        assert_eq!(version, 17);
    }
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    if riscv::mhartid() != 0 {
        // Halt if not init hart
        k_halt();
    }

    riscv::mstatus::initialize_fs_and_vs();

    // TODO: Setup MPP, MIE and TLS

    zero_bss();

    let fdtb_ptr = riscv::mscratch();
    device_tree(fdtb_ptr);

    // TODO: Kernel

    system_control::k_poweroff();
}
