use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;
use core::pin::Pin;
use core::ptr::{self, NonNull};
use std::alloc::{handle_alloc_error, Alloc, Global, Layout};
use std::borrow;
use std::cell::{Cell, RefCell};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::link::Links;
use crate::ptr::{box_free, data_offset, is_dangling, set_data_ptr, RcBox, RcBoxPtr};
use crate::Weak;

/// A single-threaded reference-counting pointer. 'Rc' stands for 'Reference
/// Counted'.
///
/// This `Rc` differs from the [`Rc`](std::rc) in `std` by adding support for
/// detecting and deallocating orphaned cycles of references. An orphaned cycle
/// is one in which all objects are only owned by other members of the cycle.
///
/// See the [module-level documentation](crate) for more details.
///
/// The inherent methods of `Rc` are all associated functions, which means
/// that you have to call them as e.g., [`Rc::get_mut(&mut value)`](Rc::get_mut)
/// instead of `value.get_mut()`. This avoids conflicts with methods of the
/// inner type `T`.
pub struct Rc<T: ?Sized> {
    pub(crate) ptr: NonNull<RcBox<T>>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: ?Sized> !Send for Rc<T> {}
impl<T: ?Sized> !Sync for Rc<T> {}

impl<T> Rc<T> {
    /// Constructs a new `Rc<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            // there is an implicit weak pointer owned by all the strong
            // pointers, which ensures that the weak destructor never frees
            // the allocation while the strong destructor is running, even
            // if the weak pointer is stored inside the strong one.
            ptr: Box::into_raw_non_null(Box::new(RcBox {
                strong: Cell::new(1),
                weak: Cell::new(1),
                links: RefCell::new(Links::default()),
                value,
            })),
            phantom: PhantomData,
        }
    }

    /// Constructs a new `Pin<Rc<T>>`. If `T` does not implement `Unpin`, then
    /// `value` will be pinned in memory and unable to be moved.
    pub fn pin(value: T) -> Pin<Self> {
        unsafe { Pin::new_unchecked(Self::new(value)) }
    }

    /// Returns the contained value, if the `Rc` has exactly one strong reference.
    ///
    /// Otherwise, an [`Err`] is returned with the same `Rc` that was
    /// passed in.
    ///
    /// This will succeed even if there are outstanding weak references.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let x = Rc::new(Object(3));
    /// assert_eq!(Rc::try_unwrap(x), Ok(Object(3)));
    ///
    /// let x = Rc::new(Object(4));
    /// let _y = Rc::clone(&x);
    /// assert_eq!(*Rc::try_unwrap(x).unwrap_err(), Object(4));
    /// ```
    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        if Self::strong_count(&this) == 1 {
            unsafe {
                let val = ptr::read(&*this); // copy the contained object

                // Indicate to Weaks that they can't be promoted by decrementing
                // the strong count, and then remove the implicit "strong weak"
                // pointer while also handling drop logic by just crafting a
                // fake Weak.
                this.dec_strong();
                let _weak = Weak { ptr: this.ptr };
                mem::forget(this);
                Ok(val)
            }
        } else {
            Err(this)
        }
    }
}

impl<T: ?Sized> Rc<T> {
    /// Consumes the `Rc`, returning the wrapped pointer.
    ///
    /// To avoid a memory leak the pointer must be converted back to an `Rc` using
    /// [`Rc::from_raw`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(String);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let x = Rc::new(Object("hello".to_owned()));
    /// let x_ptr = Rc::into_raw(x);
    /// assert_eq!(unsafe { &(*x_ptr).0 }, "hello");
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw(this: Self) -> *const T {
        let ptr: *const T = &*this;
        mem::forget(this);
        ptr
    }

    /// Constructs an `Rc` from a raw pointer.
    ///
    /// The raw pointer must have been previously returned by a call to a
    /// [`Rc::into_raw`].
    ///
    /// This function is unsafe because improper use may lead to memory
    /// problems. For example, a double-free may occur if the function is called
    /// twice on the same raw pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(String);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let x = Rc::new(Object("hello".to_owned()));
    /// let x_ptr = Rc::into_raw(x);
    ///
    /// unsafe {
    ///     // Convert back to an `Rc` to prevent leak.
    ///     let x = Rc::from_raw(x_ptr);
    ///     assert_eq!(&*x.0, "hello");
    ///
    ///     // Further calls to `Rc::from_raw(x_ptr)` would be memory unsafe.
    /// }
    ///
    /// // The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
    /// ```
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        let offset = data_offset(ptr);

        // Reverse the offset to find the original RcBox.
        let fake_ptr = ptr as *mut RcBox<T>;
        let rc_ptr = set_data_ptr(fake_ptr, (ptr as *mut u8).offset(-offset));

