//! Uniprocessor interior mutability primative

use core::cell::UnsafeCell;

/// `UpSafeCell` is safe to be shared in uniprocessor.
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

    /// # Safety
    /// User is responsible for the exclusive ownership of `self`.
    /// It is an *undefined behavior* if the object is borrowed multiple times.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn exclusive_access(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}
