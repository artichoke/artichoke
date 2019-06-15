#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

//! # mruby
//!
//! mruby crate provides a safe interface over the raw mruby bindings in
//! [`mruby-sys`](mruby_sys). mruby crate aims to expose as much of the mruby API
//! as possible.
//!
//! ## Execute Ruby Code
//!
//! mruby crate exposes several mechanisms for executing Ruby code on the
//! interpreter.
//!
//! ### Evaling Source Code
//!
//! mruby crate exposes eval on the `mrb_state` with the [`MrbEval`](eval::MrbEval)
//! trait. Side effects from eval are persisted across invocations.
//!
//! ```rust
//! use mruby::eval::MrbEval;
//! use mruby::interpreter::Interpreter;
//!
//! let interp = Interpreter::create().unwrap();
//! let result = interp.eval("10 * 10").unwrap();
//! let result = result.try_into::<i64>();
//! assert_eq!(result, Ok(100));
//! ```
//!
//! ### Calling Ruby Functions from Rust
//!
//! The [`ValueLike`](value::ValueLike) trait exposes a _funcall interface_ which
//! can call Ruby functions on a [`Value`](value::Value) using a `String`
//! function name and a `Vec<Value>` or arguments. funcall takes a type parameter
//! bound by [`TryFromMrb`](convert::TryFromMrb) and converts the result of the function call to a Rust
//! type (which may be `Value` or another "native" type).
//!
//! mruby limits functions to a maximum of 16 arguments.
//!
//! ## Virtual Filesystem and `Kernel#require`
//!
//! The mruby [`State`](state::State) embeds an
//! [in-memory virtual Unix filesystem](mruby_vfs). The VFS stores Ruby sources
//! that are either pure Ruby, implemented with a Rust [`MrbFile`](file::MrbFile), or
//! both.
//!
//! mruby crate implements
//! [`Kernel#require` and `Kernel#require_relative`](extn::core::kernel::Kernel) which
//! loads sources from the VFS. For Ruby sources, the source is loaded from the VFS
//! as a `Vec<u8>` and evaled with [`MrbEval::eval_with_context`](eval::MrbEval::eval_with_context). For
//! Rust sources, [`MrbFile::require`](file::MrbFile::require) methods are stored as custom
//! metadata on [`File`](mruby_vfs::FakeFileSystem) nodes in the VFS.
//!
//! ```rust
//! use mruby::eval::MrbEval;
//! use mruby::interpreter::Interpreter;
//! use mruby::load::MrbLoadSources;
//!
//! let mut interp = Interpreter::create().unwrap();
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
//! The [`mrb_value`](sys::mrb_value) struct is a data type that represents a Ruby object. The
//! concrete type of an `mrb_value` is specified by its type tag, an [`mrb_vtype`](sys::mrb_vtype)
//! enum value.
//!
//! One `mrb_vtype` is `MRB_TT_DATA`, which allows an `mrb_value` to store an owned
//! `c_void` pointer. mruby crate leverages this to store an owned copy of an
//! `Rc<RefCell<T>>` for any `T` that implements [`RustBackedValue`](convert::RustBackedValue).
//!
//! [`RustBackedValue`](convert::RustBackedValue) provides two methods for working with
//! `MRB_TT_DATA`:
//!
//! - [`RustBackedValue::try_into_ruby`](convert::RustBackedValue::try_into_ruby) consumes `self` and returns a live
//!   `mrb_value` that wraps `T`.
//! - [`RustBackedValue::try_from_ruby`](convert::RustBackedValue::try_from_ruby) extracts an `Rc<RefCell<T>>` from an
//!   `mrb_value` and manages the strong count of the `Rc` smart pointer to ensure
//!   that the `mrb_value` continues to point to valid memory.
//!
//! These `mrb_value`s with type tag `MRB_TT_DATA` can be used to implement Ruby
//! `Class`es and `Module`s with Rust structs. An example of this is the
//! [`Regexp`](extn::core::regexp::Regexp) class which wraps an Oniguruma regex
//! provided by the [`onig`] crate.
//!
//! ```rust
//! use mruby::convert::{RustBackedValue, TryFromMrb};
//! use mruby::def::{rust_data_free, ClassLike, Define};
//! use mruby::eval::MrbEval;
//! use mruby::file::MrbFile;
//! use mruby::interpreter::{Interpreter, Mrb, MrbApi};
//! use mruby::load::MrbLoadSources;
//! use mruby::sys;
//! use mruby::value::Value;
//! use mruby::{interpreter_or_raise, unwrap_or_raise, unwrap_value_or_raise};
//! use mruby::MrbError;
//! use std::io::Write;
//! use std::mem;
//!
//! struct Container { inner: i64 }
//!
//! impl Container {
//!     unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, mut slf: sys::mrb_value) -> sys::mrb_value {
//!         let interp = interpreter_or_raise!(mrb);
//!         let api = interp.borrow();
//!         let int = mem::uninitialized::<sys::mrb_int>();
//!         let mut argspec = vec![];
//!         argspec.write_all(format!("{}\0", sys::specifiers::INTEGER).as_bytes()).unwrap();
//!         sys::mrb_get_args(mrb, argspec.as_ptr() as *const i8, &int);
//!         let cont = Self { inner: int };
//!         unwrap_value_or_raise!(interp, cont.try_into_ruby(&interp, Some(slf)))
//!     }
//!
//!     unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
//!         let interp = interpreter_or_raise!(mrb);
//!         let cont = unwrap_or_raise!(
//!             interp,
//!             Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
//!             interp.nil().inner()
//!         );
//!         let borrow = cont.borrow();
//!         interp.fixnum(borrow.inner).inner()
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
//! let mut interp = Interpreter::create().unwrap();
//! interp.def_file_for_type::<_, Container>("container.rb").unwrap();
//! interp.eval("require 'container'").unwrap();
//! let result = interp.eval("Container.new(15).value * 24").unwrap();
//! assert_eq!(result.try_into::<i64>(), Ok(360));
//! ```
//!
//! ## Converters Between Ruby and Rust Types
//!
//! The [`convert` module](convert) provides implementations for conversions
//! between `mrb_value` Ruby types and native Rust types like `i64` and
//! `HashMap<String, Option<Vec<u8>>>` using an [`Mrb`](interpreter::Mrb) interpreter.
//!
//! There are two converter traits:
//!
//! - [`FromMrb`](convert::FromMrb) provides infallible conversions that return
//!   `Self`. Converting from a Rust native type to a Ruby `mrb_value` is usually an
//!   infallible conversion.
//! - [`TryFromMrb`](convert::TryFromMrb) provides fallible conversions that return
//!   `Result<Self, Error>`. Converting from a Ruby `mrb_value` to a Rust native
//!   type is always an fallible conversion because an `mrb_value` may be any type
//!   tag.
//!
//! Supported conversions:
//!
//! - Ruby _primitive types_ to Rust types. Primitive Ruby types are `TrueClass`,
//!   `FalseClass`, `String` (both UTF-8 and binary), `Fixnum` (`i64`), `Float`
//!   (`f64`).
//! - Rust types to Ruby types. Supported Rust types are `bool`, `Vec<u8>`, `&[u8]`,
//!   integer types that losslessly convert to `i64` (`i64`, `i32`, `i16`, `i8`,
//!   `u32`, `u16`, `u8`), `f64`, `String`, `&str`.
//! - Ruby `nil`able types to Rust `Option<T>`.
//! - Rust `Option<T>` types to Ruby `nil` or an `mrb_value` converted from `T`.
//! - Ruby `Array` to Rust `Vec<T>` where `T` corresponds to a Ruby _primitive
//!   type_.
//! - Rust `Vec<T>` to Ruby `Array` where `T` corresponds to a Ruby _primitive
//!   type_.
//! - Ruby `Hash` to Rust `Vec<(Key, Value)>` or `HashMap<Key, Value>` where `Key`
//!   and `Value` correspond to Ruby _primitive types_.
//! - Rust `Vec<(Key, Value)>` or `HashMap<Key, Value>` to Ruby `Hash` where `Key`
//!   and `Value` correspond to Ruby _primitive types_.
//! - Identity conversion from `Value` to `Value`, which is useful when working with
//!   collection types.
//!
//! The infallible converters are safe Rust functions. The fallibile converters are
//! `unsafe` Rust functions.

use std::error;
use std::fmt;
use std::io;

#[macro_use]
#[doc(hidden)]
pub mod macros;

pub mod class;
pub mod convert;
pub mod def;
pub mod eval;
pub mod exception;
pub mod extn;
pub mod file;
pub mod gc;
pub mod interpreter;
pub mod load;
pub mod method;
pub mod module;
pub mod state;
pub mod value;

pub use mruby_sys as sys;

#[derive(Debug)]
pub enum MrbError {
    ArgSpec,
    ConvertToRuby(convert::Error<value::types::Rust, value::types::Ruby>),
    ConvertToRust(convert::Error<value::types::Ruby, value::types::Rust>),
    // TODO: this should really be an `Exception` instead of a `String`.
    Exec(String),
    New,
    NotDefined(String),
    SourceNotFound(String),
    TooManyArgs { given: usize, max: usize },
    Uninitialized,
    UnreachableValue(sys::mrb_vtype),
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
