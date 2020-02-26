#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(rust_2018_idioms)]

//! # artichoke-backend
//!
//! `artichoke-backend` crate provides a Ruby interpreter. It currently is
//! implemented with [mruby](https://github.com/mruby/mruby) bindings exported
//! by the [`sys`] module.
//!
//! ## Execute Ruby Code
//!
//! `artichoke-backend` crate exposes several mechanisms for executing Ruby code
//! on the interpreter.
//!
//! ### Evaling Source Code
//!
//! The `artichoke-backend` interpreter implements
//! [`Eval` from `artichoke-core`](crate::Eval).
//!
//! ```rust
//! use artichoke_backend::{Eval, ValueLike};
//!
//! let mut interp = artichoke_backend::interpreter().unwrap();
//! let result = interp.eval(b"10 * 10").unwrap();
//! let result = result.try_into::<i64>().unwrap();
//! assert_eq!(result, 100);
//! ```
//!
//! ### Calling Functions on Ruby Objects
//!
//! [`Value`](value::Value)s returned by the `artichoke-backend` interpreter
//! implement [`Value` from `artichoke-core`](crate::ValueLike), which enables
//! calling Ruby functions from Rust.
//!
//! ```rust
//! use artichoke_backend::{Eval, ValueLike};
//!
//! let mut interp = artichoke_backend::interpreter().unwrap();
//! let result = interp.eval(b"'ruby funcall'").unwrap();
//! let result = result.funcall::<usize>("length", &[], None).unwrap();
//! assert_eq!(result, 12);
//! ```
//!
//! ## Virtual Filesystem and `Kernel#require`
//!
//! The `artichoke-backend` interpreter includes an in-memory virtual
//! filesystem.  The filesystem stores Ruby sources and Rust extension functions
//! that are similar to MRI C extensions.
//!
//! The virtual filesystem enables applications built with `artichoke-backend`
//! to `require` sources that are embedded in the binary without host filesystem
//! access.
//!
//! ## Embed Rust Types in Ruby `Value`s
//!
//! `artichoke-backend` exposes a concept similar to `data`-typed values in MRI
//! and mruby.
//!
//! When Rust types implement a special trait, they can be embedded in a Ruby
//! [`Value`](value::Value) and passed through the Ruby VM as a Ruby object.
//! Classes defined in this way can define methods in Rust or Ruby.
//!
//! Examples of these types include:
//!
//! - `Regexp` and `MatchData`, which are backed by regular expressions from the
//!   `onig` and `regex` crates.
//! - `ENV`, which glues Ruby to an environ backend.
//!
//! ## Converters Between Ruby and Rust Types
//!
//! The [`convert` module](convert) provides implementations for conversions
//! between boxed Ruby values and native Rust types like `i64` and
//! `HashMap<String, Option<Vec<u8>>>` using an `artichoke-backend` interpreter.
//!
//! ## License
//!
//! artichoke-backend is licensed with the MIT License (c) Ryan Lopopolo.
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
use std::rc::Rc;

#[macro_use]
#[doc(hidden)]
pub mod macros;

pub mod class;
mod constant;
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
mod io;
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

pub use artichoke_core::constant::DefineConstant;
pub use artichoke_core::convert::Convert;
pub use artichoke_core::convert::ConvertMut;
pub use artichoke_core::convert::TryConvert;
pub use artichoke_core::convert::TryConvertMut;
pub use artichoke_core::eval::Eval;
pub use artichoke_core::file::File;
pub use artichoke_core::intern::Intern;
pub use artichoke_core::load::LoadSources;
pub use artichoke_core::parser::Parser;
pub use artichoke_core::top_self::TopSelf;
pub use artichoke_core::value::Value as ValueLike;
pub use artichoke_core::warn::Warn;

pub use interpreter::interpreter;

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
