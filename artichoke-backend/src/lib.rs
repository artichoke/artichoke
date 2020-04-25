#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![warn(intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

//! # artichoke-backend
//!
//! `artichoke-backend` crate provides a Ruby interpreter. It is currently
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
//! [`Eval` from `artichoke-core`](crate::prelude::core::Eval).
//!
//! ```rust
//! use artichoke_backend::prelude::core::*;
//! use artichoke_backend::prelude::*;
//!
//! let mut interp = artichoke_backend::interpreter().unwrap();
//! let result = interp.eval(b"10 * 10").unwrap();
//! let result = result.try_into::<i64>(&interp).unwrap();
//! assert_eq!(result, 100);
//! ```
//!
//! ### Calling Functions on Ruby Objects
//!
//! [`Value`](value::Value)s returned by the `artichoke-backend` interpreter
//! implement [`Value` from `artichoke-core`](crate::core::prelude::Value),
//! which enables calling Ruby functions from Rust.
//!
//! ```rust
//! use artichoke_backend::prelude::core::*;
//! use artichoke_backend::prelude::*;
//!
//! let mut interp = artichoke_backend::interpreter().unwrap();
//! let result = interp.eval(b"'ruby funcall'").unwrap();
//! let result = result.funcall::<usize>(&mut interp, "length", &[], None).unwrap();
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

#![doc(html_root_url = "https://artichoke.github.io/artichoke/artichoke_backend")]

#[macro_use]
extern crate log;

use std::ptr::NonNull;

#[macro_use]
#[doc(hidden)]
pub mod macros;

pub mod class;
pub mod class_registry;
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
mod globals;
mod intern;
mod interpreter;
mod io;
mod load;
pub mod method;
pub mod module;
pub mod module_registry;
mod parser;
#[cfg(feature = "core-random")]
mod prng;
mod regexp;
pub mod state;
pub mod string;
pub mod sys;
mod top_self;
pub mod types;
pub mod value;
mod warn;

#[cfg(test)]
mod test;

pub use crate::interpreter::interpreter;
pub use artichoke_core::prelude as core;

/// A "prelude" for users of the `artichoke-backend` crate.
///
/// This prelude is similar to the standard library's prelude in that you'll
/// almost always want to import its entire contents, but unlike the standard
/// library's prelude, you'll have to do so manually:
///
/// ```
/// use artichoke_backend::prelude::*;
/// ```
///
/// The prelude may grow over time as additional items see ubiquitous use.
pub mod prelude {
    pub use crate::core;

    pub use crate::exception::{raise, Exception, RubyException};
    pub use crate::extn::core::exception::{Exception as _, *};
    pub use crate::gc::MrbGarbageCollection;
    pub use crate::interpreter::interpreter;
    pub use crate::Artichoke;
}

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
#[derive(Debug)]
pub struct Artichoke {
    pub mrb: NonNull<sys::mrb_state>,
    pub state: Box<state::State>,
}

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
        let mrb = interp.mrb.as_mut();
        mrb.ud = Box::into_raw(interp.state);
        drop(interp);
        mrb
    }

    /// Consume an interpreter and free all
    /// [live](gc::MrbGarbageCollection::live_object_count)
    /// [`Value`](value::Value)s.
    pub fn close(mut self) {
        let mrb = unsafe { self.mrb.as_mut() };
        self.state.close(mrb);
        unsafe {
            sys::mrb_close(mrb);
        }
    }
}
