use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use lock_api::{GuardNoSend, Mutex, RawMutex};

// TODO: Keep track of the thread that acquired the lock

#[derive(Debug)]
#[repr(transparent)]
pub struct RawSpinLock(AtomicBool);

unsafe impl RawMutex for RawSpinLock {
    const INIT: Self = Self(AtomicBool::new(false));

    type GuardMarker = GuardNoSend;

    fn try_lock(&self) -> bool {
        !self.0.swap(true, Ordering::Acquire)
    }

    fn lock(&self) {
        while !self.try_lock() {
            core::hint::spin_loop();
        }
    }

    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }
}

pub type MutexSpinLock<T> = Mutex<RawSpinLock, T>;

#[derive(Debug)]
pub struct OnceSpinLock<T> {
    lock: RawSpinLock,
    initialized: AtomicBool,
    value: UnsafeCell<Option<T>>,
}

impl<T> OnceSpinLock<T> {
    pub const fn new() -> Self {
        Self {
            lock: RawSpinLock::INIT,
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
        self.lock.lock();
        if self.initialized.load(Ordering::Acquire) {
            unsafe {
                self.lock.unlock();
            }
            Err(value)
        } else {
            unsafe {
                *self.value.get() = Some(value);
            }
            self.initialized.store(true, Ordering::Release);
            unsafe {
                self.lock.unlock();
            }
            Ok(())
        }
    }
}

/// SAFETY: Manages its own synchronization and is safe to share across threads
unsafe impl<T> Send for OnceSpinLock<T> {}
unsafe impl<T> Sync for OnceSpinLock<T> {}
