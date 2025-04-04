use crate::concurrency::SpinLock;
use core::ffi::c_uchar;
use core::fmt;

pub const NEWLINE: &str = "\r\n";

/// SAFETY: Assumes the IO lock is already acquired
struct Uart;

impl Uart {
    const UART_ADDRESS: *mut c_uchar = 0x10000000 as *mut c_uchar;

    /// Initializes a new UART connection and clears previous buffers
    unsafe fn new() -> Self {
        unsafe {
            // Enable and clear FIFO buffers
            Self::UART_ADDRESS.add(2).write_volatile(0b111);

            // Set character length of 8 bits in LCR
            Self::UART_ADDRESS.add(3).write_volatile(0b11);

            // Enable interrupts
            Self::UART_ADDRESS.add(1).write_volatile(1);
        }

        Self
    }

    /// Writes a single character
    fn write_c_uchar(&mut self, c: c_uchar) {
        const LSR_EMPTY_MASK: c_uchar = 0x40;
        unsafe {
            while Self::UART_ADDRESS.add(5).read_volatile() & LSR_EMPTY_MASK == 0 {}
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

pub unsafe fn io_lock_try_acquire() -> bool {
    IO_LOCK.try_acquire()
}

pub unsafe fn io_lock_acquire() {
    IO_LOCK.acquire();
}

pub unsafe fn io_lock_release() {
    IO_LOCK.release();
}

/// SAFETY: Assumes the IO lock is already acquired
pub unsafe fn write_unsafe<const ADD_NEWLINE: bool>(args: fmt::Arguments) {
    use fmt::Write;

    let mut uart = unsafe { Uart::new() };

    uart.write_fmt(args).expect("Cannot write to output!");

    if ADD_NEWLINE {
        uart.write_str(NEWLINE)
            .expect("Cannot write new line to output!");
    }
}

pub fn write<const ADD_NEWLINE: bool>(args: fmt::Arguments) {
    unsafe {
        io_lock_acquire();
        write_unsafe::<ADD_NEWLINE>(args);
        io_lock_release();
    }
}

#[macro_export]
macro_rules! k_println {
    () => {
        $crate::uart::write::<true>(format_args!(""));
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::uart::write::<true>(format_args!($fmt $(, $($arg)+)?));
    };
}
