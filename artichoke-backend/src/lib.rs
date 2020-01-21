#![deny(clippy::all)]
#![deny(clippy::pedantic)]
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
//! use artichoke_core::eval::Eval;
//! use artichoke_core::value::Value as _;
//!
//! let interp = artichoke_backend::interpreter().unwrap();
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

use std::borrow::Cow;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::ptr::NonNull;

#[macro_use]
#[doc(hidden)]
pub mod macros;

pub mod class;
pub mod convert;
pub mod def;
pub mod eval;
pub mod exception;
pub mod exception_handler;
pub mod extn;
pub mod ffi;
pub mod fs;
pub mod gc;
mod interpreter;
pub mod load;
pub mod method;
pub mod module;
pub mod state;
pub mod sys;
pub mod top_self;
pub mod types;
pub mod value;
pub mod warn;

#[cfg(test)]
mod test;

pub use artichoke_core::ArtichokeError;
pub use interpreter::interpreter;
pub use state::State;

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
pub struct Artichoke {
    state: Box<State>,
    mrb: NonNull<sys::mrb_state>,
}

impl Artichoke {
    /// Consume an interpreter and free all
    /// [live](gc::MrbGarbageCollection::live_object_count)
    /// [`Value`](value::Value)s.
    pub fn close(mut self) {
        self.state.close(&mut self.mrb);
        unsafe {
            sys::mrb_close(self.mrb.as_mut());
        }
    }

    pub fn state(&self) -> &State {
        self.state.as_ref()
    }

    pub fn state_mut(&mut self) -> &mut State {
        self.state.as_mut()
    }

    pub fn vfs(&self) -> &fs::Filesystem {
        self.state.vfs()
    }

    pub fn vfs_mut(&mut self) -> &mut fs::Filesystem {
        self.state.vfs_mut()
    }

    pub fn regexp_last_evaluation_captures_mut(&mut self) -> &mut usize {
        self.state.regexp_last_evaluation_captures_mut()
    }

    pub fn mrb_mut(&mut self) -> &mut sys::mrb_state {
        self.mrb.as_mut()
    }

    pub fn sym_intern<T>(&mut self, sym: T) -> sys::mrb_sym
    where
        T: Into<Cow<'static, [u8]>>,
    {
        self.state.sym_intern(self.mrb_mut(), sym)
    }

    pub unsafe fn into_user_data(mut self) -> *mut sys::mrb_state {
        let state = Box::into_raw(self.state);
        self.mrb.as_mut().ud = state as *mut std::ffi::c_void;
        self.mrb.as_ptr()
    }
}

impl Drop for Artichoke {
    fn drop(&mut self) {
        self.close();
    }
}

impl TryFrom<*mut sys::mrb_state> for Artichoke {
    type Error = ArtichokeError;

    fn try_from(mrb: *mut sys::mrb_state) -> Result<Self, Self::Error> {
        unsafe { ffi::from_user_data(mrb) }
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
    #[must_use]
    fn description(&self) -> &str {
        "Artichoke interpreter boot error"
    }

    #[must_use]
    fn cause(&self) -> Option<&dyn error::Error> {
        match self.0 {
            BootErrorType::Artichoke(ref err) => Some(err),
            BootErrorType::Ruby(ref exc) => Some(exc),
        }
    }
}

impl From<ArtichokeError> for BootError {
    #[must_use]
    fn from(err: ArtichokeError) -> Self {
        Self(BootErrorType::Artichoke(err))
    }
}

impl From<Exception> for BootError {
    #[must_use]
    fn from(err: Exception) -> Self {
        Self(BootErrorType::Ruby(err))
    }
}
