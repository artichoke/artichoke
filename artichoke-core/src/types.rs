//! Artichoke Ruby and Rust type mappings.

use std::fmt;

/// Classes of Rust types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rust {
    Bool,
    Bytes,
    Float,
    Map,
    Object,
    SignedInt,
    String,
    UnsignedInt,
    Vec,
}

impl fmt::Display for Rust {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Classes of Ruby types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ruby {
    Array,
    Bool,
    Class,
    CPointer,
    Data,
    Exception,
    Fiber,
    Fixnum,
    Float,
    Hash,
    InlineStruct,
    Module,
    Nil,
    Object,
    Proc,
    Range,
    SingletonClass,
    String,
    Symbol,
    Unreachable,
}

impl fmt::Display for Ruby {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
