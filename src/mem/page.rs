use alloc::alloc::*;

use crate::{asm, uart};

const PAGE_ORDER: usize = 12;

pub const PAGE_SIZE: usize = 1 << PAGE_ORDER;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PageStateFlag {
    Free = 0,
    Allocated = 1,
}

pub struct Page {
    state: PageStateFlag,
}

const MAX_NUM_PAGES: usize = 0x10000;
pub static mut NUM_PAGES: usize = 0;

static mut ALLOC_START: usize = 0;

const fn align(value: usize, order: usize) -> usize {
    let o = (1usize << order) - 1;
    (value + o) & !o
}

unsafe fn get_page(i: usize) -> *mut Page {
    (asm::K_MEMORY_HEAP_START as *mut Page).add(i)
}

pub fn print_page_table() {
    unsafe {
        uart::uart_put_str("Pages:");
        uart::uart_put_nl();

        for i in 0..NUM_PAGES {
            uart::uart_put_str(if (*get_page(i)).state == PageStateFlag::Free { "-" } else { "X" });
        }
        uart::uart_put_nl();
    }
}

pub fn init() {
    unsafe {
        NUM_PAGES = usize::min(
            ((asm::K_MEMORY_HEAP_END as usize) - (asm::K_MEMORY_HEAP_START as usize)) / PAGE_SIZE,
            MAX_NUM_PAGES,
        );

        for i in 0..NUM_PAGES {
            (*get_page(i)).state = PageStateFlag::Free;
        }

        ALLOC_START = align(
            (asm::K_MEMORY_HEAP_START as usize) + NUM_PAGES * core::mem::size_of::<Page>(),
            PAGE_ORDER
        );
    }
}

pub fn print_mem_values() {
    unsafe {
        uart::uart_put_str("Kernel memory heap start: ");
        uart::uart_put_ptr(asm::K_MEMORY_HEAP_START);
        uart::uart_put_nl();

        uart::uart_put_str("Kernel memory heap end: ");
        uart::uart_put_ptr(asm::K_MEMORY_HEAP_END);
        uart::uart_put_nl();

        uart::uart_put_str("Num Pages: ");
        uart::uart_put_uint(NUM_PAGES);
        uart::uart_put_nl();
    }
}

pub fn alloc(num_pages: usize) -> *mut u8 {
    if num_pages == 0 {
        return core::ptr::null_mut();
    }

    unsafe {
        let mut i = NUM_PAGES;
        for page_i in 0..(NUM_PAGES - num_pages) {
            let mut all_free = true;
            for page_alloc_i in page_i..(page_i + num_pages) {
                let page = get_page(page_alloc_i);
                if (*page).state != PageStateFlag::Free {
                    all_free = false;
                }
            }
            if all_free {
                i = page_i;
                break;
            }
        }

        if i == NUM_PAGES {
            panic!("Out of memory!");
        }

        for page_i in i..(i + num_pages) {
            let page = get_page(page_i);
            (*page).state = PageStateFlag::Allocated;
        }

        let address = ALLOC_START + i * PAGE_SIZE;
        address as *mut u8
    }
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return core::ptr::null_mut();
        }

        let mut num_pages = layout.size() / PAGE_SIZE;
        if layout.size() % PAGE_SIZE != 0 {
            num_pages += 1;
        }

        alloc(num_pages)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // TODO
    }
}

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator;
