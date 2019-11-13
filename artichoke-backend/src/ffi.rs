//! Functions for interacting directly with mruby structs from [`sys`].
//!
//! These functions are unsafe. Use them carefully.

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::{Artichoke, ArtichokeError};

/// Extract an [`Artichoke`] interpreter from the user data pointer on a
/// [`sys::mrb_state`].
///
/// Calling this function will increase the [`Rc::strong_count`] on the
/// [`Artichoke`] interpreter by one.
///
/// # Safety
///
/// This function assumes that the user data pointer was created with
/// [`Rc::into_raw`] and that the pointer is to a non-free'd
/// [`Rc`]`<`[`RefCell`]`<`[`State`]`>>`.
pub unsafe fn from_user_data(mrb: *mut sys::mrb_state) -> Result<Artichoke, ArtichokeError> {
    if mrb.is_null() {
        error!("Attempted to extract Artichoke from null mrb_state");
        return Err(ArtichokeError::Uninitialized);
    }
    let ptr = (*mrb).ud;
    if ptr.is_null() {
        info!("Attempted to extract Artichoke from null mrb_state->ud pointer");
        return Err(ArtichokeError::Uninitialized);
    }
    // Extract the smart pointer that wraps the API from the user data on
    // the mrb interpreter. The `mrb_state` should retain ownership of its
    // copy of the smart pointer.
    let ud = Rc::from_raw(ptr as *const RefCell<State>);
    // Clone the API smart pointer and increase its ref count to return a
    // reference to the caller.
    let api = Rc::clone(&ud);
    // Forget the transmuted API extracted from the user data to make sure
    // the `mrb_state` maintains ownership and the smart pointer does not
    // get deallocated before `mrb_close` is called.
    mem::forget(ud);
    // At this point, `Rc::strong_count` will be increased by 1.
    trace!(
        "Extracted Artichoke from user data pointer on {}",
        mrb.debug()
    );
    Ok(Artichoke(api))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::ArtichokeError;

    #[test]
    fn from_user_data_null_pointer() {
        let err = unsafe { super::from_user_data(std::ptr::null_mut()) };
        assert_eq!(err.err(), Some(ArtichokeError::Uninitialized));
    }

    #[test]
    fn from_user_data_null_user_data() {
        let interp = crate::interpreter().expect("init");
        let mrb = interp.0.borrow().mrb;
        unsafe {
            // fake null user data
            (*mrb).ud = std::ptr::null_mut();
        }
        let err = unsafe { super::from_user_data(mrb) };
        assert_eq!(err.err(), Some(ArtichokeError::Uninitialized));
    }

    #[test]
    fn from_user_data() {
        let interp = crate::interpreter().expect("init");
        let mrb = interp.0.borrow().mrb;
        let res = unsafe { super::from_user_data(mrb) };
        assert!(res.is_ok());
    }

    #[test]
    #[should_panic]
    // This test is no longer valid now that initializing the core creates
    // `Array` objects which clone the interpreter.
    fn from_user_data_rc_refcount() {
        let interp = crate::interpreter().expect("init");
        assert_eq!(Rc::strong_count(&interp.0), 1);
        let mrb = interp.0.borrow().mrb;
        let res = unsafe { super::from_user_data(mrb) };
        assert_eq!(Rc::strong_count(&interp.0), 2);
        assert!(res.is_ok());
        drop(res);
        assert_eq!(Rc::strong_count(&interp.0), 1);
    }
}