        Self {
            ptr: NonNull::new_unchecked(rc_ptr),
            phantom: PhantomData,
        }
    }

    /// Consumes the `Rc`, returning the wrapped pointer as `NonNull<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(rc_into_raw_non_null)]
    ///
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(String);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let x = Rc::new(Object("hello".to_owned()));
    /// let ptr = Rc::into_raw_non_null(x);
    /// let deref = unsafe { ptr.as_ref() };
    /// assert_eq!(&deref.0, "hello");
    /// ```
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw_non_null(this: Self) -> NonNull<T> {
        // safe because Rc guarantees its pointer is non-null
        unsafe { NonNull::new_unchecked(Self::into_raw(this) as *mut _) }
    }

    /// Creates a new [`Weak`] pointer to this value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// let weak_five = Rc::downgrade(&five);
    /// ```
    pub fn downgrade(this: &Self) -> Weak<T> {
        this.inc_weak();
        // Make sure we do not create a dangling Weak
        debug_assert!(!is_dangling(this.ptr));
        Weak { ptr: this.ptr }
    }

    /// Gets the number of [`Weak`] pointers to this value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    /// let _weak_five = Rc::downgrade(&five);
    ///
    /// assert_eq!(1, Rc::weak_count(&five));
    /// ```
    #[inline]
    pub fn weak_count(this: &Self) -> usize {
        this.weak() - 1
    }

    /// Gets the number of strong (`Rc`) pointers to this value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    /// let _also_five = Rc::clone(&five);
    ///
    /// assert_eq!(2, Rc::strong_count(&five));
    /// ```
    #[inline]
    pub fn strong_count(this: &Self) -> usize {
        this.strong()
    }

    /// Returns `true` if there are no other `Rc` or [`Weak`] pointers to this
    /// inner value.
    #[inline]
    pub(crate) fn is_unique(this: &Self) -> bool {
        Self::weak_count(this) == 0 && Self::strong_count(this) == 1
    }

    /// Returns a mutable reference to the inner value, if there are no other
    /// `Rc` or [`Weak`] pointers to the same value.
    ///
    /// Returns [`None`] otherwise, because it is not safe to mutate a shared
    /// value.
    ///
    /// See also [`make_mut`](Rc::make_mut), which will [`clone`](Clone) the
    /// inner value when it's shared.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let mut x = Rc::new(Object(3));
    /// Rc::get_mut(&mut x).unwrap().0 = 4;
    /// assert_eq!(x.0, 4);
    ///
    /// let _y = Rc::clone(&x);
    /// assert!(Rc::get_mut(&mut x).is_none());
    /// ```
    #[inline]
    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if Self::is_unique(this) {
            unsafe { Some(&mut this.ptr.as_mut().value) }
        } else {
            None
        }
    }

    /// Returns `true` if the two `Rc`s point to the same value (not just values
    /// that compare as equal).
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    /// let same_five = Rc::clone(&five);
    /// let other_five = Rc::new(Object(5));
    ///
    /// assert!(Rc::ptr_eq(&five, &same_five));
    /// assert!(!Rc::ptr_eq(&five, &other_five));
    /// ```
    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl<T: ?Sized> Rc<T> {
    // Allocates an `RcBox<T>` with sufficient space for an unsized value
    unsafe fn allocate_for_ptr(ptr: *const T) -> *mut RcBox<T> {
        // Calculate layout using the given value.
        // Previously, layout was calculated on the expression
        // `&*(ptr as *const RcBox<T>)`, but this created a misaligned
        // reference (see #54908).
        let layout = Layout::new::<RcBox<()>>()
            .extend(Layout::for_value(&*ptr))
            .unwrap()
            .0
            .pad_to_align()
            .unwrap();

        let mem = Global
            .alloc(layout)
            .unwrap_or_else(|_| handle_alloc_error(layout));

        // Initialize the RcBox
        let inner = set_data_ptr(ptr as *mut T, mem.as_ptr() as *mut u8) as *mut RcBox<T>;
        debug_assert_eq!(Layout::for_value(&*inner), layout);

        ptr::write(&mut (*inner).strong, Cell::new(1));
        ptr::write(&mut (*inner).weak, Cell::new(1));

        inner
    }

    fn from_box(v: Box<T>) -> Self {
        unsafe {
            let box_unique = Box::into_unique(v);
            let box_ptr = box_unique.as_ptr();

            let value_size = mem::size_of_val(&*box_ptr);
            let ptr = Self::allocate_for_ptr(box_ptr);

            // Copy value as bytes
            ptr::copy_nonoverlapping(
                box_ptr as *const T as *const u8,
                &mut (*ptr).value as *mut _ as *mut u8,
                value_size,
            );

            // Free the allocation without dropping its contents
            box_free(box_unique);

            Self {
                ptr: NonNull::new_unchecked(ptr),
                phantom: PhantomData,
            }
        }
    }
}

impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T: ?Sized + Clone> Rc<T> {
    /// Makes a mutable reference into the given `Rc`.
    ///
    /// If there are other `Rc` pointers to the same value, then `make_mut` will
    /// [`clone`](Clone) the inner value to ensure unique ownership.  This is
    /// also referred to as clone-on-write.
    ///
    /// If there are no other `Rc` pointers to this value, then [`Weak`]
    /// pointers to this value will be dissassociated.
    ///
    /// See also [`get_mut`](Rc::get_mut), which will fail rather than cloning.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// #[derive(Debug, Clone)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let mut data = Rc::new(Object(5));
    ///
    /// Rc::make_mut(&mut data).0 += 1;         // Won't clone anything
    /// let mut other_data = Rc::clone(&data); // Won't clone inner data
    /// Rc::make_mut(&mut data).0 += 1;         // Clones inner data
    /// Rc::make_mut(&mut data).0 += 1;         // Won't clone anything
    /// Rc::make_mut(&mut other_data).0 *= 2;   // Won't clone anything
    ///
    /// // Now `data` and `other_data` point to different values.
    /// assert_eq!(data.0, 8);
    /// assert_eq!(other_data.0, 12);
    /// ```
    ///
    /// [`Weak`] pointers will be dissassociated:
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// #[derive(Debug, Clone)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let mut data = Rc::new(Object(75));
    /// let weak = Rc::downgrade(&data);
    ///
    /// assert!(75 == data.0);
    /// assert!(75 == weak.upgrade().unwrap().0);
    ///
    /// Rc::make_mut(&mut data).0 += 1;
    ///
    /// assert!(76 == data.0);
    /// assert!(weak.upgrade().is_none());
    /// ```
    #[inline]
    pub fn make_mut(this: &mut Self) -> &mut T {
        if Self::strong_count(this) != 1 {
            // Gotta clone the data, there are other Rcs
            *this = Self::new((**this).clone())
        } else if Self::weak_count(this) != 0 {
            // Can just steal the data, all that's left is Weaks
            unsafe {
                let mut swap = Self::new(ptr::read(&this.ptr.as_ref().value));
                mem::swap(this, &mut swap);
                swap.dec_strong();
                // Remove implicit strong-weak ref (no need to craft a fake
                // Weak here -- we know other Weaks can clean up for us)
                swap.dec_weak();
                mem::forget(swap);
            }
        }
        // This unsafety is ok because we're guaranteed that the pointer
        // returned is the *only* pointer that will ever be returned to T. Our
        // reference count is guaranteed to be 1 at this point, and we required
        // the `Rc<T>` itself to be `mut`, so we're returning the only
        // possible reference to the inner value.
        unsafe { &mut this.ptr.as_mut().value }
    }
}

impl<T: ?Sized> Clone for Rc<T> {
    /// Makes a clone of the `Rc` pointer.
    ///
    /// This creates another pointer to the same inner value, increasing the
    /// strong reference count.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// let _ = Rc::clone(&five);
    /// ```
    #[inline]
    fn clone(&self) -> Self {
        self.inc_strong();
        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized + Default> Default for Rc<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Rc<T> {
    /// Equality for two `Rc`s.
    ///
    /// Two `Rc`s are equal if their inner values are equal.
    ///
    /// If `T` also implements `Eq`, two `Rc`s that point to the same value are
    /// always equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// #[derive(PartialEq, Eq)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert!(five == Rc::new(Object(5)));
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: ?Sized + Eq> Eq for Rc<T> {}

impl<T: ?Sized + PartialOrd> PartialOrd for Rc<T> {
    /// Partial comparison for two `Rc`s.
    ///
    /// The two are compared by calling `partial_cmp()` on their inner values.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    /// use std::cmp::Ordering;
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert_eq!(Some(Ordering::Less), five.partial_cmp(&Rc::new(Object(6))));
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    /// Less-than comparison for two `Rc`s.
    ///
    /// The two are compared by calling `<` on their inner values.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert!(five < Rc::new(Object(6)));
    /// ```
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        **self < **other
    }

    /// 'Less than or equal to' comparison for two `Rc`s.
    ///
    /// The two are compared by calling `<=` on their inner values.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert!(five <= Rc::new(Object(5)));
    /// ```
    #[inline]
    fn le(&self, other: &Self) -> bool {
        **self <= **other
    }

    /// Greater-than comparison for two `Rc`s.
    ///
    /// The two are compared by calling `>` on their inner values.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert!(five > Rc::new(Object(4)));
    /// ```
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        **self > **other
    }

    /// 'Greater than or equal to' comparison for two `Rc`s.
    ///
    /// The two are compared by calling `>=` on their inner values.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert!(five >= Rc::new(Object(5)));
    /// ```
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        **self >= **other
    }
}

impl<T: ?Sized + Ord> Ord for Rc<T> {
    /// Comparison for two `Rc`s.
    ///
    /// The two are compared by calling `cmp()` on their inner values.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    /// use std::cmp::Ordering;
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Object(i32);
    ///
    /// unsafe impl Reachable for Object {
    ///     fn object_id(&self) -> usize {
    ///         self.0 as usize
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let five = Rc::new(Object(5));
    ///
    /// assert_eq!(Ordering::Less, five.cmp(&Rc::new(Object(6))));
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: ?Sized + Hash> Hash for Rc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner().value, f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner().value, f)
    }
}

impl<T> From<T> for Rc<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: ?Sized> From<Box<T>> for Rc<T> {
    #[inline]
    fn from(v: Box<T>) -> Self {
        Self::from_box(v)
    }
}

impl<T: ?Sized> borrow::Borrow<T> for Rc<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}
