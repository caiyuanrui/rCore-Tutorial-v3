//! Uniprocessor interior mutability primative

use core::cell::UnsafeCell;

pub struct UpSafeCell<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for UpSafeCell<T> {}

impl<T> UpSafeCell<T> {
    /// # Safety
    /// User has to gaurantee that inner struct is only used in uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn exclusive_access(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}
