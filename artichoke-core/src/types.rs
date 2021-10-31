//! Ruby and Rust type mappings.

use core::fmt;

/// Classes of Rust types.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Arbitrary Rust struct type.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Rust ")?;
        match self {
            Self::Bool => f.write_str("bool"),
            Self::Bytes => f.write_str("Vec<u8>"),
            Self::Float => f.write_str("f64"),
            Self::Map => f.write_str("HashMap"),
            Self::Object => f.write_str("Heap-allocated object"),
            Self::SignedInt => f.write_str("i64"),
            Self::String => f.write_str("String"),
            Self::UnsignedInt => f.write_str("u64"),
            Self::Vec => f.write_str("Vec<Value>"),
        }
    }
}

/// Classes of Ruby types.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Similar to a [`HashMap`], but iterates by insertion order.
    ///
    /// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
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
    /// Similar to a Rust [iterator](core::iter).
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
    #[must_use]
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Array => "Array",
            Self::Bool => "Boolean",
            Self::Class => "Class",
            Self::CPointer => "C Pointer",
            Self::Data => "Rust-backed Ruby instance",
            Self::Exception => "Exception",
            Self::Fiber => "Fiber",
            Self::Fixnum => "Integer",
            Self::Float => "Float",
            Self::Hash => "Hash",
            Self::InlineStruct => "Inline Struct",
            Self::Module => "Module",
            Self::Nil => "NilClass",
            Self::Object => "Object",
            Self::Proc => "Proc",
            Self::Range => "Range",
            Self::SingletonClass => "Singleton (anonymous) class",
            Self::String => "String",
            Self::Symbol => "Symbol",
            Self::Unreachable => "internal and unreachable",
            Self::RecursiveSelfOwnership => "recursive self ownership",
        }
    }
}

impl fmt::Display for Ruby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Ruby ")?;
        f.write_str(self.class_name())?;
        Ok(())
    }
}
