#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Built in Ruby exception types.
//!
//! Descendants of class [`Exception`] are used to communicate between
//! [`Kernel#raise`] and `rescue` statements in `begin ... end` blocks.
//! Exception objects carry information about the exception – its type (the
//! exception's class name), an optional descriptive string, and optional
//! traceback information. `Exception` subclasses may add additional information
//! like [`NameError#name`].
//!
//! # Ruby Exception Hierarchy
//!
//! The built-in subclasses of [`Exception`] are:
//!
//! - [`NoMemoryError`]
//! - [`ScriptError`]
//!   - [`LoadError`]
//!   - [`NotImplementedError`]
//!   - [`SyntaxError`]
//! - [`SecurityError`]
//! - [`SignalException`]
//!   - [`Interrupt`]
//! - [`StandardError`] — default for `rescue`
//!   - [`ArgumentError`]
//!     - [`UncaughtThrowError`]
//!   - [`EncodingError`]
//!   - [`FiberError`]
//!   - [`IOError`]
//!     - [`EOFError`]
//!   - [`IndexError`]
//!     - [`KeyError`]
//!     - [`StopIteration`]
//!   - [`LocalJumpError`]
//!   - [`NameError`]
//!     - [`NoMethodError`]
//!   - [`RangeError`]
//!     - [`FloatDomainError`]
//!   - [`RegexpError`]
//!   - [`RuntimeError`] — default for `raise`
//!     - [`FrozenError`]
//!   - [`SystemCallError`]
//!     - `Errno::*`
//!   - [`ThreadError`]
//!   - [`TypeError`]
//!   - [`ZeroDivisionError`]
//! - [`SystemExit`]
//! - [`SystemStackError`]
//! - `fatal` — impossible to rescue
//!
//! # `no_std`
//!
//! This crate is `no_std` compatible when built without the `std` feature. This
//! crate has a required dependency on [`alloc`].
//!
//! # Crate features
//!
//! All features are enabled by default.
//!
//! - **std** - Enables a dependency on the Rust Standard Library. Activating
//!   this feature enables [`std::error::Error`] impls on error types in this
//!   crate.
//!
//! [`Exception`]: https://ruby-doc.org/core-2.6.3/Exception.html
//! [`Kernel#raise`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-raise
//! [`NameError#name`]: https://ruby-doc.org/core-2.6.3/NameError.html#method-i-name
//! [`NoMemoryError`]: https://ruby-doc.org/core-2.6.3/NoMemoryError.html
//! [`ScriptError`]: https://ruby-doc.org/core-2.6.3/ScriptError.html
//! [`LoadError`]: https://ruby-doc.org/core-2.6.3/LoadError.html
//! [`NotImplementedError`]: https://ruby-doc.org/core-2.6.3/NotImplementedError.html
//! [`SyntaxError`]: https://ruby-doc.org/core-2.6.3/SyntaxError.html
//! [`SecurityError`]: https://ruby-doc.org/core-2.6.3/SecurityError.html
//! [`SignalException`]: https://ruby-doc.org/core-2.6.3/SignalException.html
//! [`Interrupt`]: https://ruby-doc.org/core-2.6.3/Interrupt.html
//! [`StandardError`]: https://ruby-doc.org/core-2.6.3/StandardError.html
//! [`ArgumentError`]: https://ruby-doc.org/core-2.6.3/ArgumentError.html
//! [`UncaughtThrowError`]: https://ruby-doc.org/core-2.6.3/UncaughtThrowError.html
//! [`EncodingError`]: https://ruby-doc.org/core-2.6.3/EncodingError.html
//! [`FiberError`]: https://ruby-doc.org/core-2.6.3/FiberError.html
//! [`IOError`]: https://ruby-doc.org/core-2.6.3/IOError.html
//! [`EOFError`]: https://ruby-doc.org/core-2.6.3/EOFError.html
//! [`IndexError`]: https://ruby-doc.org/core-2.6.3/IndexError.html
//! [`KeyError`]: https://ruby-doc.org/core-2.6.3/KeyError.html
//! [`StopIteration`]: https://ruby-doc.org/core-2.6.3/StopIteration.html
//! [`LocalJumpError`]: https://ruby-doc.org/core-2.6.3/LocalJumpError.html
//! [`NameError`]: https://ruby-doc.org/core-2.6.3/NameError.html
//! [`NoMethodError`]: https://ruby-doc.org/core-2.6.3/NoMethodError.html
//! [`RangeError`]: https://ruby-doc.org/core-2.6.3/RangeError.html
//! [`FloatDomainError`]: https://ruby-doc.org/core-2.6.3/FloatDomainError.html
//! [`RegexpError`]: https://ruby-doc.org/core-2.6.3/RegexpError.html
//! [`RuntimeError`]: https://ruby-doc.org/core-2.6.3/RuntimeError.html
//! [`FrozenError`]: https://ruby-doc.org/core-2.6.3/FrozenError.html
//! [`SystemCallError`]: https://ruby-doc.org/core-2.6.3/SystemCallError.html
//! [`ThreadError`]: https://ruby-doc.org/core-2.6.3/ThreadError.html
//! [`TypeError`]: https://ruby-doc.org/core-2.6.3/TypeError.html
//! [`ZeroDivisionError`]: https://ruby-doc.org/core-2.6.3/ZeroDivisionError.html
//! [`SystemExit`]: https://ruby-doc.org/core-2.6.3/SystemExit.html
//! [`SystemStackError`]: https://ruby-doc.org/core-2.6.3/SystemStackError.html

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::borrow::Cow;

pub mod core;

#[doc(inline)]
pub use self::core::*;

/// Polymorphic exception type that corresponds to Ruby's `Exception`.
///
/// This trait unifies all concrete exception types defined in this crate and is
/// [object safe]. This means `RubyException` can be used as a trait object to
/// represent an error type of any set of exception subclasses.
///
/// All types that implement `RubyException` should be `raise`able in an
/// Artichoke Ruby VM.
///
/// # Examples
///
/// ```
/// # use spinoso_exception::*;
/// # struct Array(()); impl Array { pub fn is_frozen(&self) -> bool { true } }
/// fn array_concat(slf: Array, other: Array) -> Result<Array, Box<dyn RubyException>> {
///     if slf.is_frozen() {
///         return Err(Box::new(FrozenError::new()));
///     }
///     Err(Box::new(NotImplementedError::new()))
/// }
/// ```
///
/// [object safe]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
pub trait RubyException {
    /// The exception's message.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_exception::*;
    /// fn exception_inspect(exc: &dyn RubyException) {
    ///     let message = exc.message();
    ///     let message = String::from_utf8_lossy(&message);
    ///     println!("{} ({})", exc.name(), message);
    /// }
    /// ```
    ///
    /// # Implementation notes
    ///
    /// This method returns a byte slice since Ruby `String`s are best
    /// represented as a [`Vec<u8>`].
    ///
    /// [`Vec<u8>`]: alloc::vec::Vec
    fn message(&self) -> Cow<'_, [u8]>;

    /// The exception's class name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_exception::*;
    /// fn exception_inspect(exc: &dyn RubyException) {
    ///     let message = exc.message();
    ///     let message = String::from_utf8_lossy(&message);
    ///     println!("{} ({})", exc.name(), message);
    /// }
    /// ```
    fn name(&self) -> Cow<'_, str>;
}

// Assert that `RubyException` is object-safe (i.e. supports dynamic dispatch).
const _: Option<&dyn RubyException> = None;
