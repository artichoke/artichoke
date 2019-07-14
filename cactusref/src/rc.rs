use core::borrow;
use core::cell::{Cell, RefCell};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::{PhantomData, Unpin};
use core::mem;
use core::ops::Deref;
use core::pin::Pin;
use core::ptr::{self, NonNull};
use core::slice;

use std::alloc::{handle_alloc_error, Alloc, Global, Layout};

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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
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
                link: Cell::new(0),
                links: RefCell::new(Links::default()),
                back_links: RefCell::new(Links::default()),
                value,
            })),
            phantom: PhantomData,
        }
    }

    /// Constructs a new [`Pin`]`<`[`Rc`]`<T>>`. If `T` does not implement
    /// [`Unpin`], then `value` will be pinned in memory and unable to be moved.
    pub fn pin(value: T) -> Pin<Self> {
        unsafe { Pin::new_unchecked(Self::new(value)) }
    }

    /// Returns the contained value, if the `Rc` has exactly one strong
    /// reference.
    ///
    /// Otherwise, an [`Err`] is returned with the same `Rc` that was passed in.
    ///
    /// This will succeed even if there are outstanding weak references.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let x = Rc::new(3);
    /// assert_eq!(Rc::try_unwrap(x), Ok(3));
    ///
    /// let x = Rc::new(4);
    /// let _y = Rc::clone(&x);
    /// assert_eq!(*Rc::try_unwrap(x).unwrap_err(), 4);
    /// ```
    #[inline]
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
    /// To avoid a memory leak the pointer must be converted back to an `Rc`
    /// using [`Rc::from_raw`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let x = Rc::new("hello".to_owned());
    /// let x_ptr = Rc::into_raw(x);
    /// assert_eq!(unsafe { &*x_ptr }, "hello");
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
    /// use cactusref::Rc;
    ///
    /// let x = Rc::new("hello".to_owned());
    /// let x_ptr = Rc::into_raw(x);
    ///
    /// unsafe {
    ///     // Convert back to an `Rc` to prevent leak.
    ///     let x = Rc::from_raw(x_ptr);
    ///     assert_eq!(&*x, "hello");
    ///
    ///     // Further calls to `Self::from_raw(x_ptr)` would be memory unsafe.
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

    /// Consumes the `Rc`, returning the wrapped pointer as [`NonNull`]`<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let x = Rc::new("hello".to_owned());
    /// let ptr = Rc::into_raw_non_null(x);
    /// let deref = unsafe { ptr.as_ref() };
    /// assert_eq!(deref, "hello");
    /// ```
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn into_raw_non_null(this: Self) -> NonNull<T> {
        // safe because Rc guarantees its pointer is non-null
        unsafe { NonNull::new_unchecked(Self::into_raw(this) as *mut _) }
    }

    /// Creates a new [`Weak`][weak] pointer to this value.
    ///
    /// [weak]: struct.Weak.html
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    /// let _also_five = Rc::clone(&five);
    ///
    /// assert_eq!(2, Rc::strong_count(&five));
    /// ```
    #[inline]
    pub fn strong_count(this: &Self) -> usize {
        this.strong() - this.link()
    }

    /// Returns `true` if there are no other `Rc` or [`Weak`] pointers to this
    /// inner value.
    ///
    /// [weak]: struct.Weak.html
    #[inline]
    pub(crate) fn is_unique(this: &Self) -> bool {
        Self::weak_count(this) == 0 && Self::strong_count(this) == 1
    }

    /// Returns a mutable reference to the inner value, if there are
    /// no other `Rc` or [`Weak`] pointers to the same value.
    ///
    /// Returns [`None`] otherwise, because it is not safe to
    /// mutate a shared value.
    ///
    /// See also [`make_mut`](Rc::make_mut), which will [`clone`](Clone::clone)
    /// the inner value when it's shared.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let mut x = Rc::new(3);
    /// *Rc::get_mut(&mut x).unwrap() = 4;
    /// assert_eq!(*x, 4);
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

    /// Returns `true` if the two `Rc`s point to the same value (not
    /// just values that compare as equal).
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    /// let same_five = Rc::clone(&five);
    /// let other_five = Rc::new(5);
    ///
    /// assert!(Rc::ptr_eq(&five, &same_five));
    /// assert!(!Rc::ptr_eq(&five, &other_five));
    /// ```
    #[inline]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl<T: Clone> Rc<T> {
    /// Makes a mutable reference into the given `Rc`.
    ///
    /// If there are other `Rc` pointers to the same value, then `make_mut` will
    /// [`clone`](Clone::clone) the inner value to ensure unique ownership. This
    /// is also referred to as clone-on-write.
    ///
    /// If there are no other `Rc` pointers to this value, then [`Weak`]
    /// pointers to this value will be dissassociated.
    ///
    /// See also [`get_mut`](Rc::get_mut), which will fail rather than cloning.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let mut data = Rc::new(5);
    ///
    /// *Rc::make_mut(&mut data) += 1;        // Won't clone anything
    /// let mut other_data = Rc::clone(&data);    // Won't clone inner data
    /// *Rc::make_mut(&mut data) += 1;        // Clones inner data
    /// *Rc::make_mut(&mut data) += 1;        // Won't clone anything
    /// *Rc::make_mut(&mut other_data) *= 2;  // Won't clone anything
    ///
    /// // Now `data` and `other_data` point to different values.
    /// assert_eq!(*data, 8);
    /// assert_eq!(*other_data, 12);
    /// ```
    ///
    /// [`Weak`] pointers will be dissassociated:
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let mut data = Rc::new(75);
    /// let weak = Rc::downgrade(&data);
    ///
    /// assert!(75 == *data);
    /// assert!(75 == *weak.upgrade().unwrap());
    ///
    /// *Rc::make_mut(&mut data) += 1;
    ///
    /// assert!(76 == *data);
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
        // the `Rc<T>` itself to be `mut`, so we're returning the only possible
        // reference to the inner value.
        unsafe { &mut this.ptr.as_mut().value }
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
        ptr::write(&mut (*inner).link, Cell::new(0));
        ptr::write(&mut (*inner).links, RefCell::new(Links::default()));
        ptr::write(&mut (*inner).back_links, RefCell::new(Links::default()));

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

