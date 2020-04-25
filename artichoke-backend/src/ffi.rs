//! Functions for interacting directly with mruby structs from [`sys`].
//!
//! These functions are unsafe. Use them carefully.

use bstr::{ByteSlice, ByteVec};
use std::error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::mem;
use std::ptr::{self, NonNull};

use crate::class_registry::ClassRegistry;
use crate::core::ConvertMut;
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::{ArgumentError, Fatal};
use crate::state::State;
use crate::sys;
use crate::Artichoke;

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
pub unsafe fn from_user_data(
    mrb: *mut sys::mrb_state,
) -> Result<Artichoke, InterpreterExtractError> {
    let mut mrb = if let Some(mrb) = NonNull::new(mrb) {
        mrb
    } else {
        error!("Attempted to extract Artichoke from null mrb_state");
        return Err(InterpreterExtractError);
    };
    let ud = mem::replace(mrb.as_mut().ud, ptr::null_mut());
    let state = if let Some(state) = NonNull::new(ud) {
        state.cast::<State>()
    } else {
        info!("Attempted to extract Artichoke from null mrb_state->ud pointer");
        return Err(InterpreterExtractError);
    };
    let state = Box::from_raw(state.as_ref());
    // At this point, `Rc::strong_count` will be increased by 1.
    trace!(
        "Extracted Artichoke from user data pointer on {}",
        sys::mrb_sys_state_debug(mrb.as_mut())
    );
    Ok(Artichoke { mrb, state })
}

/// Failed to extract Artichoke interpreter at an FFI boundary.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct InterpreterExtractError;

impl fmt::Display for InterpreterExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to extract Artichoke Ruby interpreter from mrb_state userdata"
        )
    }
}

impl error::Error for InterpreterExtractError {}

impl RubyException for InterpreterExtractError {
    fn message(&self) -> &[u8] {
        &b"Failed to extract Artichoke Ruby interpreter from mrb_state"[..]
    }

    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let spec = interp.class_spec::<Fatal>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<InterpreterExtractError> for Exception {
    fn from(exception: InterpreterExtractError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<InterpreterExtractError>> for Exception {
    fn from(exception: Box<InterpreterExtractError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<InterpreterExtractError> for Box<dyn RubyException> {
    fn from(exception: InterpreterExtractError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<InterpreterExtractError>> for Box<dyn RubyException> {
    fn from(exception: Box<InterpreterExtractError>) -> Box<dyn RubyException> {
        exception
    }
}

/// Convert a byte slice to a platform-specific [`OsStr`].
///
/// Unsupported platforms fallback to converting through `str`.
#[inline]
pub fn bytes_to_os_str(value: &[u8]) -> Result<&OsStr, ConvertBytesError> {
    value.to_os_str().map_err(|_| ConvertBytesError)
}

/// Convert a platform-specific [`OsStr`] to a byte slice.
///
/// Unsupported platforms fallback to converting through `str`.
#[inline]
pub fn os_str_to_bytes(value: &OsStr) -> Result<&[u8], ConvertBytesError> {
    <[u8]>::from_os_str(value).ok_or(ConvertBytesError)
}

/// Convert a platform-specific [`OsString`] to a byte vec.
///
/// Unsupported platforms fallback to converting through `String`.
#[inline]
pub fn os_string_to_bytes(value: OsString) -> Result<Vec<u8>, ConvertBytesError> {
    Vec::from_os_string(value).map_err(|_| ConvertBytesError)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConvertBytesError;

impl fmt::Display for ConvertBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not convert between bytes and platform string")
    }
}

impl error::Error for ConvertBytesError {}

impl RubyException for ConvertBytesError {
    fn message(&self) -> &[u8] {
        &b"invalid byte sequence"[..]
    }

    fn name(&self) -> String {
        String::from("ArgumentError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let spec = interp.class_spec::<ArgumentError>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<ConvertBytesError> for Exception {
    fn from(exception: ConvertBytesError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<ConvertBytesError>> for Exception {
    fn from(exception: Box<ConvertBytesError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<ConvertBytesError> for Box<dyn RubyException> {
    fn from(exception: ConvertBytesError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<ConvertBytesError>> for Box<dyn RubyException> {
    fn from(exception: Box<ConvertBytesError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use std::rc::Rc;

    use crate::ffi::InterpreterExtractError;

    #[test]
    fn from_user_data_null_pointer() {
        let err = unsafe { super::from_user_data(ptr::null_mut()) };
        assert_eq!(err.err(), Some(InterpreterExtractError));
    }

    #[test]
    fn from_user_data_null_user_data() {
        let interp = crate::interpreter().expect("init");
        unsafe {
            let mrb = interp.mrb.as_mut();
            // fake null user data
            (*mrb).ud = ptr::null_mut();
        }
        let err = unsafe { super::from_user_data(mrb) };
        assert_eq!(err.err(), Some(InterpreterExtractError));
    }

    #[test]
    fn from_user_data() {
        let interp = crate::interpreter().expect("init");
        let res = unsafe {
            let mrb = interp.mrb.as_mut();
            super::from_user_data(mrb)
        };
        assert!(res.is_ok());
    }
}
