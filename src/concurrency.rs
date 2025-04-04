use core::cell::UnsafeCell;
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

pub struct OnceLock<T> {
    lock: SpinLock,
    initialized: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self {
            lock: SpinLock::new(),
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(unsafe { core::mem::zeroed() }),
        }
    }

    pub fn get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            Some(unsafe { &*self.value.get() })
        } else {
            None
        }
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        self.lock.acquire();
        if self.initialized.load(Ordering::Acquire) {
            self.lock.release();
            Err(value)
        } else {
            unsafe {
                core::ptr::write(self.value.get(), value);
            }
            self.initialized.store(true, Ordering::Release);
            self.lock.release();
            Ok(())
        }
    }
}

/// SAFETY: Manages its own synchronization and is safe to share across threads
unsafe impl<T> Send for OnceLock<T> {}
unsafe impl<T> Sync for OnceLock<T> {}
