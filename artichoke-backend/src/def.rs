use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::class;
use crate::convert::RustBackedValue;
use crate::module;
use crate::sys;
use crate::{Artichoke, ArtichokeError};

/// Typedef for an mruby free function for an [`mrb_value`](sys::mrb_value) with
/// `tt` [`MRB_TT_DATA`](sys::mrb_vtype::MRB_TT_DATA).
pub type Free = unsafe extern "C" fn(mrb: *mut sys::mrb_state, data: *mut c_void);

/// A generic implementation of a [`Free`] function for
/// [`mrb_value`](sys::mrb_value)s that store an owned copy of an [`Rc`] smart
/// pointer.
///
/// This function calls [`Rc::from_raw`] on the data pointer and drops the
/// resulting [`Rc`].
///
/// # Safety
///
/// This function assumes that the data pointer is to an
/// [`Rc`]`<`[`RefCell`]`<T>>` created by [`Rc::into_raw`]. This fuction bounds
/// `T` by [`RustBackedValue`] which boxes `T` for the mruby VM like this.
///
/// This function assumes it is called by the mruby VM as a free function for
/// an [`MRB_TT_DATA`](sys::mrb_vtype::MRB_TT_DATA).
pub unsafe extern "C" fn rust_data_free<T: 'static + RustBackedValue>(
    _mrb: *mut sys::mrb_state,
    data: *mut c_void,
) {
    if data.is_null() {
        panic!(
            "Received null pointer in rust_data_free<{}>",
            T::ruby_type_name()
        );
    }
    let data = Rc::from_raw(data as *const RefCell<T>);
    drop(data);
}

/// Typedef for a method exposed in the mruby interpreter.
///
/// This function signature is used for all types of mruby methods, including
/// instance methods, class methods, singleton methods, and global methods.
///
/// `slf` is the method receiver, e.g. `s` in the following invocation of
/// `String#start_with?`.
///
/// ```ruby
/// s = 'artichoke crate'
/// s.start_with?('artichoke')
/// ```
///
/// To extract method arguments, use [`sys::mrb_get_args`] and the suppilied
/// interpreter.
pub type Method =
    unsafe extern "C" fn(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;

/// Typesafe wrapper for the [`RClass *`](sys::RClass) of the enclosing scope
/// for an mruby `Module` or `Class`.
///
/// In Ruby, classes and modules can be defined inside of another class or
/// module. mruby only supports resolving [`RClass`](sys::RClass) pointers
/// relative to an enclosing scope. This can be the top level with
/// [`mrb_class_get`](sys::mrb_class_get) and
/// [`mrb_module_get`](sys::mrb_module_get) or it can be under another
/// [`ClassLike`] with [`mrb_class_get_under`](sys::mrb_class_get_under) and
/// [`mrb_module_get_under`](sys::mrb_module_get_under).
///
/// Because there is no C API to resolve class and module names directly, each
/// [`ClassLike`] holds a reference to its enclosing scope so it can recursively
/// resolve its enclosing [`RClass *`](sys::RClass).
#[derive(Clone, Debug)]
pub enum EnclosingRubyScope {
    /// Reference to a Ruby `Class` enclosing scope.
    Class {
        /// Shared copy of the underlying [class definition](class::Spec).
        spec: class::Spec,
    },
    /// Reference to a Ruby `Module` enclosing scope.
    Module {
        /// Shared copy of the underlying [module definition](module::Spec).
        spec: module::Spec,
    },
}

impl EnclosingRubyScope {
    /// Factory for [`EnclosingRubyScope::Class`] that clones an `Rc` smart
    /// pointer wrapped [`class::Spec`].
    ///
    /// This function is useful when extracting an enclosing scope from the
    /// class registry.
    pub fn class(spec: &class::Spec) -> Self {
        Self::Class { spec: spec.clone() }
    }

    /// Factory for [`EnclosingRubyScope::Module`] that clones an `Rc` smart
    /// pointer wrapped [`module::Spec`].
    ///
    /// This function is useful when extracting an enclosing scope from the
    /// module registry.
    pub fn module(spec: &module::Spec) -> Self {
        Self::Module { spec: spec.clone() }
    }

    /// Resolve the [`RClass *`](sys::RClass) of the wrapped [`ClassLike`].
    ///
    /// Return [`None`] if the `ClassLike` has no [`EnclosingRubyScope`].
    ///
    /// The current implemention results in recursive calls to this function
    /// for each enclosing scope.
    pub fn rclass(&self, interp: &Artichoke) -> Option<*mut sys::RClass> {
        match self {
            Self::Class { spec } => spec.rclass(interp),
            Self::Module { spec } => spec.rclass(interp),
        }
    }

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
    pub fn fqname(&self) -> String {
        match self {
            Self::Class { spec } => spec.fqname(),
            Self::Module { spec } => spec.fqname(),
        }
    }
}

impl Eq for EnclosingRubyScope {}

impl PartialEq for EnclosingRubyScope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Class { spec: this }, Self::Class { spec: other }) => this == other,
            (Self::Module { spec: this }, Self::Module { spec: other }) => this == other,
            _ => false,
        }
    }
}

