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

pub use artichoke_core::class_registry::ClassRegistry as _;
pub use artichoke_core::coerce_to_numeric::CoerceToNumeric;
pub use artichoke_core::constant::DefineConstant as _;
pub use artichoke_core::convert::{Convert, ConvertMut, TryConvert, TryConvertMut};
pub use artichoke_core::debug::Debug as _;
pub use artichoke_core::encoding_registry::EncodingRegistry as _;
pub use artichoke_core::eval::Eval as _;
pub use artichoke_core::file::File;
pub use artichoke_core::globals::Globals as _;
pub use artichoke_core::intern::Intern as _;
pub use artichoke_core::io::Io as _;
pub use artichoke_core::load::LoadSources as _;
pub use artichoke_core::module_registry::ModuleRegistry as _;
pub use artichoke_core::parser::Parser as _;
pub use artichoke_core::prng::Prng as _;
pub use artichoke_core::regexp::Regexp as _;
pub use artichoke_core::types::Ruby;
pub use artichoke_core::value::Value as _;
pub use artichoke_core::warn::Warn as _;
pub use scolapasta_aref as aref;
pub use spinoso_exception::core::*;

pub use crate::block::Block;
pub use crate::class;
pub use crate::convert::{BoxUnboxVmValue, HeapAllocatedData};
pub use crate::def::{self, EnclosingRubyScope, NotDefinedError};
pub use crate::error::{self, Error, RubyException};
pub use crate::ffi::InterpreterExtractError;
pub use crate::gc::MrbGarbageCollection as _;
pub use crate::module;
pub use crate::sys;
pub use crate::value::Value;
pub use crate::{Artichoke, Guard};

/// Type alias for errors returned from `init` functions in
/// [`extn`](crate::extn).
pub type InitializeResult<T> = Result<T, Error>;
