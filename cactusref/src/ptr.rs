use core::mem;
use core::ptr::{self, NonNull, Unique};
use std::alloc::{dealloc, Layout};
use std::cell::{Cell, RefCell};
use std::intrinsics::abort;

use crate::link::Links;
use crate::Rc;

#[allow(clippy::module_name_repetitions)]
pub trait RcBoxPtr<T: ?Sized> {
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

    #[inline]
    fn kill(&self) {
        self.inner().strong.set(0);
    }

    #[inline]
    fn is_dead(&self) -> bool {
        self.strong() == 0
    }
}

impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
    fn inner(&self) -> &RcBox<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> RcBoxPtr<T> for RcBox<T> {
    fn inner(&self) -> &Self {
        self
    }
}

pub struct RcBox<T: ?Sized> {
    pub strong: Cell<usize>,
    pub weak: Cell<usize>,
    pub links: RefCell<Links<T>>,
    pub back_links: RefCell<Links<T>>,
    pub value: T,
}

pub fn is_dangling<T: ?Sized>(ptr: NonNull<T>) -> bool {
    let address = ptr.as_ptr() as *mut () as usize;
    address == usize::max_value()
}

// duplicated from a crate-private function in std
// <https://github.com/rust-lang/rust/blob/baab1914/src/liballoc/alloc.rs#L212-L223>
#[inline]
pub unsafe fn box_free<T: ?Sized>(ptr: Unique<T>) {
    let ptr = ptr.as_ptr();
    let size = mem::size_of_val(&*ptr);
    let align = mem::align_of_val(&*ptr);
    // We do not allocate for Box<T> when T is ZST, so deallocation is also not necessary.
    if size != 0 {
        let layout = Layout::from_size_align_unchecked(size, align);
        dealloc(ptr as *mut u8, layout);
    }
}

// Sets the data pointer of a `?Sized` raw pointer.
//
// For a slice/trait object, this sets the `data` field and leaves the rest
// unchanged. For a sized raw pointer, this simply sets the pointer.
#[allow(clippy::module_name_repetitions)]
pub unsafe fn set_data_ptr<T: ?Sized, U>(mut ptr: *mut T, data: *mut U) -> *mut T {
    ptr::write(&mut ptr as *mut _ as *mut *mut u8, data as *mut u8);
    ptr
}

#[allow(clippy::cast_possible_wrap)]
pub unsafe fn data_offset<T: ?Sized>(ptr: *const T) -> isize {
    // Align the unsized value to the end of the RcBox.
    // Because it is ?Sized, it will always be the last field in memory.
    let align = mem::align_of_val(&*ptr);
    let layout = Layout::new::<RcBox<()>>();
    (layout.size() + layout.padding_needed_for(align)) as isize
}

/// Computes the offset of the data field within `RcBox`.
///
/// Unlike [`data_offset`], this doesn't need the pointer, but it works only on `T: Sized`.
#[allow(clippy::cast_possible_wrap)]
pub fn data_offset_sized<T>() -> isize {
    let align = mem::align_of::<T>();
    let layout = Layout::new::<RcBox<()>>();
    (layout.size() + layout.padding_needed_for(align)) as isize
}
