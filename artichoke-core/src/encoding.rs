//! Define an encoding which can be stored by `EncodingRegistry`.
use alloc::vec::Vec;

/// Define an encoding which can be stored by `EncodingRegistry`.
pub trait Encoding {
    /// An identifier which can be stored in the flags part of an `RString`.
    fn flag(&self) -> u8;

    /// The different bare symbol aliases that can be used for the `Encoding`
    /// when it is defined.
    fn aliases(&self) -> Vec<Vec<u8>>;

    /// Whether the Encoding is compatible with ASCII encoding.
    fn is_ascii_compatible(&self) -> bool;

    /// Whether the encoding is a dummy encoding.
    fn is_dummy(&self) -> bool;

    /// The string be displayed when debugging the encoding.
    fn inspect(&self) -> &'static str;

    /// The primary name to display when displaying the encoding.
    fn name(&self) -> &'static str;

    /// All names that can be used to refer to this encoding.
    fn names(&self) -> &'static [&'static str];
}