impl<T> Rc<[T]> {
    // Copy elements from slice into newly allocated Rc<[T]>
    //
    // Unsafe because the caller must either take ownership or bind `T: Copy`
    unsafe fn copy_from_slice(v: &[T]) -> Self {
        let v_ptr = v as *const [T];
        let ptr = Self::allocate_for_ptr(v_ptr);

        ptr::copy_nonoverlapping(v.as_ptr(), &mut (*ptr).value as *mut [T] as *mut T, v.len());

        Self {
            ptr: NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        }
    }
}

trait RcFromSlice<T> {
    fn from_slice(slice: &[T]) -> Self;
}

impl<T: Clone> RcFromSlice<T> for Rc<[T]> {
    #[inline]
    default fn from_slice(v: &[T]) -> Self {
        // Panic guard while cloning T elements.
        // In the event of a panic, elements that have been written
        // into the new RcBox will be dropped, then the memory freed.
        struct Guard<T> {
            mem: NonNull<u8>,
            elems: *mut T,
            layout: Layout,
            n_elems: usize,
        }

        impl<T> Drop for Guard<T> {
            fn drop(&mut self) {
                unsafe {
                    let slice = slice::from_raw_parts_mut(self.elems, self.n_elems);
                    ptr::drop_in_place(slice);

                    Global.dealloc(self.mem, self.layout);
                }
            }
        }

        unsafe {
            let v_ptr = v as *const [T];
            let ptr = Self::allocate_for_ptr(v_ptr);

            let mem = ptr as *mut _ as *mut u8;
            let layout = Layout::for_value(&*ptr);

            // Pointer to first element
            let elems = &mut (*ptr).value as *mut [T] as *mut T;

            let mut guard = Guard {
                mem: NonNull::new_unchecked(mem),
                elems,
                layout,
                n_elems: 0,
            };

            for (i, item) in v.iter().enumerate() {
                ptr::write(elems.add(i), item.clone());
                guard.n_elems += 1;
            }

            // All clear. Forget the guard so it doesn't free the new RcBox.
            mem::forget(guard);

            Self {
                ptr: NonNull::new_unchecked(ptr),
                phantom: PhantomData,
            }
        }
    }
}

