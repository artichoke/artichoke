//! Run code on an Artichoke interpreter.

use crate::value::Value;
use crate::ArtichokeError;

/// Marker trait for a context used by [`Eval`].
pub trait Context {}

/// Interpreters that implement [`Eval`] expose methods for injecting code and
/// extracting [`Value`]s from the interpereter.
///
/// Implementations are expected to maintain a stack of `Context` objects
/// that maintain filename context across nested invocations of
/// [`Eval::eval`].
pub trait Eval {
    /// Concrete type for eval context.
    type Context: Context;

    /// Concrete type for return values from eval.
    type ReturnValue: Value;

    /// Filename of the top eval context.
    const TOP_FILENAME: &'static str = "(eval)";

    /// Eval code on the artichoke interpreter using the current `Context`.
    fn eval(&self, code: &[u8]) -> Result<Self::ReturnValue, ArtichokeError>;

    /// Eval code on the artichoke interpreter using the current `Context`.
    ///
    /// Exceptions will unwind past this call.
    fn unchecked_eval(&self, code: &[u8]) -> Self::ReturnValue;

    /// Eval code on the artichoke interpreter using a custom `Context`.
    ///
    /// `Context` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    fn eval_with_context(
        &self,
        code: &[u8],
        context: Self::Context,
    ) -> Result<Self::ReturnValue, ArtichokeError>;

    /// Eval code on the artichoke interpreter using a custom `Context`.
    ///
    /// `Context` allows manipulating interpreter state before eval, for
    /// example, setting the `__FILE__` magic constant.
    ///
    /// Exceptions will unwind past this call.
    fn unchecked_eval_with_context(&self, code: &[u8], context: Self::Context)
        -> Self::ReturnValue;

    /// Peek at the top of the [`Context`] stack.
    fn peek_context(&self) -> Option<&Self::Context>;

    /// Push an `Context` onto the stack.
    fn push_context(&mut self, context: Self::Context);

    /// Pop an `Context` from the stack.
    fn pop_context(&mut self);
}
