use std::ffi::c_void;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use crate::ffi::{self, InterpreterExtractError};
use crate::state::State;
use crate::sys;
use crate::value::Value;

/// Interpreter instance.
///
/// Functionality is added to the interpreter via traits, for example,
/// [garbage collection](crate::gc::MrbGarbageCollection) or
/// [eval](crate::core::Eval).
#[derive(Debug)]
pub struct Artichoke {
    /// Underlying mruby interpreter.
    ///
    /// This is an owned reference to the interpreter via a mutable pointer.
    pub mrb: NonNull<sys::mrb_state>,

    /// Interpreter state.
    ///
    /// This field is an `Option` because the `State` is moved in and out of the
    /// `Artichoke` struct as the call graph crosses between Rust and C and C to
    /// Rust.
    pub state: Option<Box<State>>,
}

impl Artichoke {
    /// Create a new interpreter from an underlying `mrb` and a `State`.
    #[must_use]
    pub const fn new(mrb: NonNull<sys::mrb_state>, state: Box<State>) -> Self {
        let state = Some(state);
        Self { mrb, state }
    }

    /// Prevent the given value from being garbage collected.
    ///
    /// Calls [`sys::mrb_gc_protect`] on this value which adds it to the GC
    /// arena. This object will remain in the arena until [`ArenaIndex::restore`]
    /// restores the arena to an index before this call to protect.
    ///
    /// [`ArenaIndex::restore`]: crate::gc::arena::ArenaIndex::restore
    pub fn protect(&mut self, value: Value) -> Value {
        unsafe {
            let value = value.inner();
            let _ = self.with_ffi_boundary(|mrb| sys::mrb_gc_protect(mrb, value));
        }
        value
    }

    /// Execute a a closure by moving the [`State`] into the `mrb` instance.
    ///
    /// This method prepares this interpreter to cross an FFI boundary. When the
    /// Artichoke implementation calls mruby FFI functions, the `State` must be
    /// moved into the [`sys::mrb_state`] userdata pointer.
    ///
    /// # Safety
    ///
    /// This method moves the `State` out of this instance into the `mrb`
    /// instance. During this function's execution, this instance may be
    /// partially initialized.
    ///
    /// This function is only safe to call if the closure only calls FFI
    /// functions that use a raw `*mut sys::mrb_state`.
    pub unsafe fn with_ffi_boundary<F, T>(&mut self, func: F) -> Result<T, InterpreterExtractError>
    where
        F: FnOnce(*mut sys::mrb_state) -> T,
    {
        if let Some(state) = self.state.take() {
            // Ensure we don't create multiple mutable references by moving the
            // `mrb` out of the `Artichoke` and converting to a raw pointer.
            //
            // Safety:
            //
            // 1. Extract a `*mut sys::mrb_state` pointer from the `NonNull`
            //    `mrb` field.
            // 2. Function safety conditions declare that `Artichoke` is not
            //    accessed inside the closure.
            // 3. Rust borrowing rules enforce that `Artichoke` is not accessed
            //    inside the closure.
            // 4. This function moves the `State` into the `mrb`.
            // 5. If `mrb` re-enters `Artichoke` via trampoline, a new
            //    `Artichoke` is made by moving the `State` out of the `mrb`.
            // 6. The `Artichoke` in the FFI entry point is wrapped in a `Guard`.
            // 7. On drop, `Guard` moves the `State` back into the `mrb`.
            // 8. On return from `mrb`, here, extract the `State` which should be
            //    moved back into the `mrb`.
            // 9. Replace `self` with the new interpreter.

            // Step 1
            let mrb = self.mrb.as_ptr();

            // Step 4
            (*mrb).ud = Box::into_raw(state).cast::<c_void>();

            // Steps 5-7
            let result = func(mrb);

            // Step 8
            let extracted = ffi::from_user_data(mrb)?;

            // Step 9
            self.state = extracted.state;
            Ok(result)
        } else {
            Err(InterpreterExtractError::new())
        }
    }

