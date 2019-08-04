//! Define `Class`es, `Module`s and methods on an interpreter.

use std::any::Any;

use crate::ArtichokeError;

/// Interpreters that implement [`DeclareClassLike`] expose methods for
/// declaring Ruby `Class`es and `Module`s on the interpreter.
///
/// A [`ClassLike`] may need to be declared before it is defined with
/// [`Define`].
pub trait DeclareClassLike {
    /// Concrete type used to free instances of `Class`es.
    type Free;

    /// Concrete type for a `Class` spec.
    type ClassSpec: ClassLike;

    /// Concrete type for a `Module` spec.
    type ModuleSpec: ClassLike;

    /// Concrete type for an enclosing scope
    type Scope: EnclosingRubyScope;

    /// Create a class definition bound to a Rust type `T`. Class definitions
    /// have the same lifetime as `self`. Class defs are stored by
    /// [`TypeId`](std::any::TypeId) of `T`.
    ///
    /// Class specs can also be retrieved from the state after creation with
    /// [`DeclareClassLike::class_spec`].
    fn def_class<T: Any>(
        &mut self,
        name: &str,
        enclosing_scope: Option<Self::Scope>,
        free: Option<Self::Free>,
    ) -> Self::ClassSpec;

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`DeclareClassLike::def_class`].
    fn class_spec<T: Any>(&self) -> Option<Self::ClassSpec>;

    /// Create a module definition bound to a Rust type `T`. Module definitions
    /// have the same lifetime as `self`. Module defs are stored by
    /// [`TypeId`](std::any::TypeId) of `T`.
    ///
    /// Module specs can also be retrieved from the state after creation with
    /// [`DeclareClassLike::module_spec`].
    fn def_module<T: Any>(
        &mut self,
        name: &str,
        enclosing_scope: Option<Self::Scope>,
    ) -> Self::ModuleSpec;

    /// Retrieve a module definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a module spec
    /// registered for it using [`DeclareClassLike::def_module`].
    fn module_spec<T: Any>(&self) -> Option<Self::ModuleSpec>;
}

/// Typesafe wrapper for the [`ClassLike`] of the enclosing scope for a Ruby
/// `Module` or `Class`.
///
/// In Ruby, classes and modules can be defined inside of another class or
/// module. Some intepreters may only support resolving `ClassLike`s relative to
/// an enclosing scope.
pub trait EnclosingRubyScope {
    /// Concrete type for a `Class` spec.
    type ClassSpec: ClassLike;

    /// Concrete type for a `Module` spec.
    type ModuleSpec: ClassLike;

    /// Factory for an [`EnclosingRubyScope`] that is a Ruby `Class`.
    fn class(spec: Self::ClassSpec) -> Self;

    /// Factory for an [`EnclosingRubyScope`] that is a Ruby `Module`.
    fn module(spec: Self::ModuleSpec) -> Self;

    /// Get the fully qualified name of the wrapped [`ClassLike`].
    ///
    /// For example, in the following Ruby code, `C` has an fqname of `A::B::C`.
    ///
    /// ```ruby
    /// module A
    ///   class B
    ///     module C
    ///       CONST = 1
    ///     end
    ///   end
    /// end
    /// ```
    ///
    /// The current implemention results in recursive calls to this function
    /// for each enclosing scope.
    fn fqname(&self) -> String;
}

/// `Define` trait allows a type to install classes, modules, and
/// methods into an Artichoke interpreter.
pub trait Define {
    /// Concrete type for Artichoke interpreter;
    type Artichoke;
    /// Concrete type for result of defining `self`.
    type Defined;

    /// Define the class or module and all of its methods into the interpreter.
    fn define(&self, interp: Self::Artichoke) -> Result<Self::Defined, ArtichokeError>;
}

/// `ClassLike` trait unifies Ruby `Class`es and `Module`s.
pub trait ClassLike {
    /// Concrete type for method.
    type Method;
    /// Concrete type for argspec.
    ///
    /// Argspecs define how the interpreter handles arguments for a method.
    type ArgSpec;

    /// Concrete type for an enclosing scope
    type Scope: EnclosingRubyScope;

    /// Add, but do not define, a method to this `ClassLike`.
    fn add_method(&mut self, name: &str, method: Self::Method, args: Self::ArgSpec);

    /// Add, but do not define, a method to the singleton class for this
    /// `ClassLike`.
    fn add_self_method(&mut self, name: &str, method: Self::Method, args: Self::ArgSpec);

    /// Name of this `Class` or `Module`.
    ///
    /// The local constant defined in the [`EnclosingRubyScope`].
    fn name(&self) -> String;

    /// The [`EnclosingRubyScope`] this `ClassLike` will be defined under or
    /// `None` if this `ClassLike` is defined in
    /// [top self](crate::top_self::TopSelf).
    fn enclosing_scope(&self) -> Option<Self::Scope>;

    /// Compute the fully qualified name of a `Class` or `Module`.
    ///
    /// See [`EnclosingRubyScope::fqname`].
    fn fqname(&self) -> String;
}
