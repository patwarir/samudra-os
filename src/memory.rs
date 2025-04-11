use crate::concurrency::OnceSpinLock;
use crate::fdtb_variables;
use core::ffi::c_void;

unsafe extern "C" {
    #[link_name = "__memory_start"]
    unsafe static MEMORY_START: c_void;
}

pub static STACK_END: OnceSpinLock<usize> = OnceSpinLock::new();

static HEAP_START: OnceSpinLock<usize> = OnceSpinLock::new();
static HEAP_END: OnceSpinLock<usize> = OnceSpinLock::new();

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
}
