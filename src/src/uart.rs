use core::ffi::{c_uchar, CStr};

#[no_mangle]
pub static NEWLINE: &[c_uchar] = b"\r\n\0";

pub const UART_ADDRESS: usize = 0x1000_0000;

pub fn uart_init() {
    let ptr = UART_ADDRESS as *mut c_uchar;
    unsafe {
        ptr.add(3).write_volatile((1 << 1) | (1 << 0));
        ptr.add(2).write_volatile(1 << 0);
        ptr.add(1).write_volatile(1 << 0);
    }
}

#[no_mangle]
pub fn uart_put_char(c: c_uchar) {
    let ptr = UART_ADDRESS as *mut c_uchar;
    unsafe {
        ptr.write_volatile(c);
    }
}

pub fn uart_put_str(s: &str) {
    for c in s.bytes() {
        uart_put_char(c);
    }
}

#[no_mangle]
pub fn uart_put_c_string(s: *const c_uchar) {
    let s = unsafe { CStr::from_ptr(s as *const i8) };
    for c in s.to_bytes() {
        uart_put_char(*c);
    }
}

#[no_mangle]
pub fn uart_put_uint(i: usize) {
    if (i / 10) != 0 {
        uart_put_uint(i / 10);
    }
    uart_put_char((i % 10) as u8 + b'0');
}

#[no_mangle]
pub fn uart_put_uint_hex(i: usize) {
    if (i / 16) != 0 {
        uart_put_uint_hex(i / 16);
    }
    let r = i % 16;
    uart_put_char(r as u8 + if r < 10 { b'0' } else { b'A' - 10 });
}

#[no_mangle]
pub fn uart_put_int(mut i: isize) {
    if i < 0 {
        uart_put_char(b'-');
        i = -i;
    }
    if (i / 10) != 0 {
        uart_put_int(i / 10);
    }
    uart_put_char((i % 10) as u8 + b'0');
}
