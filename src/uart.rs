use core::ffi::{c_char, c_uchar, CStr, c_void};

const NEWLINE: &str = "\r\n";

const UART_ADDRESS: *mut c_uchar = 0x1000_0000 as *mut c_uchar;

pub fn uart_init() {
    unsafe {
        UART_ADDRESS.add(3).write_volatile(0b11);
        UART_ADDRESS.add(2).write_volatile(1);
        UART_ADDRESS.add(1).write_volatile(1);
    }
}

#[no_mangle]
pub extern "C" fn uart_put_c_uchar(c: c_uchar) {
    const UART_LSR_EMPTY_MASK: c_uchar = 0x40;
    unsafe {
        while UART_ADDRESS.add(5).read_volatile() & UART_LSR_EMPTY_MASK == 0 {}
        UART_ADDRESS.write_volatile(c);
    }
}

pub fn uart_put_str(s: &str) {
    for c in s.bytes() {
        uart_put_c_uchar(c);
    }
}

#[no_mangle]
pub extern "C" fn uart_put_nl() {
    uart_put_str(NEWLINE);
}

#[no_mangle]
pub extern "C" fn uart_put_c_string(s: *const c_uchar) {
    if s.is_null() {
        uart_put_str("<null_ptr>");
        return;
    }

    let s = unsafe { CStr::from_ptr(s as *const c_char) };
    for c in s.to_bytes() {
        uart_put_c_uchar(*c);
    }
}

#[no_mangle]
pub extern "C" fn uart_put_uint(i: usize) {
    if (i / 10) != 0 {
        uart_put_uint(i / 10);
    }
    uart_put_c_uchar((i % 10) as u8 + b'0');
}

#[no_mangle]
pub extern "C" fn uart_put_uint_hex(i: usize) {
    if (i / 16) != 0 {
        uart_put_uint_hex(i / 16);
    }
    let r = i % 16;
    uart_put_c_uchar(r as u8 + if r < 10 { b'0' } else { b'A' - 10 });
}

#[no_mangle]
pub extern "C" fn uart_put_sint(mut i: isize) {
    if i < 0 {
        uart_put_c_uchar(b'-');
        i = -i;
    }
    // Safety: Guaranteed to be non-negative
    uart_put_uint(i.try_into().unwrap());
}

pub fn uart_put_ptr<T>(ptr: *const T) {
    if ptr.is_null() {
        uart_put_str("<null_ptr>");
        return;
    }

    uart_put_str("0x");
    uart_put_uint_hex(ptr as usize);
}

#[no_mangle]
pub extern "C" fn uart_put_c_ptr(ptr: *const c_void) {
    uart_put_ptr(ptr);
}
