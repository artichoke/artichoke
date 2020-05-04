use std::ffi::c_void;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use crate::ffi::{self, InterpreterExtractError};
use crate::state::State;
use crate::sys;

/// Interpreter instance.
///
/// The interpreter [`State`](state::State) is wrapped in an `Rc<RefCell<_>>`.
///
/// The [`Rc`] enables the State to be cloned so it can be stored in the
/// [`sys::mrb_state`],
/// [extracted in `extern "C"` functions](ffi::from_user_data), and used in
/// [`Value`](value::Value) instances.
///
/// The [`RefCell`] enables mutable access to the underlying
/// [`State`](state::State), even across an FFI boundary.
///
/// Functionality is added to the interpreter via traits, for example,
/// [garbage collection](gc::MrbGarbageCollection) or [eval](eval::Eval).
#[derive(Debug)]
pub struct Artichoke {
    pub mrb: NonNull<sys::mrb_state>,
    pub state: Option<Box<State>>,
}

impl Artichoke {
    pub fn new(mrb: NonNull<sys::mrb_state>, state: Box<State>) -> Self {
        Self {
            mrb,
            state: Some(state),
        }
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
    #[must_use]
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
            // - The `Artichoke` struct is partially uninitialized with the
            //   dangling `NonNull`.
            // - Function safety conditions declare that `Artichoke` is not
            //   accessed inside the closure.
            // - Rust borrowing rules enforce that `Artichoke` is not accessed
            //   inside the closure.
            // - This function moves the `State` into the `mrb`.
            // - If `mrb` re-enters Artichoke via trampoline, a new Artichoke is
            //   made by moving the `State` out of the `mrb` via an
            //   `ArtichokeGuard`.
            // - On drop, `ArtichokeGuard` moves the `State` back into the
            //   `mrb`.
            // - On return from `mrb`, here, extract the `State` which should be
            //   moved back into the `mrb` and replace `self` with the new
            //   interpreter.
            let mrb = mem::replace(&mut self.mrb, NonNull::dangling());
            let mrb = mrb.as_ptr();
            (*mrb).ud = Box::into_raw(state) as *mut c_void;
            let result = func(mrb);
            let extracted = ffi::from_user_data(mrb)?;
            *self = extracted;
            Ok(result)
        } else {
            Err(InterpreterExtractError)
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
        let mrb = interp.mrb.as_mut();
        if let Some(state) = interp.state {
            mrb.ud = Box::into_raw(state) as *mut c_void;
        } else {
            error!("Called Artichoke::into_raw with no State");
        }
        mrb
    }

    /// Consume an interpreter and free all live objects.
    pub fn close(mut self) {
        unsafe {
            let mrb = self.mrb.as_mut();
            if let Some(state) = self.state {
                state.close(mrb);
            }
            sys::mrb_close(mrb);
        }
    }
}

#[derive(Debug)]
pub struct Guard<'a>(&'a mut Artichoke);

impl<'a> Guard<'a> {
    pub fn new(interp: &'a mut Artichoke) -> Self {
        Self(interp)
    }

    pub fn interp(&mut self) -> &mut Artichoke {
        self.0
    }
}

impl<'a> AsRef<Artichoke> for Guard<'a> {
    fn as_ref(&self) -> &Artichoke {
        &*self.0
    }
}

impl<'a> AsMut<Artichoke> for Guard<'a> {
    fn as_mut(&mut self) -> &mut Artichoke {
        self.0
    }
}

impl<'a> Deref for Guard<'a> {
    type Target = Artichoke;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a> DerefMut for Guard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        unsafe {
            let mrb = self.0.mrb.as_mut();
            if let Some(state) = self.0.state.take() {
                trace!("Serializing Artichoke State into mrb to prepare for FFI boundary");
                mrb.ud = Box::into_raw(state) as *mut c_void;
            } else {
                error!("Dropping Guard with no State");
            }
        }
    }
}