    /// Consume an interpreter and return the pointer to the underlying
    /// [`sys::mrb_state`].
    ///
    /// This function does not free any interpreter resources. Its intended use
    /// is to prepare the interpreter to cross over an FFI boundary.
    ///
    /// This is an associated function and must be called as
    /// `Artichoke::into_raw(interp)`.
    ///
    /// # Safety
    ///
    /// After calling this function, the caller is responsible for properly
    /// freeing the memory occupied by the interpreter heap. The easiest way to
    /// do this is to call [`ffi::from_user_data`] with the returned pointer and
    /// then call [`Artichoke::close`].
    #[must_use]
    pub unsafe fn into_raw(mut interp: Self) -> *mut sys::mrb_state {
        let mut guard = Guard::new(&mut interp);
        guard.interp().mrb.as_ptr()
    }

    /// Consume an interpreter and free all live objects.
    pub fn close(mut self) {
        // Safety:
        //
        // It is permissible to directly access the `*mut sys::mrb_state`
        // because we are tearing down the interpreter. The only `MRB_API`
        // calls made from this point are related to freeing interpreter
        // memory.
        let mrb = unsafe { self.mrb.as_mut() };
        if let Some(state) = self.state.take() {
            // Do not free class and module specs before running the final
            // garbage collection on `mrb_close`.
            let State {
                parser,
                classes,
                modules,
                ..
            } = *state;

            // Safety
            //
            // - The parser must be deallocated to free the associated
            //   `mrbc_context`.
            // - The parser must be freed before the `mrb_state` because the
            //   `mrb_state` may hold a copy of the context pointer.
            // - `classes` and `modules` from the Artichoke Rust `State`
            //   must be live allocations before calling `mrb_close` because
            //   these registries allow resolving the `dfree` free functions
            //   for Ruby types defined with type tag `MRB_TT_DATA`.
            unsafe {
                if let Some(parser) = parser {
                    parser.close(mrb);
                }
                sys::mrb_close(mrb);
            }

            drop(classes);
            drop(modules);
        } else {
            // Safety
            //
            // If there is no Artichoke Rust `State`, the mruby interpreter
            // cannot be safely closed. Prefer to leak the interpreter than
            // try to close it.
            let _ = mrb;
        }
    }
}

/// Interpreter guard that prepares an [`Artichoke`] to re-enter an FFI
/// boundary.
///
/// Artichoke integrates with the mruby VM via many `extern "C" fn` trampolines
/// that are invoked by mruby to run some portion of the VM in Rust.
///
/// These trampolines typically require an [`Artichoke`] interpreter to do
/// useful work, so they move the [`State`](crate::state::State) out of the
/// `mrb` userdata pointer into an `Artichoke` struct.
///
/// To ensure safety, the `State` must be moved back into the `mrb` userdata
/// pointer before re-entering the FFI boundary. This guard implements [`Drop`]
/// to re-serialize the `State` into the `mrb` once it goes out of scope.
///
/// `Guard` is passed directly to [`error::raise`](crate::error::raise).
#[derive(Debug)]
pub struct Guard<'a>(&'a mut Artichoke);

impl<'a> Guard<'a> {
    /// Create a new guard that wraps an interpreter.
    ///
    /// This function is most effective when the interpreter is temporarily
    /// created from a source `mrb_state` and stored on the stack.
    pub fn new(interp: &'a mut Artichoke) -> Self {
        Self(interp)
    }

    /// Access the inner guarded interpreter.
    ///
    /// The interpreter is also accessible via [`Deref`], [`DerefMut`],
    /// [`AsRef`], and [`AsMut`].
    #[inline]
    pub fn interp(&mut self) -> &mut Artichoke {
        self.0
    }
}

impl<'a> AsRef<Artichoke> for Guard<'a> {
    #[inline]
    fn as_ref(&self) -> &Artichoke {
        &*self.0
    }
}

impl<'a> AsMut<Artichoke> for Guard<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Artichoke {
        self.0
    }
}

impl<'a> Deref for Guard<'a> {
    type Target = Artichoke;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a> DerefMut for Guard<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        let state = self.0.state.take();
        let state = state.unwrap_or_else(|| panic!("Dropping Guard with no State"));

        unsafe {
            let mrb = self.0.mrb.as_ptr();
            (*mrb).ud = Box::into_raw(state).cast::<c_void>();
        }
    }
}
