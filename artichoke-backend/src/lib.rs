#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

//! # artichoke-backend
//!
//! artichoke-backend crate provides a Ruby interpreter. It currently is
//! implemented with [`mruby-sys`](mruby_sys).
//!
//! ## Execute Ruby Code
//!
//! artichoke-backend crate exposes several mechanisms for executing Ruby code
//! on the interpreter.
//!
//! ### Evaling Source Code
//!
//! artichoke-backend crate exposes eval on the `State` with the
//! [`MrbEval`](eval::MrbEval) trait. Side effects from eval are persisted
//! across invocations.
//!
//! ```rust
//! use artichoke_backend::eval::MrbEval;
//!
//! let interp = artichoke_backend::interpreter().unwrap();
//! let result = interp.eval("10 * 10").unwrap();
//! let result = result.try_into::<i64>();
//! assert_eq!(result, Ok(100));
//! ```
//!
//! ### Calling Ruby Functions from Rust
//!
//! The [`ValueLike`](value::ValueLike) trait exposes a _funcall interface_
//! which can call Ruby functions on a [`Value`](value::Value) using a `String`
//! function name and a `Vec<Value>` or arguments. funcall takes a type
//! parameter bound by [`TryConvert`](convert::TryConvert) and converts the
//! result of the function call to a Rust type (which may be `Value` or another
//! "native" type).
//!
//! artichoke-backend limits functions to a maximum of 16 arguments.
//!
//! ## Virtual Filesystem and `Kernel#require`
//!
//! The artichoke-backend [`State`](state::State) embeds an
//! [in-memory virtual Unix filesystem](artichoke_vfs). The VFS stores Ruby
//! sources that are either pure Ruby, implemented with a Rust
//! [`MrbFile`](file::MrbFile), or both.
//!
//! artichoke-backend crate implements
//! [`Kernel#require` and `Kernel#require_relative`](extn::core::kernel::Kernel)
//! which loads sources from the VFS. For Ruby sources, the source is loaded
//! from the VFS as a `Vec<u8>` and evaled with
//! [`MrbEval::eval_with_context`](eval::MrbEval::eval_with_context). For Rust
//! sources, [`MrbFile::require`](file::MrbFile::require) methods are stored as
//! custom metadata on [`File`](artichoke_vfs::FakeFileSystem) nodes in the VFS.
//!
//! ```rust
//! use artichoke_backend::eval::MrbEval;
//! use artichoke_backend::load::MrbLoadSources;
//!
//! let mut interp = artichoke_backend::interpreter().unwrap();
//! let code = "
//! def source_location
//!   __FILE__
//! end
//! ";
//! interp.def_rb_source_file("source.rb", code).unwrap();
//! interp.eval("require 'source'").unwrap();
//! let result = interp.eval("source_location").unwrap();
//! let result = result.try_into::<String>().unwrap();
//! assert_eq!(&result, "/src/lib/source.rb");
//! ```
//!
//! ## Embed Rust Objects in `mrb_value`
//!
//! The [`mrb_value`](sys::mrb_value) struct is a data type that represents a
//! Ruby object. The concrete type of an `mrb_value` is specified by its type
//! tag, an [`mrb_vtype`](sys::mrb_vtype) enum value.
//!
//! One `mrb_vtype` is `MRB_TT_DATA`, which allows an `mrb_value` to store an
//! owned `c_void` pointer. artichoke-backend crate leverages this to store an
//! owned copy of an `Rc<RefCell<T>>` for any `T` that implements
//! [`RustBackedValue`](convert::RustBackedValue).
//!
//! [`RustBackedValue`](convert::RustBackedValue) provides two methods for working with
//! `MRB_TT_DATA`:
//!
//! - [`RustBackedValue::try_into_ruby`](convert::RustBackedValue::try_into_ruby)
//!   consumes `self` and returns a live
//!   `mrb_value` that wraps `T`.
//! - [`RustBackedValue::try_from_ruby`](convert::RustBackedValue::try_from_ruby)
//!   extracts an `Rc<RefCell<T>>` from an `mrb_value` and manages the strong
//!   count of the `Rc` smart pointer to ensure that the `mrb_value` continues
//!   to point to valid memory.
//!
//! These `mrb_value`s with type tag `MRB_TT_DATA` can be used to implement Ruby
//! `Class`es and `Module`s with Rust structs. An example of this is the
//! [`Regexp`](extn::core::regexp::Regexp) class which wraps an Oniguruma regex
//! provided by the [`onig`] crate.
//!
//! ```rust
//! #[macro_use]
//! extern crate artichoke_backend;
//!
//! use artichoke_backend::convert::{Convert, RustBackedValue, TryConvert};
//! use artichoke_backend::def::{rust_data_free, ClassLike, Define};
//! use artichoke_backend::eval::MrbEval;
//! use artichoke_backend::file::MrbFile;
//! use artichoke_backend::load::MrbLoadSources;
//! use artichoke_backend::sys;
//! use artichoke_backend::value::Value;
//! use artichoke_backend::{Mrb, MrbError};
//! use std::io::Write;
//! use std::mem;
//!
//! struct Container { inner: i64 }
//!
//! impl Container {
//!     unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, mut slf: sys::mrb_value) -> sys::mrb_value {
//!         let interp = unwrap_interpreter!(mrb);
//!         let api = interp.borrow();
//!         let int = mem::uninitialized::<sys::mrb_int>();
//!         let mut argspec = vec![];
//!         argspec.write_all(format!("{}\0", sys::specifiers::INTEGER).as_bytes()).unwrap();
//!         sys::mrb_get_args(mrb, argspec.as_ptr() as *const i8, &int);
//!         let cont = Self { inner: int };
//!         cont
//!             .try_into_ruby(&interp, Some(slf))
//!             .unwrap_or_else(|_| Value::from_mrb(&interp, None::<Value>))
//!             .inner()
//!     }
//!
//!     unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
//!         let interp = unwrap_interpreter!(mrb);
//!         if let Ok(cont) = Self::try_from_ruby(&interp, &Value::new(&interp, slf)) {
//!             let borrow = cont.borrow();
//!             Value::from_mrb(&interp, borrow.inner).inner()
//!         } else {
//!             Value::from_mrb(&interp, None::<Value>).inner()
//!         }
//!     }
//! }
//!
//! impl RustBackedValue for Container {}
//!
//! impl MrbFile for Container {
//!   fn require(interp: Mrb) -> Result<(), MrbError> {
//!         let spec = interp.borrow_mut().def_class::<Self>("Container", None, Some(rust_data_free::<Self>));
//!         spec.borrow_mut().add_method("initialize", Self::initialize, sys::mrb_args_req(1));
//!         spec.borrow_mut().add_method("value", Self::value, sys::mrb_args_none());
//!         spec.borrow_mut().mrb_value_is_rust_backed(true);
//!         spec.borrow().define(&interp)?;
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     let interp = artichoke_backend::interpreter().unwrap();
//!     interp.def_file_for_type::<_, Container>("container.rb").unwrap();
//!     interp.eval("require 'container'").unwrap();
//!     let result = interp.eval("Container.new(15).value * 24").unwrap();
//!     assert_eq!(result.try_into::<i64>(), Ok(360));
//! }
//! ```
//!
//! ## Converters Between Ruby and Rust Types
//!
//! The [`convert` module](convert) provides implementations for conversions
//! between `mrb_value` Ruby types and native Rust types like `i64` and
//! `HashMap<String, Option<Vec<u8>>>` using an [`Mrb`](interpreter::Mrb)
//! interpreter.
//!
//! There are two converter traits:
//!
//! - [`Convert`](convert::Convert) provides infallible conversions that return
//!   `Self`. Converting from a Rust native type to a Ruby `mrb_value` is
//!   usually an infallible conversion.
//! - [`TryConvert`](convert::TryConvert) provides fallible conversions that
//!   return `Result<Self, Error>`. Converting from a Ruby `mrb_value` to a Rust
//!   native type is always an fallible conversion because an `mrb_value` may be
//!   any type tag.
//!
//! Supported conversions:
//!
//! - Ruby _primitive types_ to Rust types. Primitive Ruby types are
//!   `TrueClass`, `FalseClass`, `String` (both UTF-8 and binary), `Fixnum`
//!   (`i64`), `Float` (`f64`).
//! - Rust types to Ruby types. Supported Rust types are `bool`, `Vec<u8>`,
//!   `&[u8]`, integer types that losslessly convert to `i64` (`i64`, `i32`,
//!   `i16`, `i8`, `u32`, `u16`, `u8`), `f64`, `String`, `&str`.
//! - Ruby `nil`able types to Rust `Option<T>`.
//! - Rust `Option<T>` types to Ruby `nil` or an `mrb_value` converted from `T`.
//! - Ruby `Array` to Rust `Vec<T>` where `T` corresponds to a Ruby _primitive
//!   type_.
//! - Rust `Vec<T>` to Ruby `Array` where `T` corresponds to a Ruby _primitive
//!   type_.
//! - Ruby `Hash` to Rust `Vec<(Key, Value)>` or `HashMap<Key, Value>` where
//!   `Key` and `Value` correspond to Ruby _primitive types_.
//! - Rust `Vec<(Key, Value)>` or `HashMap<Key, Value>` to Ruby `Hash` where
//!   `Key` and `Value` correspond to Ruby _primitive types_.
//! - Identity conversion from `Value` to `Value`, which is useful when working
//!   with collection types.
//!
//! The infallible converters are safe Rust functions. The fallibile converters are
//! `unsafe` Rust functions.

