//! Get and set global variables on an interpreter.
//!
//! Global variables can be an arbitrary Ruby value. Variable names must start
//! with `$`.

use alloc::borrow::Cow;

use crate::value::Value;

/// Get and set global variables on an interpreter.
///
/// Global variables can be an arbitrary Ruby value. Variable names must start
/// with `$`.
pub trait Globals {
    /// Concrete value type for global variables.
    type Value: Value;

    /// Concrete error type for failures manipulating global variables.
    type Error;

    /// Set global variable pointed to by `name` to the given Ruby value.
    ///
    /// # Errors
    ///
    /// If the name is not a valid global name, an error is returned.
    ///
    /// If there is insufficient capacity in the global storage, an error is
    /// returned.
    fn set_global_variable<T>(&mut self, name: T, value: &Self::Value) -> Result<(), Self::Error>
    where
        T: Into<Cow<'static, [u8]>>;

    /// Unset global variable pointed to by `name`.
    ///
    /// Unsetting a global variable removes the name from the global storage
    /// table. Unset globals resolve to `nil` in the Ruby VM.
    ///
    /// Unsetting a global that is currently unset is a no-op.
    ///
    /// # Errors
    ///
    /// If the name is not a valid global name, an error is returned.
    fn unset_global_variable<T>(&mut self, name: T) -> Result<(), Self::Error>
    where
        T: Into<Cow<'static, [u8]>>;

    /// Get the Ruby value stored in the global variable pointed to by `name`.
    ///
    /// # Compatibility Notes
    ///
    /// Getting a global that is currently may return `Ok(None)` even through
    /// a non-existent global resolves to `nil` in the Ruby VM. Consult the
    /// documentation on implementations of this trait for implementation-defined
    /// behavior.
    ///
    /// # Errors
    ///
    /// If the name is not a valid global name, an error is returned.
    fn get_global_variable<T>(&mut self, name: T) -> Result<Option<Self::Value>, Self::Error>
    where
        T: Into<Cow<'static, [u8]>>;
}
