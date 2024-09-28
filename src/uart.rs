use crate::concurrency::SpinLock;
use core::ffi::c_uchar;
use core::fmt;

pub const NEWLINE: &str = "\r\n";

/// Safety: Assumes the IO lock is already held.
struct Uart;

impl Uart {
    const UART_ADDRESS: *mut c_uchar = 0x10000000 as *mut c_uchar;

    unsafe fn new() -> Self {
        unsafe {
            Self::UART_ADDRESS.add(3).write_volatile(0b11);
            Self::UART_ADDRESS.add(2).write_volatile(1);
            Self::UART_ADDRESS.add(1).write_volatile(1);
        }

        Self
    }

    fn write_c_uchar(&self, c: c_uchar) {
        const UART_LSR_EMPTY_MASK: c_uchar = 0x40;
        unsafe {
            while Self::UART_ADDRESS.add(5).read_volatile() & UART_LSR_EMPTY_MASK == 0 {}
            Self::UART_ADDRESS.write_volatile(c);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.write_c_uchar(c);
        }

        Ok(())
    }
}

static IO_LOCK: SpinLock = SpinLock::new();

pub fn write(args: fmt::Arguments, newline: bool) {
    use fmt::Write;

    IO_LOCK.lock();

    let mut uart = unsafe { Uart::new() };
    uart.write_fmt(args).unwrap();
    if newline {
        uart.write_str(NEWLINE).unwrap();
    }

    IO_LOCK.release();
}

#[macro_export]
macro_rules! print {
    ($($arg: tt)+) => {
        $crate::uart::write(format_args!($($arg)*), false);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::uart::write(format_args!(""), true);
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::uart::write(format_args!($fmt $(, $($arg)+)?), true);
    };
}
