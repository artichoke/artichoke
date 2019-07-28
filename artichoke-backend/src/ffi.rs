//! Functions for interacting directly with [`mruby_sys`] structs.
//!
//! These functions are unsafe. Use them carefully.

use log::{error, trace};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::state::State;
use crate::sys::{self, DescribeState};
use crate::{ArtichokeError, Mrb};

/// Extract an [`Mrb`] interpreter from the userdata pointer on a
/// [`sys::mrb_state`].
///
/// This function is unsafe! It manipulates a raw pointer stored as a
/// [`c_void`](std::ffi::c_void) on `mrb->ud`. `from_user_data` assumes that
/// this [`c_void`](std::ffi::c_void) was created with [`Rc::into_raw`], see
/// calling this function, [`Rc::strong_count`] on the [`Mrb`] instance will
/// increase by one.
pub unsafe fn from_user_data(mrb: *mut sys::mrb_state) -> Result<Mrb, ArtichokeError> {
    if mrb.is_null() {
        error!("Attempted to extract Mrb from null mrb_state");
        return Err(ArtichokeError::Uninitialized);
    }
    let ptr = (*mrb).ud;
    if ptr.is_null() {
        error!("Attempted to extract Mrb from null mrb_state->ud pointer");
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
    trace!("Extracted Mrb from user data pointer on {}", mrb.debug());
    Ok(api)
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
        let interp = crate::interpreter().expect("mrb init");
        let mrb = interp.borrow().mrb;
        unsafe {
            // fake null user data
            (*mrb).ud = std::ptr::null_mut();
        }
        let err = unsafe { super::from_user_data(mrb) };
        assert_eq!(err.err(), Some(ArtichokeError::Uninitialized));
    }

    #[test]
    fn from_user_data() {
        let interp = crate::interpreter().expect("mrb init");
        let mrb = interp.borrow().mrb;
        let res = unsafe { super::from_user_data(mrb) };
        assert!(res.is_ok());
    }

    #[test]
    fn from_user_data_rc_refcount() {
        let interp = crate::interpreter().expect("mrb init");
        assert_eq!(Rc::strong_count(&interp), 1);
        let mrb = interp.borrow().mrb;
        let res = unsafe { super::from_user_data(mrb) };
        assert_eq!(Rc::strong_count(&interp), 2);
        assert!(res.is_ok());
        drop(res);
        assert_eq!(Rc::strong_count(&interp), 1);
    }
}