use std::cell::RefCell;
use std::error;
use std::fmt;
use std::io;
use std::rc::Rc;

#[macro_use]
#[doc(hidden)]
pub mod macros;

pub mod class;
pub mod convert;
pub mod def;
pub mod eval;
pub mod exception;
pub mod extn;
pub mod ffi;
pub mod file;
pub mod fs;
pub mod gc;
mod interpreter;
pub mod load;
pub mod method;
pub mod module;
pub mod state;
pub mod top_self;
pub mod value;
pub mod warn;

pub use interpreter::interpreter;
/// Re-exported bindings from [`mruby_sys`].
///
/// Useful for referring to [`mruby_sys`] from macros defined in
/// artichoke-backend crate.
pub use mruby_sys as sys;

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
/// [garbage collection](gc::MrbGarbageCollection) or [eval](eval::MrbEval).
pub type Mrb = Rc<RefCell<state::State>>;

/// Errors returned by artichoke-backend crate.
#[derive(Debug)]
pub enum MrbError {
    /// Failed to create an [argspec](sys::args) `CString`.
    ArgSpec,
    /// Failed to convert from a Rust type to a [`sys::mrb_value`].
    ConvertToRuby(convert::Error<value::types::Rust, value::types::Ruby>),
    /// Failed to convert from a [`sys::mrb_value`] to a Rust type.
    ConvertToRust(convert::Error<value::types::Ruby, value::types::Rust>),
    /// Exception raised during eval.
    ///
    /// See [`MrbEval`](eval::MrbEval).
    // TODO: wrap an `Exception` instead of a `String`, see GH-152.
    Exec(String),
    /// Unable to initalize interpreter.
    ///
    /// See [`sys::mrb_open`], [`interpreter`](interpreter::interpreter).
    New,
    /// Class or module with this name is not defined in the artichoke VM.
    NotDefined(String),
    /// Unable to load Ruby source file with this path from the embedded
    /// sources.
    ///
    /// See [`rust_embed`](https://docs.rs/rust-embed/).
    SourceNotFound(String),
    /// Arg count exceeds maximum allowed by artichoke.
    ///
    /// Affects [`sys::mrb_funcall`], [`sys::mrb_funcall_argv`],
    /// [`sys::mrb_funcall_with_block`], [`sys::mrb_yield`], and
    /// [`sys::mrb_yield_argv`].
    TooManyArgs { given: usize, max: usize },
    /// Attempted to extract an [`Mrb`] from a [`sys::mrb_state`] but could not.
    Uninitialized,
    /// Eval or funcall returned an interpreter-internal value.
    ///
    /// See [`Value::is_unreachable`](value::Value::is_unreachable).
    UnreachableValue(sys::mrb_vtype),
    /// [`io::Error`] when interacting with virtual filesystem.
    ///
    /// See [`artichoke_vfs`].
    Vfs(io::Error),
}