impl Hash for EnclosingRubyScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Class { spec } => spec.hash(state),
            Self::Module { spec } => spec.hash(state),
        };
    }
}

/// `Define` trait allows a type to install classes, modules, and
/// methods into an mruby interpreter.
pub trait Define
where
    Self: ClassLike,
{
    /// Define the class or module and all of its methods into the interpreter.
    ///
    /// Returns the [`RClass *`](sys::RClass) of the newly defined item.
    ///
    /// This function takes a mutable borrow on the [`Artichoke`] interpreter. Ensure
    /// that there are no outstanding borrows on the interpreter or else Rust
    /// will panic.
    fn define(&self, interp: &Artichoke) -> Result<*mut sys::RClass, ArtichokeError>;
}

/// `ClassLike` trait unifies `class::Spec` and `module::Spec`.
pub trait ClassLike
where
    Self: fmt::Debug + fmt::Display,
{
    fn add_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec);

    fn add_self_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec);

    fn cstring(&self) -> &CString;

    fn name(&self) -> &str;

    fn enclosing_scope(&self) -> Option<&EnclosingRubyScope>;

    fn rclass(&self, interp: &Artichoke) -> Option<*mut sys::RClass>;

    /// Compute the fully qualified name of a Class or module. See
    /// [`EnclosingRubyScope::fqname`].
    fn fqname(&self) -> String {
        if let Some(scope) = self.enclosing_scope() {
            format!("{}::{}", scope.fqname(), self.name())
        } else {
            self.name().to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::def::{ClassLike, Define, EnclosingRubyScope};

    #[test]
    fn fqname() {
        struct Root; // A
        struct ModuleUnderRoot; // A::B
        struct ClassUnderRoot; // A::C
        struct ClassUnderModule; // A::B::D
        struct ModuleUnderClass; // A::C::E
        struct ClassUnderClass; // A::C::F

        // Setup: define module and class hierarchy
        let interp = crate::interpreter().expect("init");
        {
            let mut api = interp.0.borrow_mut();
            let root = api.def_module::<Root>("A", None);
            let mod_under_root = api.def_module::<ModuleUnderRoot>(
                "B",
                Some(EnclosingRubyScope::module(Rc::clone(&root))),
            );
            let cls_under_root =
                api.def_class::<ClassUnderRoot>("C", Some(EnclosingRubyScope::module(root)), None);
            let _cls_under_mod = api.def_class::<ClassUnderModule>(
                "D",
                Some(EnclosingRubyScope::module(mod_under_root)),
                None,
            );
            let _mod_under_cls = api.def_module::<ModuleUnderClass>(
                "E",
                Some(EnclosingRubyScope::class(Rc::clone(&cls_under_root))),
            );
            let _cls_under_cls = api.def_class::<ClassUnderClass>(
                "F",
                Some(EnclosingRubyScope::class(cls_under_root)),
                None,
            );
        }

        let spec = interp.0.borrow().module_spec::<Root>().unwrap();
        spec.borrow().define(&interp).expect("def module");
        let spec = interp.0.borrow().module_spec::<ModuleUnderRoot>().unwrap();
        spec.borrow().define(&interp).expect("def module");
        let spec = interp.0.borrow().class_spec::<ClassUnderRoot>().unwrap();
        spec.borrow().define(&interp).expect("def class");
        let spec = interp.0.borrow().class_spec::<ClassUnderModule>().unwrap();
        spec.borrow().define(&interp).expect("def class");
        let spec = interp.0.borrow().module_spec::<ModuleUnderClass>().unwrap();
        spec.borrow().define(&interp).expect("def module");
        let spec = interp.0.borrow().class_spec::<ClassUnderClass>().unwrap();
        spec.borrow().define(&interp).expect("def class");

        let spec = interp
            .0
            .borrow()
            .module_spec::<Root>()
            .expect("Root not defined");
        assert_eq!(&spec.borrow().fqname(), "A");
        assert_eq!(&format!("{}", spec.borrow()), "artichoke module spec -- A");
        let spec = interp
            .0
            .borrow()
            .module_spec::<ModuleUnderRoot>()
            .expect("ModuleUnderRoot not defined");
        assert_eq!(&spec.borrow().fqname(), "A::B");
        assert_eq!(
            &format!("{}", spec.borrow()),
            "artichoke module spec -- A::B"
        );
        let spec = interp
            .0
            .borrow()
            .class_spec::<ClassUnderRoot>()
            .expect("ClassUnderRoot not defined");
        assert_eq!(&spec.borrow().fqname(), "A::C");
        assert_eq!(
            &format!("{}", spec.borrow()),
            "artichoke class spec -- A::C"
        );
        let spec = interp
            .0
            .borrow()
            .class_spec::<ClassUnderModule>()
            .expect("ClassUnderModule not defined");
        assert_eq!(&spec.borrow().fqname(), "A::B::D");
        assert_eq!(
            &format!("{}", spec.borrow()),
            "artichoke class spec -- A::B::D"
        );
        let spec = interp
            .0
            .borrow()
            .module_spec::<ModuleUnderClass>()
            .expect("ModuleUnderClass not defined");
        assert_eq!(&spec.borrow().fqname(), "A::C::E");
        assert_eq!(
            &format!("{}", spec.borrow()),
            "artichoke module spec -- A::C::E"
        );
        let spec = interp
            .0
            .borrow()
            .class_spec::<ClassUnderClass>()
            .expect("ClassUnderClass not defined");
        assert_eq!(&spec.borrow().fqname(), "A::C::F");
        assert_eq!(
            &format!("{}", spec.borrow()),
            "artichoke class spec -- A::C::F"
        );
    }

    mod functional {
        use artichoke_core::eval::Eval;

        use crate::def::{ClassLike, Define};
        use crate::sys;
        use crate::value::ValueLike;

        #[test]
        fn define_method() {
            struct Class;
            struct Module;

            extern "C" fn value(_mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
                unsafe {
                    match slf.tt {
                        sys::mrb_vtype::MRB_TT_CLASS => sys::mrb_sys_fixnum_value(8),
                        sys::mrb_vtype::MRB_TT_MODULE => sys::mrb_sys_fixnum_value(27),
                        sys::mrb_vtype::MRB_TT_OBJECT => sys::mrb_sys_fixnum_value(64),
                        _ => sys::mrb_sys_fixnum_value(125),
                    }
                }
            }
            let interp = crate::interpreter().expect("init");
            let (cls, module) = {
                let mut api = interp.0.borrow_mut();
                let cls = api.def_class::<Class>("DefineMethodTestClass", None, None);
                let module = api.def_module::<Module>("DefineMethodTestModule", None);
                cls.borrow_mut()
                    .add_method("value", value, sys::mrb_args_none());
                cls.borrow_mut()
                    .add_self_method("value", value, sys::mrb_args_none());
                module
                    .borrow_mut()
                    .add_method("value", value, sys::mrb_args_none());
                module
                    .borrow_mut()
                    .add_self_method("value", value, sys::mrb_args_none());
                (cls, module)
            };
            cls.borrow().define(&interp).expect("class install");
            module.borrow().define(&interp).expect("module install");

            interp
                .eval(
                    br#"
                    class DynamicTestClass
                        include DefineMethodTestModule
                        extend DefineMethodTestModule
                    end

                    module DynamicTestModule
                        extend DefineMethodTestModule
                    end
                    "#,
                )
                .expect("eval");

            let result = interp
                .eval(b"DefineMethodTestClass.new.value")
                .expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 64);
            let result = interp.eval(b"DefineMethodTestClass.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 8);
            let result = interp.eval(b"DefineMethodTestModule.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 27);
            let result = interp.eval(b"DynamicTestClass.new.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 64);
            let result = interp.eval(b"DynamicTestClass.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 8);
            let result = interp.eval(b"DynamicTestModule.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 27);
        }
    }
}
