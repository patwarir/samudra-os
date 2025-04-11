use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
#[repr(transparent)]
pub struct SpinLock(AtomicBool);

impl SpinLock {
    pub const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    pub fn try_acquire(&self) -> bool {
        // TODO: Keep track of the thread that acquired the lock
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

unsafe impl lock_api::RawMutex for SpinLock {
    const INIT: Self = Self::new();

    type GuardMarker = lock_api::GuardNoSend;

    fn try_lock(&self) -> bool {
        self.try_acquire()
    }

    fn lock(&self) {
        self.acquire();
    }

    unsafe fn unlock(&self) {
        self.release();
    }
}

#[derive(Debug)]
pub struct OnceSpinLock<T> {
    lock: SpinLock,
    initialized: AtomicBool,
    value: UnsafeCell<Option<T>>,
}

impl<T> OnceSpinLock<T> {
    pub const fn new() -> Self {
        Self {
            lock: SpinLock::new(),
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            unsafe { (*self.value.get()).as_ref() }
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
                *self.value.get() = Some(value);
            }
            self.initialized.store(true, Ordering::Release);
            self.lock.release();
            Ok(())
        }
    }
}

/// SAFETY: Manages its own synchronization and is safe to share across threads
unsafe impl<T> Send for OnceSpinLock<T> {}
unsafe impl<T> Sync for OnceSpinLock<T> {}
