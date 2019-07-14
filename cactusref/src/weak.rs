use core::marker::PhantomData;
use core::mem;
use core::ptr::{self, NonNull};
use std::alloc::{Alloc, Global, Layout};
use std::fmt;

use crate::ptr::{data_offset, data_offset_sized, is_dangling, set_data_ptr, RcBox, RcBoxPtr};
use crate::Rc;

/// `Weak` is a version of [`Rc`] that holds a non-owning reference to the
/// managed value. The value is accessed by calling [`upgrade`](Weak::upgrade)
/// on the `Weak` pointer, which returns an [`Option`]`<`[`Rc`]`<T>>`.
///
/// Since a `Weak` reference does not count towards ownership, it will not
/// prevent the inner value from being dropped, and `Weak` itself makes no
/// guarantees about the value still being present and may return [`None`]
/// when [`upgrade`](Weak::upgrade)d.
///
/// A `Weak` pointer is useful for keeping a temporary reference to the value
/// within [`Rc`] without extending its lifetime. It is also used to prevent
/// circular references between [`Rc`] pointers, since mutual owning references
/// would never allow either [`Rc`] to be dropped. For example, a tree could
/// have strong [`Rc`] pointers from parent nodes to children, and `Weak`
/// pointers from children back to their parents.
///
/// The typical way to obtain a `Weak` pointer is to call [`Rc::downgrade`].
pub struct Weak<T: ?Sized> {
    // This is a `NonNull` to allow optimizing the size of this type in enums,
    // but it is not necessarily a valid pointer.
    // `Weak::new` sets this to `usize::MAX` so that it doesn’t need
    // to allocate space on the heap.  That's not a value a real pointer
    // will ever have because RcBox has alignment at least 2.
    pub(crate) ptr: NonNull<RcBox<T>>,
}

impl<T: ?Sized> !Send for Weak<T> {}
impl<T: ?Sized> !Sync for Weak<T> {}

impl<T> Weak<T> {
    /// Constructs a new `Weak<T>`, without allocating any memory.
    ///
    /// Calling [`upgrade`](Weak::upgrade) on the return value always gives
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Weak;
    ///
    /// let empty: Weak<i64> = Weak::new();
    /// assert!(empty.upgrade().is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            ptr: NonNull::new(usize::max_value() as *mut RcBox<T>).expect("MAX is not 0"),
        }
    }

    /// Returns a raw pointer to the object `T` pointed to by this `Weak<T>`.
    ///
    /// It is up to the caller to ensure that the object is still alive when
    /// accessing it through the pointer.
    ///
    /// The pointer may be [`null`](std::ptr::null) or be dangling in case the
    /// object has already been destroyed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    /// use std::ptr;
    ///
    /// let strong = Rc::new("hello".to_owned());
    /// let weak = Rc::downgrade(&strong);
    /// // Both point to the same object
    /// assert!(ptr::eq(&*strong, weak.as_raw()));
    /// // The strong here keeps it alive, so we can still access the object.
    /// assert_eq!("hello", unsafe { &*weak.as_raw() });
    ///
    /// drop(strong);
    /// // But not any more. We can do weak.as_raw(), but accessing the pointer
    /// // would lead to undefined behaviour.
    /// // assert_eq!("hello", unsafe { &*weak.as_raw() });
    /// ```
    pub fn as_raw(&self) -> *const T {
        match self.inner() {
            None => ptr::null(),
            Some(inner) => {
                let offset = data_offset_sized::<T>();
                let ptr = inner as *const RcBox<T>;
                // Note: while the pointer we create may already point to
                // dropped value, the allocation still lives (it must hold the
                // weak point as long as we are alive).  Therefore, the offset
                // is OK to do, it won't get out of the allocation.
                let ptr = unsafe { (ptr as *const u8).offset(offset) };
                ptr as *const T
            }
        }
    }

    /// Consumes the `Weak<T>` and turns it into a raw pointer.
    ///
    /// This converts the weak pointer into a raw pointer, preserving the
    /// original weak count. It can be turned back into the `Weak<T>` with
    /// [`from_raw`](Weak::from_raw).
    ///
    /// The same restrictions of accessing the target of the pointer as with
    /// [`as_raw`](Weak::as_raw) apply.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Weak};
    ///
    /// let strong = Rc::new("hello".to_owned());
    /// let weak = Rc::downgrade(&strong);
    /// let raw = weak.into_raw();
    ///
    /// assert_eq!(1, Rc::weak_count(&strong));
    /// assert_eq!("hello", unsafe { &*raw });
    ///
    /// drop(unsafe { Weak::from_raw(raw) });
    /// assert_eq!(0, Rc::weak_count(&strong));
    /// ```
    pub fn into_raw(self) -> *const T {
        let result = self.as_raw();
        mem::forget(self);
        result
    }

    /// Converts a raw pointer previously created by
    /// [`into_raw`](Weak::into_raw) back into `Weak<T>`.
    ///
    /// This can be used to safely get a strong reference (by calling
    /// [`upgrade`](Weak::upgrade) later) or to deallocate the weak count by
    /// dropping the `Weak<T>`.
    ///
    /// It takes ownership of one weak count. In case a [`null`](std::ptr::null)
    /// is passed, a dangling [`Weak`] is returned.
    ///
    /// # Safety
    ///
    /// The pointer must represent one valid weak count. In other words, it must
    /// point to `T` which is or *was* managed by an [`Rc`] and the weak count
    /// of that [`Rc`] must not have reached 0. It is allowed for the strong
    /// count to be 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Weak};
    ///
    /// let strong = Rc::new("hello".to_owned());
    ///
    /// let raw_1 = Rc::downgrade(&strong).into_raw();
    /// let raw_2 = Rc::downgrade(&strong).into_raw();
    ///
    /// assert_eq!(2, Rc::weak_count(&strong));
    ///
    /// assert_eq!("hello", &*unsafe { Weak::from_raw(raw_1) }.upgrade().unwrap());
    /// assert_eq!(1, Rc::weak_count(&strong));
    ///
    /// drop(strong);
    ///
    /// // Decrement the last weak count.
    /// assert!(unsafe { Weak::from_raw(raw_2) }.upgrade().is_none());
    /// ```
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        if ptr.is_null() {
            Self::new()
        } else {
            // See Rc::from_raw for details
            let offset = data_offset(ptr);
            let fake_ptr = ptr as *mut RcBox<T>;
            let ptr = set_data_ptr(fake_ptr, (ptr as *mut u8).offset(-offset));
            Self {
                ptr: NonNull::new(ptr).expect("Invalid pointer passed to from_raw"),
            }
        }
    }
}

