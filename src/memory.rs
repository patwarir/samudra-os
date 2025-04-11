use crate::concurrency::{OnceSpinLock, SpinLock};
use crate::{fdtb_variables, k_println};
use core::alloc::Layout;
use core::ffi::c_void;
use core::ptr::NonNull;
use talc::{ErrOnOom, Span, Talc, Talck};

unsafe extern "C" {
    #[link_name = "__memory_start"]
    unsafe static MEMORY_START: c_void;
}

pub static STACK_END: OnceSpinLock<usize> = OnceSpinLock::new();

static HEAP_START: OnceSpinLock<usize> = OnceSpinLock::new();
static HEAP_END: OnceSpinLock<usize> = OnceSpinLock::new();

static TALCK_ALLOCATOR: OnceSpinLock<Talck<SpinLock, ErrOnOom>> = OnceSpinLock::new();

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
    STACK_END.set(stack_end).expect("Failed to set STACK_END!");

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
    HEAP_START
        .set(heap_start)
        .expect("Failed to set HEAP_START!");

    let heap_end = memory_start
        + fdtb_variables::MEMORY_SIZE_BYTES
            .get()
            .expect("MEMORY_SIZE_BYTES not initialized!");
    HEAP_END.set(heap_end).expect("Failed to set HEAP_END!");

    assert!(
        heap_start < heap_end,
        "Heap start must be less than heap end!"
    );

    k_println!("Memory start: {:#x}", memory_start);

    k_println!("Stack end: {:#x}", stack_end);
    k_println!("TLS end: {:#x}", tls_end);

    k_println!("Heap start: {:#x}", heap_start);
    k_println!("Heap end: {:#x}", heap_end);

    let talck = Talc::new(ErrOnOom).lock::<SpinLock>();

    unsafe {
        talck
            .lock()
            .claim(Span::new(
                heap_start as *mut u8,
                (heap_end - (tls_end - memory_start)) as *mut u8,
            ))
            .expect("Failed to claim heap span!");
    }

    if let Ok(_) = TALCK_ALLOCATOR.set(talck) {
        k_println!("Allocator initialized!");
    } else {
        panic!("Failed to set TALCK_ALLOCATOR!");
    }
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
