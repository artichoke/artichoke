//! A "prelude" for users of the `extn` module in the `artichoke-backend`
//! crate.
//!
//! This prelude is similar to the standard library's prelude in that you'll
//! almost always want to import its entire contents, but unlike the standard
//! library's prelude, you'll have to do so manually:
//!
//! ```
//! use artichoke_backend::extn::prelude::*;
//! ```
//!
//! This prelude is most useful to include when developing functionality in the
//! Artichoke standard library.
//!
//! The prelude may grow over time as additional items see ubiquitous use.

pub use crate::block::Block;
pub use crate::class;
pub use crate::convert::{BoxUnboxVmValue, HeapAllocatedData};
pub use crate::core::{Regexp as _, Value as _, *};
pub use crate::def::{self, EnclosingRubyScope, NotDefinedError};
pub use crate::ffi::InterpreterExtractError;
pub use crate::module;
pub use crate::prelude::*;
pub use crate::string::{format_unicode_debug_into, WriteError};
pub use crate::sys;
pub use crate::value::Value;
pub use scolapasta_aref as aref;

/// Type alias for errors returned from `init` functions in
/// [`extn`](crate::extn).
pub type InitializeResult<T> = Result<T, Error>;
