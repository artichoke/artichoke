//! Define and store encodings on an interpreter.

use crate::encoding::Encoding;
use alloc::vec::Vec;

/// Define and store encoding specs on an interpreter.
///
/// An encoding spec is a static set of information the interpreter requires to
/// define a Ruby `Encoding` object.
pub trait EncodingRegistry {
    /// Concrete value type for boxed Ruby values.
    type Value;

    /// Concrete error type for errors encountered when manipulating the encoding registry.
    type Error;

    /// Type representing an encoding specification.
    type Spec: Encoding + 'static;

    /// Create an encoding definition bound to a Rust type `T`.
    ///
    /// Encoding definitions have the same lifetime as the interpreter.
    ///
    /// # Errors
    ///
    /// If the encoding registry state is inaccessible, an error is returned.
    fn def_encoding(&mut self, spec: Self::Spec) -> Result<(), Self::Error>;

    /// Retrieve all defined encodings
    ///
    /// # Errors
    ///
    /// If the encoding registry state is inaccessible, an error is returned.
    fn encodings(&self) -> Result<Vec<&Self::Spec>, Self::Error>;

    /// Retrieve a boxed Ruby value containing an `Encoding` object bound to a
    /// given encoding.
    ///
    /// `Ok(None)` is returned if the interpreter cannot find or load the
    /// associated encoding. Encoding should be defined with `def_encoding`.
    ///
    /// # Errors
    ///
    /// If the encoding registry state is inaccessible, an error is returned.
    fn encoding_of(&self, spec: &Self::Spec) -> Result<Option<Self::Value>, Self::Error>;
}