impl Eq for MrbError {}

impl PartialEq for MrbError {
    fn eq(&self, other: &Self) -> bool {
        // this is a hack because io::Error does not impl PartialEq
        format!("{}", self) == format!("{}", other)
    }
}

impl fmt::Display for MrbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MrbError::ArgSpec => write!(f, "could not generate argspec"),
            MrbError::ConvertToRuby(inner) => write!(f, "conversion error: {}", inner),
            MrbError::ConvertToRust(inner) => write!(f, "conversion error: {}", inner),
            MrbError::Exec(backtrace) => write!(f, "mruby exception: {}", backtrace),
            MrbError::New => write!(f, "failed to create mrb interpreter"),
            MrbError::NotDefined(fqname) => write!(f, "{} not defined", fqname),
            MrbError::SourceNotFound(source) => write!(f, "Could not load Ruby source {}", source),
            MrbError::TooManyArgs { given, max } => write!(
                f,
                "Too many args for funcall. Gave {}, but max is {}",
                given, max
            ),
            MrbError::Uninitialized => write!(f, "mrb interpreter not initialized"),
            MrbError::UnreachableValue(tt) => {
                write!(f, "extracted unreachable type {:?} from interpreter", tt)
            }
            MrbError::Vfs(err) => write!(f, "mrb vfs io error: {}", err),
        }
    }
}

impl error::Error for MrbError {
    fn description(&self) -> &str {
        "mruby interpreter error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            MrbError::ConvertToRuby(inner) => Some(inner),
            MrbError::ConvertToRust(inner) => Some(inner),
            MrbError::Vfs(inner) => Some(inner),
            _ => None,
        }
    }
}
