#![no_std]
#![no_main]

core::arch::global_asm!(core::include_str!("./asm/boot.S"));
core::arch::global_asm!(core::include_str!("./asm/trap.S"));

#[no_mangle]
pub static STACK_SIZE_PER_HART: usize = 256 * 1024;

pub mod memory;
pub mod riscv;
pub mod system_control;
pub mod uart;

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    uart::uart_put_str("Panicked at: ");
    if let Some(location) = info.location() {
        uart::uart_put_str(location.file());
        uart::uart_put_str(":");
        uart::uart_put_uint(location.line().try_into().unwrap());
    } else {
        uart::uart_put_str("unknown location");
    }
    uart::uart_put_nl();

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

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    if riscv::hart_id() != 0 {
        // Halt if not init hart
        k_halt();
    }

    zero_bss();

    // TODO: Setup FPU + vector

    uart::uart_init();

    system_control::k_poweroff();
}
