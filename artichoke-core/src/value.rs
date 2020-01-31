//! Types that implement `Value` can be represented in the Artichoke VM.

use crate::convert::TryConvert;
use crate::ArtichokeError;

/// A value in the Artichoke VM, equivalent to an `RValue` in MRI.
pub trait Value
where
    Self: Sized,
{
    /// Concrete type for Artichoke interpreter.
    type Artichoke;

    /// Concrete type for arguments passed to [`funcall`](Value::funcall).
    type Arg;

    /// Concrete type for blocks passed to [`funcall`](Value::funcall).
    type Block;

    /// Concrete error type for funcall errors.
    type Error: std::error::Error;

    /// Call a method on this [`Value`] with arguments and an optional block.
    ///
    /// # Errors
    ///
    /// All Ruby expressions are fallible because they may raise exceptions.
    /// `funcall` should return raised exceptions that reach the top level as
    /// errors.
    fn funcall<T>(
        &self,
        func: &str,
        args: &[Self::Arg],
        block: Option<Self::Block>,
    ) -> Result<T, Self::Error>
    where
        Self::Artichoke: TryConvert<Self, T>;

    /// Consume `self` and try to convert `self` to type `T`.
    ///
    /// If you do not want to consume this [`Value`], use [`Value::itself`].
    ///
    /// # Errors
    ///
    /// This method will attempt a fallible conversion by invoking
    /// [`TryConvert::try_convert`]. Implementors should propagate these errors.
    fn try_into<T>(self) -> Result<T, ArtichokeError>
    where
        Self::Artichoke: TryConvert<Self, T>;

    /// Call `#itself` on this [`Value`] and try to convert the result to type
    /// `T`.
    ///
    /// If you want to consume this [`Value`], use [`Value::try_into`].
    ///
    /// # Errors
    ///
    /// This method will attempt a fallible conversion by invoking
    /// [`TryConvert::try_convert`]. Implementors should propagate these errors.
    fn itself<T>(&self) -> Result<T, ArtichokeError>
    where
        Self::Artichoke: TryConvert<Self, T>;

    /// Call `#freeze` on this [`Value`].
    ///
    /// # Errors
    ///
    /// This method may delegate to the underlying VM by calling
    /// `self.freeze`. In Ruby all methods are overrideable, so calling
    /// `freeze` may be fallible and raise an exception.
    ///
    /// Implementors should propagate these exceptions.
    fn freeze(&mut self) -> Result<(), Self::Error>;

    /// Call `#frozen?` on this [`Value`].
    fn is_frozen(&self) -> bool;

    /// Whether `self` is `nil`
    fn is_nil(&self) -> bool;

    /// Whether `self` responds to a method.
    ///
    /// Equivalent to invoking `#respond_to?` on this [`Value`].
    ///
    /// # Errors
    ///
    /// This method may delegate to the underlying VM by calling
    /// `self.respond_to?`. In Ruby all methods are overrideable, so calling
    /// `respond_to?` may be fallible and raise an exception.
    ///
    /// Implementors should propagate these exceptions.
    fn respond_to(&self, method: &str) -> Result<bool, Self::Error>;

    /// Call `#inspect` on this [`Value`].
    ///
    /// This function can never fail.
    fn inspect(&self) -> Vec<u8>;

    /// Call `#to_s` on this [`Value`].
    ///
    /// This function can never fail.
    fn to_s(&self) -> Vec<u8>;
}
