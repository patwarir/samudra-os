use crate::{fdtb_variables, k_println};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::NonNull;
use spin::{Mutex, Once};
use talc::{ErrOnOom, Span, Talc, Talck};

unsafe extern "C" {
    #[link_name = "__memory_start"]
    unsafe static MEMORY_START: c_void;
}

static TALCK_ALLOCATOR: Once<Talck<Mutex<()>, ErrOnOom>> = Once::new();
static TLS_TALCK_ALLOCATORS: Once<Arc<[Talck<Mutex<()>, ErrOnOom>]>> = Once::new();

fn align_to_next_multiple<const N: usize>(addr: usize) -> usize {
    let mask = N - 1;
    (addr + mask) & !mask
}

pub unsafe fn init() {
    let memory_start = unsafe { (&MEMORY_START as *const c_void) as usize };
    let num_harts = fdtb_variables::NUM_HARTS
        .get()
        .expect("NUM_HARTS not initialized!");

    let stack_end = memory_start + (num_harts * crate::K_STACK_SIZE_PER_HART_BYTES);
    let tls_end = stack_end + (num_harts * crate::K_TLS_SIZE_PER_HART_BYTES);

    const K_STACK_MEMORY_SLACK_BYTES: usize = 0x10;
    const K_HEAP_START_ALIGNMENT_BYTES: usize = 0x1000;

    let heap_start = align_to_next_multiple::<K_HEAP_START_ALIGNMENT_BYTES>(
        tls_end + K_STACK_MEMORY_SLACK_BYTES,
    );

    assert!(
        heap_start % K_HEAP_START_ALIGNMENT_BYTES == 0,
        "Heap start must be aligned to 4KiB!"
    );

    let heap_end = memory_start
        + fdtb_variables::MEMORY_SIZE_BYTES
            .get()
            .expect("MEMORY_SIZE_BYTES not initialized!");

    assert!(
        heap_start < heap_end,
        "Heap start must be less than heap end!"
    );

    k_println!("Memory start: {:#x}", memory_start);

    k_println!("Stack end: {:#x}", stack_end);
    k_println!("TLS end: {:#x}", tls_end);

    k_println!("Heap start: {:#x}", heap_start);
    k_println!("Heap end: {:#x}", heap_end);

    let talck = Talc::new(ErrOnOom).lock::<Mutex<()>>();
    unsafe {
        talck
            .lock()
            .claim(Span::new(
                heap_start as *mut u8,
                (heap_end - (tls_end - memory_start)) as *mut u8,
            ))
            .expect("Failed to claim heap span!");
    }
    TALCK_ALLOCATOR.call_once(|| talck);

    let mut tls_talcks = Vec::with_capacity(*num_harts);
    for i in 0..*num_harts {
        let tls_start = stack_end + (i * crate::K_TLS_SIZE_PER_HART_BYTES);
        let tls_end = tls_start + crate::K_TLS_SIZE_PER_HART_BYTES;

        let talck = Talc::new(ErrOnOom).lock::<Mutex<()>>();

        unsafe {
            talck
                .lock()
                .claim(Span::new(tls_start as *mut u8, tls_end as *mut u8))
                .expect("Failed to claim TLS span!");
        }

        tls_talcks.push(talck);
    }
    TLS_TALCK_ALLOCATORS.call_once(|| Arc::from(tls_talcks));
}

#[derive(Debug)]
struct Allocator;

impl Allocator {
    pub const fn new() -> Self {
        Self
    }
}

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            TALCK_ALLOCATOR
                .get()
                .expect("TALCK_ALLOCATOR not initialized!")
                .lock()
                .malloc(layout)
                .expect("Failed to allocate memory!")
                .as_ptr()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            TALCK_ALLOCATOR
                .get()
                .expect("TALCK_ALLOCATOR not initialized!")
                .lock()
                .free(NonNull::new(ptr).expect("Invalid pointer!"), layout)
        }
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

pub fn alloc_tls(hart_id: usize, layout: Layout) -> Result<NonNull<u8>, ()> {
    let tls_talcks = TLS_TALCK_ALLOCATORS
        .get()
        .expect("TLS_TALCK_ALLOCATORS not initialized!");

    if hart_id >= tls_talcks.len() {
        return Err(());
    }

    unsafe { tls_talcks[hart_id].lock().malloc(layout) }
}

pub fn dealloc_tls(hart_id: usize, ptr: NonNull<u8>, layout: Layout) {
    let tls_talcks = TLS_TALCK_ALLOCATORS
        .get()
        .expect("TLS_TALCK_ALLOCATORS not initialized!");

    if hart_id >= tls_talcks.len() {
        return;
    }

    unsafe {
        tls_talcks[hart_id].lock().free(ptr, layout);
    }
}
