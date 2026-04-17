use std::{
    fmt::{self, Debug},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

pub struct DropGuard<T, F>
where
    F: FnOnce(T),
{
    inner: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

impl<T, F> DropGuard<T, F>
where
    F: FnOnce(T),
{
    pub const fn new(inner: T, f: F) -> Self {
        Self {
            inner: ManuallyDrop::new(inner),
            f: ManuallyDrop::new(f),
        }
    }
}

impl<T, F> Deref for DropGuard<T, F>
where
    F: FnOnce(T),
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T, F> DerefMut for DropGuard<T, F>
where
    F: FnOnce(T),
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T, F> Drop for DropGuard<T, F>
where
    F: FnOnce(T),
{
    fn drop(&mut self) {
        // SAFETY: `DropGuard` is in the process of being dropped.
        let inner = unsafe { ManuallyDrop::take(&mut self.inner) };

        // SAFETY: `DropGuard` is in the process of being dropped.
        let f = unsafe { ManuallyDrop::take(&mut self.f) };

        f(inner);
    }
}

impl<T, F> Debug for DropGuard<T, F>
where
    T: Debug,
    F: FnOnce(T),
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
