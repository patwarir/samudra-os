use core::sync::atomic::{AtomicBool, Ordering};

#[repr(transparent)]
pub struct SpinLock(AtomicBool);

impl SpinLock {
    pub const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    pub fn try_acquire(&self) -> bool {
        !self.0.swap(true, Ordering::Acquire)
    }

    pub fn acquire(&self) {
        while !self.try_acquire() {
            core::hint::spin_loop();
        }
    }

    pub fn release(&self) {
        self.0.store(false, Ordering::Release);
    }
}
