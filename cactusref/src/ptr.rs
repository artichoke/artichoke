use core::ptr::NonNull;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::intrinsics::abort;

use crate::link::Link;
use crate::{Rc, Reachable};

pub trait RcBoxPtr<T: ?Sized + Reachable> {
    fn inner(&self) -> &RcBox<T>;

    #[inline]
    fn strong(&self) -> usize {
        self.inner().strong.get()
    }

    #[inline]
    fn inc_strong(&self) {
        // We want to abort on overflow instead of dropping the value.
        // nevertheless, we insert an abort here to hint LLVM at
        // an otherwise missed optimization.
        if self.strong() == 0 || self.strong() == usize::max_value() {
            unsafe {
                abort();
            }
        }
        self.inner().strong.set(self.strong() + 1);
    }

    #[inline]
    fn dec_strong(&self) {
        self.inner().strong.set(self.strong() - 1);
    }

    #[inline]
    fn weak(&self) -> usize {
        self.inner().weak.get()
    }

    #[inline]
    fn inc_weak(&self) {
        // We want to abort on overflow instead of dropping the value.
        // The reference count will never be zero when this is called;
        // nevertheless, we insert an abort here to hint LLVM at
        // an otherwise missed optimization.
        if self.weak() == 0 || self.weak() == usize::max_value() {
            unsafe {
                abort();
            }
        }
        self.inner().weak.set(self.weak() + 1);
    }

    #[inline]
    fn dec_weak(&self) {
        self.inner().weak.set(self.weak() - 1);
    }
}

impl<T: ?Sized + Reachable> RcBoxPtr<T> for Rc<T> {
    fn inner(&self) -> &RcBox<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized + Reachable> RcBoxPtr<T> for RcBox<T> {
    fn inner(&self) -> &Self {
        self
    }
}

pub struct RcBox<T: ?Sized + Reachable> {
    pub(crate) strong: Cell<usize>,
    pub(crate) weak: Cell<usize>,
    pub(crate) links: RefCell<HashSet<Link<T>>>,
    pub(crate) value: Box<T>,
}

pub(crate) fn is_dangling<T: ?Sized>(ptr: NonNull<T>) -> bool {
    let address = ptr.as_ptr() as *mut () as usize;
    address == usize::max_value()
}
