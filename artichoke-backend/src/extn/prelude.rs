//! Prelude for [`extn`](crate::extn) development.
//!
//! Include this module in a source with:
//!
//! ```rust,ignore
//! use crate::extn::prelude::*;
//! ```

pub use crate::class;
pub use crate::convert::RustBackedValue;
pub use crate::def::{self, EnclosingRubyScope, NotDefinedError};
pub use crate::exception::{self, Exception, RubyException};
pub use crate::extn::core::exception::*;
pub use crate::module;
pub use crate::string;
pub use crate::sys;
pub use crate::types::{Float, Int, Ruby};
pub use crate::value::{Block, Value};
pub use crate::{
    Artichoke, Convert, ConvertMut, DefineConstant, Eval, Intern, LoadSources, TryConvert,
    TryConvertMut, ValueLike, Warn,
};

/// Type alias for errors returned from `init` functions in
/// [`extn`](crate::extn).
pub type InitializeResult<T> = Result<T, Exception>;
