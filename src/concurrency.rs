use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    is_locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.is_locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
    }

    pub fn release(&self) {
        self.is_locked.store(false, Ordering::Release);
    }
}
