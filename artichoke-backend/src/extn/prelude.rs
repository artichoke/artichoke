//! Prelude for [`extn`](crate::extn) development.
//!
//! Include this module in a source with:
//!
//! ```rust
//! use crate::extn::prelude::*;
//! ```

pub use artichoke_core::eval::Eval;
pub use artichoke_core::load::LoadSources;
pub use artichoke_core::value::Value as ValueLike;
pub use artichoke_core::warn::Warn;

pub use crate::class;
pub use crate::convert::{Convert, RustBackedValue, TryConvert};
pub use crate::def::{self, EnclosingRubyScope};
pub use crate::exception::{self, Exception, RubyException};
pub use crate::extn::core::exception::*;
pub use crate::module;
pub use crate::sys;
pub use crate::types::{Float, Int, Ruby};
pub use crate::value::{Block, Value};
pub use crate::{Artichoke, ArtichokeError, BootError};

/// Type alias for errors returned from `init` functions in
/// [`extn`](crate::extn).
pub type InitializeResult<T> = Result<T, BootError>;
