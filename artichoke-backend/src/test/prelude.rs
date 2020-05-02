//! A "prelude" for writing tests in the `artichoke-backend` crate.
//!
//! This prelude is similar to the standard library's prelude in that you'll
//! almost always want to import its entire contents, but unlike the standard
//! library's prelude, you'll have to do so manually:
//!
//! ```
//! use artichoke_backend::test::prelude::*;
//! ```
//!
//! The prelude may grow over time as additional items see ubiquitous use.

pub use crate::class;
pub use crate::convert::RustBackedValue;
pub use crate::core::{Value as _, *};
pub use crate::def::{self, EnclosingRubyScope, NotDefinedError};
pub use crate::exception;
pub use crate::module;
pub use crate::prelude::*;
pub use crate::state::parser::Context;
pub use crate::string;
pub use crate::sys;
pub use crate::types::{Fp, Int};
pub use crate::value::{Block, Value};
