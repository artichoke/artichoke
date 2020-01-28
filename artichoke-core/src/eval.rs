//! Run code on an Artichoke interpreter.

use crate::value::Value;

/// Interpreters that implement [`Eval`] expose methods for injecting code and
/// extracting [`Value`]s from the interpereter.
///
/// Implementations are expected to maintain a stack of `Context` objects
/// that maintain filename context across nested invocations of
/// [`Eval::eval`].
pub trait Eval {
    /// Concrete type for return values from eval.
    type Value: Value;

    /// Concrete error type for eval functions.
    type Error: std::error::Error;

    /// Eval code on the artichoke interpreter using the current `Context`.
    fn eval(&self, code: &[u8]) -> Result<Self::Value, Self::Error>;
}
