use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem;
use core::ops::Deref;
use core::pin::Pin;
use core::ptr::{self, NonNull};
use itertools::Itertools;
use std::alloc::{handle_alloc_error, Alloc, Global, Layout};
use std::borrow;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::link::Link;
use crate::ptr::{box_free, data_offset, is_dangling, set_data_ptr, RcBox, RcBoxPtr};
use crate::{Reachable, Weak};

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
pub struct Rc<T: ?Sized + Reachable> {
    pub(crate) ptr: NonNull<RcBox<T>>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: ?Sized> !Send for Rc<T> {}
impl<T: ?Sized> !Sync for Rc<T> {}

impl<T: Reachable> Rc<T> {
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
                links: RefCell::new(HashSet::default()),
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

impl<T: ?Sized + Reachable> Rc<T> {
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

impl<T: ?Sized + Reachable> Rc<T> {
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

impl<T: ?Sized + Reachable> Deref for Rc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner().value
    }
}

unsafe impl<#[may_dangle] T: ?Sized + Reachable> Drop for Rc<T> {
    /// Drops the `Rc`.
    ///
    /// This will decrement the strong reference count. If the strong reference
    /// count reaches zero then the only other references (if any) are
    /// [`Weak`], so we `drop` the inner value.
    ///
    /// If this `Rc` has adopted any other `Rc`s, drop will trace the reachable
    /// object graph and detect if this `Rc` is part of an orphaned cycle. An
    /// orphaned cycle is a cycle in which all members have no owned references
    /// held by `Rc`s outside of the cycle.
    ///
    /// Cycle detection is a zero-cost abstraction. `Rc`s do not pay the cost of
    /// the reachability check unless they use [`Rc::adopt`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cactusref::{Rc, Reachable};
    ///
    /// struct Foo;
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped!");
    ///     }
    /// }
    ///
    /// unsafe impl Reachable for Foo {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let foo  = Rc::new(Foo);
    /// let foo2 = Rc::clone(&foo);
    ///
    /// drop(foo);    // Doesn't print anything
    /// drop(foo2);   // Prints "dropped!"
    /// ```
    ///
    /// ```
    /// use cactusref::{Adoptable, Rc, Reachable};
    ///
    /// struct Foo(u8);
    ///
    /// impl Drop for Foo {
    ///     fn drop(&mut self) {
    ///         println!("dropped {}!", self.0);
    ///     }
    /// }
    ///
    /// unsafe impl Reachable for Foo {
    ///     fn object_id(&self) -> usize {
    ///         0
    ///     }
    ///
    ///     fn can_reach(&self, _object_id: usize) -> bool {
    ///         false
    ///     }
    /// }
    ///
    /// let foo  = Rc::new(Foo(10));
    /// let foo2 = Rc::new(Foo(20));
    ///
    /// Rc::adopt(&foo, &foo2);
    /// Rc::adopt(&foo2, &foo);
    ///
    /// drop(foo);    // Doesn't print anything
    /// drop(foo2);   // Prints "dropped 10!" and "dropped 20!"
    /// ```
    ///
    /// # Cycle Detection and Deallocation Algorithm
    ///
    /// `Rc::adopt` does explicit bookkeeping to store links to adoptee `Rc`s.
    /// Each link increases the strong count on the adoptee but does not
    /// allocate another `Rc`.
    ///
    /// On drop, if a `Rc` has no links, it is dropped like a normal `Rc`. If
    /// the `Rc` has links, it performs a breadth first search using its wrapped
    /// value's Reachable implementation to determine the all `Rc`s that it can
    /// reach.
    ///
    /// After determining all reachable objects, `Rc` reduces the graph to
    /// objects that form a cycle by performing pairwise reachability checks.
    /// During this step, for each object in the cycle, `Rc` counts the number
    /// of refs held by other objects in the cycle.
    ///
    /// Using the cycle-held strong refs, `Rc` computes whether the object graph
    /// is reachable by any non-cycle nodes by comparing strong counts.
    ///
    /// If the cycle is orphaned, `Rc` busts all the link `HashSet`s and
    /// deallocates each object.
    fn drop(&mut self) {
        unsafe {
            self.dec_strong();

            // If links is empty, the object is either not in a cycle or part of
            // a cycle that has been link busted for deallocation.
            if self.inner().links.borrow().is_empty() {
                if self.strong() == 0 {
                    // destroy the contained object
                    ptr::drop_in_place(self.ptr.as_mut());

                    // remove the implicit "strong weak" pointer now that we've
                    // destroyed the contents.
                    self.dec_weak();

                    if self.weak() == 0 {
                        Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                    }
                }
                return;
            }
            // Perform a breadth first search over all of the links to determine
            // the clique of refs that self can reach.
            let mut clique = HashSet::new();
            clique.insert(Link(self.ptr));
            let mut strong_counts_in_cycle = HashMap::new();
            loop {
                let size = clique.len();
                for item in clique.clone() {
                    let links = item.0.as_ref().links.borrow();
                    for link in links.iter() {
                        clique.insert(*link);
                    }
                }
                // BFS has found no new refs in the clique.
                if size == clique.len() {
                    break;
                }
            }
            // Iterate over the items in the clique. For each pair of nodes,
            // find nodes that can reach each other. These nodes form a cycle.
            let mut cycle = HashSet::new();
            for (left, right) in clique
                .iter()
                .cartesian_product(clique.iter())
                .filter(|(left, right)| left != right)
            {
                let left_reaches_right = left.value().can_reach(right.value().object_id());
                let right_reaches_left = right.value().can_reach(left.value().object_id());
                let is_new = !cycle.iter().any(|item: &Link<T>| *item == *right);
                if left_reaches_right && right_reaches_left && is_new {
                    cycle.insert(*right);
                    let count = *strong_counts_in_cycle.get(&right).unwrap_or(&0);
                    strong_counts_in_cycle.insert(right, count + 1);
                }
            }
            let cycle_has_external_owners = cycle.iter().any(|item| {
                let cycle_strong_count = strong_counts_in_cycle[item];
                item.0.as_ref().strong() > cycle_strong_count
            });
            if !cycle.is_empty() && !cycle_has_external_owners {
                let ids = cycle
                    .iter()
                    .map(|item| item.value().object_id())
                    .collect::<HashSet<_>>();
                debug!("orphaned cycle detected with object ids: {:?}", ids);
                // Break the cycle and remove all links to prevent loops when
                // dropping cycle refs.
                for (left, right) in cycle
                    .iter()
                    .cartesian_product(cycle.iter())
                    .filter(|(left, right)| left != right)
                {
                    let mut links = left.0.as_ref().links.borrow_mut();
                    links.remove(right);
                }
                for mut obj in cycle {
                    debug!("dropping cycle participant {{{}}}", obj.value().object_id());
                    // destroy the contained object
                    ptr::drop_in_place(obj.0.as_mut());
                }

                // remove the implicit "strong weak" pointer now that we've
                // destroyed the contents.
                self.dec_weak();

                if self.weak() == 0 {
                    Global.dealloc(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                }
            }
        }
    }
}

impl<T: ?Sized + Clone + Reachable> Rc<T> {
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

impl<T: ?Sized + Reachable> Clone for Rc<T> {
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

impl<T: ?Sized + Reachable + Default> Default for Rc<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized + PartialEq + Reachable> PartialEq for Rc<T> {
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

impl<T: ?Sized + Eq + Reachable> Eq for Rc<T> {}

impl<T: ?Sized + PartialOrd + Reachable> PartialOrd for Rc<T> {
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

impl<T: ?Sized + Ord + Reachable> Ord for Rc<T> {
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

impl<T: ?Sized + Hash + Reachable> Hash for Rc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized + Reachable + fmt::Display> fmt::Display for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner().value, f)
    }
}

impl<T: ?Sized + Reachable + fmt::Debug> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner().value, f)
    }
}

impl<T: Reachable> From<T> for Rc<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T: ?Sized + Reachable> From<Box<T>> for Rc<T> {
    #[inline]
    fn from(v: Box<T>) -> Self {
        Self::from_box(v)
    }
}

impl<T: ?Sized + Reachable> borrow::Borrow<T> for Rc<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized + Reachable> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}
