//! Artichoke Ruby and Rust type mappings.

use std::fmt;

/// Classes of Rust types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rust {
    /// Rust `bool` type.
    Bool,
    /// Rust `Vec<u8>` type.
    Bytes,
    /// Rust float type.
    ///
    /// Float width is dependent on interpreter implementation and architecture.
    Float,
    /// Rust `HashMap<K, V>` type.
    Map,
    /// Aribtrary Rust struct type.
    Object,
    /// Rust signed integer type.
    ///
    /// Int width is dependent on interpreter implementation and architecture.
    SignedInt,
    /// Rust `String` type.
    String,
    /// Rust unsigned integer type.
    ///
    /// Int width is dependent on interpreter implementation and architecture.
    UnsignedInt,
    /// Rust `Vec<T>` type.
    Vec,
}

impl fmt::Display for Rust {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Classes of Ruby types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ruby {
    /// Ruby `Array` type.
    Array,
    /// Ruby `TrueClass` and `FalseClass` type.
    Bool,
    /// Ruby `Class` type.
    Class,
    /// FFI type for a borrowed C pointer.
    CPointer,
    /// FFI type for an owned C pointer.
    Data,
    /// Ruby `Exception` type.
    Exception,
    /// Ruby `Fiber` type.
    Fiber,
    /// Ruby `Fixnum` type.
    ///
    /// `Fixnum` is a type of `Integer` which represents numbers from
    /// `[-u64::MAX, us64::MAX]`. `Fixnum`s have a special algorithm for
    /// object IDs: `2 * self - 1`.
    Fixnum,
    /// Ruby `Float` type.
    Float,
    /// Ruby `Hash` type.
    ///
    /// Similar to a [`HashMap`](std::collections::HashMap), but with remembered
    /// insertion order.
    Hash,
    /// Internal type for non-heap allocated structs.
    InlineStruct,
    /// Ruby `Module` type.
    Module,
    /// Ruby `nil` singleton type, the only instance of `NilClass`.
    Nil,
    /// Ruby `Object` type.
    ///
    /// This type represents instances of classes defined in the Artichoke VM.
    Object,
    /// Ruby `Proc` type.
    ///
    /// `Proc` is a callable closure that captures lexical scope. `Proc`s can
    /// be arbitrary arity and may or may not enforce this arity when called.
    Proc,
    /// Ruby `Range` type.
    ///
    /// Similar to a Rust [iterator](std::iter).
    Range,
    /// Internal type for the singleton class of an object.
    SingletonClass,
    /// Ruby `String` type.
    ///
    /// In Artichoke, `String`s have a limited set of encodings. A `String` can
    /// be UTF-8, [maybe UTF-8](https://docs.rs/bstr/), or binary.
    String,
    /// Ruby `Symbol` type.
    ///
    /// An interned `String`. Symbols are never freed by the interpreter.
    Symbol,
    /// Unreachable interpreter value. Receiving one of these from the
    /// interpreter is a bug.
    Unreachable,
    /// A special `Value` that is a placeholder for collections that own
    /// themselves.
    RecursiveSelfOwnership,
}

impl Ruby {
    /// Ruby `Class` name for VM type.
    pub fn class_name(self) -> &'static str {
        match self {
            Ruby::Array => "Array",
            Ruby::Bool => "Boolean",
            Ruby::Class => "Class",
            Ruby::CPointer => "C Pointer",
            Ruby::Data => "Rust-backed Ruby instance",
            Ruby::Exception => "Exception",
            Ruby::Fiber => "Fiber",
            Ruby::Fixnum => "Fixnum",
            Ruby::Float => "Float",
            Ruby::Hash => "Hash",
            Ruby::InlineStruct => "Inline Struct",
            Ruby::Module => "Module",
            Ruby::Nil => "NilClass",
            Ruby::Object => "Object",
            Ruby::Proc => "Proc",
            Ruby::Range => "Range",
            Ruby::SingletonClass => "Singleton (anonymous) class",
            Ruby::String => "String",
            Ruby::Symbol => "Symbol",
            Ruby::Unreachable => "internal and unreachable",
            Ruby::RecursiveSelfOwnership => "recursive self ownership",
        }
    }
}

impl fmt::Display for Ruby {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ruby {}", self.class_name())
    }
}