impl<T: ?Sized> Weak<T> {
    /// Attempts to upgrade the `Weak` pointer to an [`Rc`], extending
    /// the lifetime of the value if successful.
    ///
    /// Returns [`None`] if the value has since been dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// let weak_five = Rc::downgrade(&five);
    ///
    /// let strong_five: Option<Rc<_>> = weak_five.upgrade();
    /// assert!(strong_five.is_some());
    ///
    /// // Destroy all strong pointers.
    /// drop(strong_five);
    /// drop(five);
    ///
    /// assert!(weak_five.upgrade().is_none());
    /// ```
    pub fn upgrade(&self) -> Option<Rc<T>> {
        let inner = self.inner()?;
        if inner.strong() <= inner.link() {
            None
        } else {
            inner.inc_strong();
            Some(Rc {
                ptr: self.ptr,
                phantom: PhantomData,
            })
        }
    }

    /// Gets the number of strong (`Rc`) pointers pointing to this value.
    ///
    /// If `self` was created using [`Weak::new`], this will return 0.
    pub fn strong_count(&self) -> usize {
        if let Some(inner) = self.inner() {
            inner.strong() - inner.link()
        } else {
            0
        }
    }

    /// Gets the number of `Weak` pointers pointing to this value.
    ///
    /// If `self` was created using [`Weak::new`], this will return `None`. If
    /// not, the returned value is at least 1, since `self` still points to the
    /// value.
    pub fn weak_count(&self) -> Option<usize> {
        self.inner().map(|inner| {
            if inner.strong() - inner.link() > 0 {
                inner.weak() - 1 // subtract the implicit weak ptr
            } else {
                inner.weak()
            }
        })
    }

    /// Returns `None` when the pointer is dangling and there is no allocated
    /// `RcBox` (i.e., when this `Weak` was created by [`Weak::new`]).
    #[inline]
    fn inner(&self) -> Option<&RcBox<T>> {
        if is_dangling(self.ptr) {
            None
        } else {
            Some(unsafe { self.ptr.as_ref() })
        }
    }

    /// Returns `true` if the two `Weak`s point to the same value (not just
    /// values that compare as equal).
    ///
    /// # Notes
    ///
    /// Since this compares pointers it means that `Weak::new()` will equal each
    /// other, even though they don't point to any value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let first_rc = Rc::new(5);
    /// let first = Rc::downgrade(&first_rc);
    /// let second = Rc::downgrade(&first_rc);
    ///
    /// assert!(first.ptr_eq(&second));
    ///
    /// let third_rc = Rc::new(5);
    /// let third = Rc::downgrade(&third_rc);
    ///
    /// assert!(!first.ptr_eq(&third));
    /// ```
    ///
    /// Comparing `Weak::new`.
    ///
    /// ```
    /// use cactusref::{Rc, Weak};
    ///
    /// let first = Weak::new();
    /// let second = Weak::new();
    /// assert!(first.ptr_eq(&second));
    ///
    /// let third_rc = Rc::new(());
    /// let third = Rc::downgrade(&third_rc);
    /// assert!(!first.ptr_eq(&third));
    /// ```
    #[inline]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl<T: ?Sized> Drop for Weak<T> {
    /// Drops the `Weak` pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Weak};
    ///
    /// struct Foo;
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped!");
    ///     }
    /// }
    ///
    /// let foo = Rc::new(Foo);
    /// let weak_foo = Rc::downgrade(&foo);
    /// let other_weak_foo = Weak::clone(&weak_foo);
    ///
    /// drop(weak_foo);   // Doesn't print anything
    /// drop(foo);        // Prints "dropped!"
    ///
    /// assert!(other_weak_foo.upgrade().is_none());
    /// ```
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

impl<T: ?Sized> Clone for Weak<T> {
    /// Makes a clone of the `Weak` pointer that points to the same value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Weak};
    ///
    /// let weak_five = Rc::downgrade(&Rc::new(5));
    ///
    /// let _ = Weak::clone(&weak_five);
    /// ```
    #[inline]
    fn clone(&self) -> Self {
        if let Some(inner) = self.inner() {
            inner.inc_weak()
        }
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Weak)")
    }
}

impl<T> Default for Weak<T> {
    /// Constructs a new `Weak<T>`, allocating memory for `T` without
    /// initializing it. Calling [`upgrade`](Weak::upgrade) on the return value
    /// always gives [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Weak;
    ///
    /// let empty: Weak<i64> = Default::default();
    /// assert!(empty.upgrade().is_none());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
