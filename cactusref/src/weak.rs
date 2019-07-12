use core::marker::PhantomData;
use core::ptr::NonNull;
use std::alloc::{Alloc, Global, Layout};
use std::fmt;

use crate::ptr::{RcBox, RcBoxPtr};
use crate::{is_dangling, CactusRef, Reachable};

pub struct Weak<T: ?Sized + Reachable> {
    // This is a `NonNull` to allow optimizing the size of this type in enums,
    // but it is not necessarily a valid pointer.
    // `Weak::new` sets this to `usize::MAX` so that it doesn’t need
    // to allocate space on the heap.  That's not a value a real pointer
    // will ever have because RcBox has alignment at least 2.
    pub(crate) ptr: NonNull<RcBox<T>>,
}

impl<T: ?Sized + Reachable> Weak<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::new(usize::max_value() as *mut RcBox<T>).expect("MAX is not 0"),
        }
    }

    pub fn upgrade(&self) -> Option<CactusRef<T>> {
        let inner = self.inner()?;
        if inner.strong() == 0 {
            None
        } else {
            inner.inc_strong();
            Some(CactusRef {
                ptr: self.ptr,
                phantom: PhantomData,
            })
        }
    }

    pub fn strong_count(&self) -> usize {
        if let Some(inner) = self.inner() {
            inner.strong()
        } else {
            0
        }
    }

    pub fn weak_count(&self) -> Option<usize> {
        self.inner().map(|inner| {
            if inner.strong() > 0 {
                inner.weak() - 1 // subtract the implicit weak ptr
            } else {
                inner.weak()
            }
        })
    }

    #[inline]
    fn inner(&self) -> Option<&RcBox<T>> {
        if is_dangling(self.ptr) {
            None
        } else {
            Some(unsafe { self.ptr.as_ref() })
        }
    }

    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl<T: ?Sized + Reachable> Drop for Weak<T> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner() {
            inner.dec_weak();
            // the weak count starts at 1, and will only go to zero if all
            // the strong pointers have disappeared.
            if inner.weak() == 0 {
                unsafe {
                    Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                }
            }
        }
    }
}

impl<T: ?Sized + Reachable> Clone for Weak<T> {
    #[inline]
    fn clone(&self) -> Self {
        if let Some(inner) = self.inner() {
            inner.inc_weak()
        }
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized + Reachable + fmt::Debug> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Weak)")
    }
}

impl<T: ?Sized + Reachable> Default for Weak<T> {
    fn default() -> Self {
        Self::new()
    }
}
