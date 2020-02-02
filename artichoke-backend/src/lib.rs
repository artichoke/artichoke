#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

//! # artichoke-backend
//!
//! artichoke-backend crate provides a Ruby interpreter. It currently is implemented
//! with [mruby](https://github.com/mruby/mruby) bindings exported by the
//! [`sys`] module.
//!
//! ## Execute Ruby Code
//!
//! artichoke-backend crate exposes several mechanisms for executing Ruby code on
//! the interpreter.
//!
//! ### Evaling Source Code
//!
//! artichoke-backend crate exposes eval on the `State` with the `Eval` trait. Side
//! effects from eval are persisted across invocations.
//!
//! ```rust
//! use artichoke_backend::{Eval, ValueLike};
//!
//! let mut interp = artichoke_backend::interpreter().unwrap();
//! let result = interp.eval(b"10 * 10").unwrap();
//! let result = result.try_into::<i64>();
//! assert_eq!(result, Ok(100));
//! ```
//!
//! ## Virtual Filesystem and `Kernel#require`
//!
//! The artichoke-backend `State` embeds an
//! [in-memory virtual Unix filesystem](/artichoke-vfs). The VFS stores Ruby sources
//! that are either pure Ruby, implemented with a Rust `File`, or both.
//!
//! artichoke-backend crate implements
//! [`Kernel#require` and `Kernel#require_relative`](src/extn/core/kernel) which
//! loads sources from the VFS. For Ruby sources, the source is loaded from the VFS
//! as a `Vec<u8>` and evaled with `Eval::eval_with_context`. For Rust sources,
//! `File::require` methods are stored as custom metadata on `File` nodes in the
//! VFS.
//!
//! ## Embed Rust Types in Ruby `Value`s
//!
//! Rust types that implement `RustBackedValue` can be injected into the interpreter
//! as the backend for a Ruby object.
//!
//! Examples of `RustBackedValues` include:
//!
//! - `Regexp` and `MatchData`, which are backed by regular expressions from the
//!   `onig` and `regex` crates.
//! - `ENV` which glues Ruby to an environ backend.
//!
//! ## Converters Between Ruby and Rust Types
//!
//! The [`convert`] module provides implementations for conversions
//! between boxed Ruby values and native Rust types like `i64` and
//! `HashMap<String, Option<Vec<u8>>>` using an `Artichoke` interpreter.
//!
//! ## License
//!
//! artichoke-backend is licensed with the [MIT License](/LICENSE) (c) Ryan
//! Lopopolo.
//!
//! Some portions of artichoke-backend are derived from
//! [mruby](https://github.com/mruby/mruby) which is Copyright (c) 2019 mruby
//! developers. mruby is licensed with the
//! [MIT License](https://github.com/mruby/mruby/blob/master/LICENSE).
//!
//! Some portions of artichoke-backend are derived from Ruby @
//! [2.6.3](https://github.com/ruby/ruby/tree/v2_6_3) which is copyright Yukihiro
//! Matsumoto \<matz@netlab.jp\>. Ruby is licensed with the
//! [2-clause BSDL License](https://github.com/ruby/ruby/blob/v2_6_3/COPYING).
//!
//! artichoke-backend vendors headers provided by
//! [emsdk](https://github.com/emscripten-core/emsdk) which is Copyright (c) 2018
//! Emscripten authors. emsdk is licensed with the
//! [MIT/Expat License](https://github.com/emscripten-core/emsdk/blob/master/LICENSE).

#[macro_use]
extern crate downcast;
#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::error;
use std::fmt;
use std::rc::Rc;

#[macro_use]
#[doc(hidden)]
pub mod macros;

pub mod class;
pub mod convert;
pub mod def;
mod eval;
pub mod exception;
pub mod exception_handler;
pub mod extn;
pub mod ffi;
pub mod fs;
pub mod gc;
mod intern;
mod interpreter;
mod load;
pub mod method;
pub mod module;
mod parser;
pub mod state;
pub mod string;
pub mod sys;
mod top_self;
pub mod types;
pub mod value;
mod warn;

#[cfg(test)]
mod test;

pub use artichoke_core as core;

/// Re-export from [`artichoke_core`](artichoke_core::convert::Convert).
pub use crate::core::convert::Convert;
/// Re-export from [`artichoke_core`](artichoke_core::convert::ConvertMut).
pub use crate::core::convert::ConvertMut;
/// Re-export from [`artichoke_core`](artichoke_core::convert::TryConvert).
pub use crate::core::convert::TryConvert;
/// Re-export from [`artichoke_core`](artichoke_core::convert::TryConvertMut).
pub use crate::core::convert::TryConvertMut;

pub use artichoke_core::eval::Eval;
pub use artichoke_core::file::File;
pub use artichoke_core::intern::Intern;
pub use artichoke_core::load::LoadSources;
pub use artichoke_core::parser::Parser;
pub use artichoke_core::top_self::TopSelf;
pub use artichoke_core::value::Value as ValueLike;
pub use artichoke_core::warn::Warn;
pub use artichoke_core::ArtichokeError;

pub use interpreter::interpreter;

use crate::exception::Exception;

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
#[derive(Debug, Clone)]
pub struct Artichoke(pub Rc<RefCell<state::State>>); // TODO: this should not be pub

impl Artichoke {
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
    /// do this is to call `ffi::from_user_data` with the returned pointer and
    /// then call `Artichoke::close`.
    #[must_use]
    pub unsafe fn into_raw(interp: Self) -> *mut sys::mrb_state {
        let mrb = interp.0.borrow_mut().mrb;
        drop(interp);
        mrb
    }

    /// Consume an interpreter and free all
    /// [live](gc::MrbGarbageCollection::live_object_count)
    /// [`Value`](value::Value)s.
    pub fn close(self) {
        self.0.borrow_mut().close();
    }
}

/// Error returned when initializing an [`Artichoke`] interpreter.
///
/// This error type allows static errors as well as dynamic errors raised on the
/// Ruby interpreter.
#[derive(Debug)]
pub struct BootError(BootErrorType);

#[derive(Debug)]
enum BootErrorType {
    Artichoke(ArtichokeError),
    Ruby(Exception),
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            BootErrorType::Artichoke(ref err) => write!(f, "{}", err),
            BootErrorType::Ruby(ref exc) => write!(f, "{}", exc),
        }
    }
}

impl error::Error for BootError {
    fn description(&self) -> &str {
        "Artichoke interpreter boot error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self.0 {
            BootErrorType::Artichoke(ref err) => Some(err),
            BootErrorType::Ruby(ref exc) => Some(exc),
        }
    }
}

impl From<ArtichokeError> for BootError {
    fn from(err: ArtichokeError) -> Self {
        Self(BootErrorType::Artichoke(err))
    }
}

impl From<Exception> for BootError {
    fn from(err: Exception) -> Self {
        Self(BootErrorType::Ruby(err))
    }
}
