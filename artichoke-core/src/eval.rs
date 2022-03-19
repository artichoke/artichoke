//! Run code on an Artichoke interpreter.

#[cfg(feature = "std")]
use std::ffi::OsStr;
#[cfg(feature = "std")]
use std::path::Path;

use crate::value::Value;

/// Execute code and retrieve its result.
pub trait Eval {
    /// Concrete type for return values from eval.
    type Value: Value;

    /// Concrete error type for eval functions.
    type Error;

    /// Eval code on the Artichoke interpreter using the current parser context.
    ///
    /// # Errors
    ///
    /// If an exception is raised on the interpreter, then an error is returned.
    fn eval(&mut self, code: &[u8]) -> Result<Self::Value, Self::Error>;

    /// Eval code on the Artichoke interpreter using the current parser context
    /// when given code as an [`OsStr`].
    ///
    /// # Errors
    ///
    /// If an exception is raised on the interpreter, then an error is returned.
    ///
    /// If `code` cannot be converted to a `&[u8]` on the current platform, then
    /// an error is returned.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn eval_os_str(&mut self, code: &OsStr) -> Result<Self::Value, Self::Error>;

    /// Eval code on the Artichoke interpreter using a new file `Context` given
    /// a file path.
    ///
    /// # Errors
    ///
    /// If an exception is raised on the interpreter, then an error is returned.
    ///
    /// If `path` does not exist or code cannot be read, an error is returned.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn eval_file(&mut self, file: &Path) -> Result<Self::Value, Self::Error>;
}
