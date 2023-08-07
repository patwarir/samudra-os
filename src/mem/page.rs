use alloc::alloc::*;

use crate::{asm, uart};

pub fn print_mem_values() {
    unsafe {
        uart::uart_put_str("Kernel memory heap start: ");
        uart::uart_put_ptr(asm::K_MEMORY_HEAP_START);
        uart::uart_put_nl();

        uart::uart_put_str("Kernel memory heap end: ");
        uart::uart_put_ptr(asm::K_MEMORY_HEAP_END);
        uart::uart_put_nl();
    }
}

// TODO: Temporary

pub struct Alloc;

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

#[global_allocator]
pub static ALLOC: Alloc = Alloc;