impl<T: Copy> RcFromSlice<T> for Rc<[T]> {
    #[inline]
    fn from_slice(v: &[T]) -> Self {
        unsafe { Self::copy_from_slice(v) }
    }
}

impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner().value
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
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

impl<T: Default> Default for Rc<T> {
    /// Creates a new `Rc<T>`, with the `Default` value for `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let x: Rc<i32> = Default::default();
    /// assert_eq!(*x, 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

trait RcEqIdent<T: ?Sized + PartialEq> {
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
}

impl<T: ?Sized + PartialEq> RcEqIdent<T> for Rc<T> {
    #[inline]
    default fn eq(&self, other: &Self) -> bool {
        **self == **other
    }

    #[inline]
    default fn ne(&self, other: &Self) -> bool {
        **self != **other
    }
}

/// We're doing this specialization here, and not as a more general optimization
/// on `&T`, because it would otherwise add a cost to all equality checks on
/// refs. We assume that `Rc`s are used to store large values, that are slow to
/// clone, but also heavy to check for equality, causing this cost to pay off
/// more easily. It's also more likely to have two `Rc` clones, that point to
/// the same value, than two `&T`s.
impl<T: ?Sized + Eq> RcEqIdent<T> for Rc<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Self::ptr_eq(self, other) || **self == **other
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !Self::ptr_eq(self, other) && **self != **other
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert!(five == Rc::new(5));
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        RcEqIdent::eq(self, other)
    }

    /// Inequality for two `Rc`s.
    ///
    /// Two `Rc`s are unequal if their inner values are unequal.
    ///
    /// If `T` also implements `Eq`, two `Rc`s that point to the same value are
    /// never unequal.
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert!(five != Rc::new(6));
    /// ```
    #[inline]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        RcEqIdent::ne(self, other)
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
    /// use cactusref::Rc;
    /// use std::cmp::Ordering;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert_eq!(Some(Ordering::Less), five.partial_cmp(&Rc::new(6)));
    /// ```
    #[inline(always)]
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert!(five < Rc::new(6));
    /// ```
    #[inline(always)]
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert!(five <= Rc::new(5));
    /// ```
    #[inline(always)]
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert!(five > Rc::new(4));
    /// ```
    #[inline(always)]
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
    /// use cactusref::Rc;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert!(five >= Rc::new(5));
    /// ```
    #[inline(always)]
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
    /// use cactusref::Rc;
    /// use std::cmp::Ordering;
    ///
    /// let five = Rc::new(5);
    ///
    /// assert_eq!(Ordering::Less, five.cmp(&Rc::new(6)));
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
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> fmt::Pointer for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&(&**self as *const T), f)
    }
}

impl<T> From<T> for Rc<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: Clone> From<&[T]> for Rc<[T]> {
    #[inline]
    fn from(v: &[T]) -> Self {
        <Self as RcFromSlice<T>>::from_slice(v)
    }
}

#[allow(clippy::use_self)]
impl From<&str> for Rc<str> {
    #[inline]
    fn from(v: &str) -> Self {
        let rc = Rc::<[u8]>::from(v.as_bytes());
        unsafe { Self::from_raw(Rc::into_raw(rc) as *const str) }
    }
}

impl From<String> for Rc<str> {
    #[inline]
    fn from(v: String) -> Self {
        Self::from(&v[..])
    }
}

impl<T: ?Sized> From<Box<T>> for Rc<T> {
    #[inline]
    fn from(v: Box<T>) -> Self {
        Self::from_box(v)
    }
}

impl<T> From<Vec<T>> for Rc<[T]> {
    #[inline]
    fn from(mut v: Vec<T>) -> Self {
        unsafe {
            let rc = Self::copy_from_slice(&v);

            // Allow the Vec to free its memory, but not destroy its contents
            v.set_len(0);

            rc
        }
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

impl<T: ?Sized> Unpin for Rc<T> {}
