use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::rc::Rc;

use crate::class;
use crate::interpreter::Mrb;
use crate::module;
use crate::sys;
use crate::MrbError;

/// Typedef for an mruby free function for an [`mrb_value`](sys::mrb_value) with
/// `tt` [`MRB_TT_DATA`](sys::mrb_vtype::MRB_TT_DATA).
pub type Free = unsafe extern "C" fn(mrb: *mut sys::mrb_state, data: *mut c_void);

/// A generic implementation of a [`Free`] function for
/// [`mrb_value`](sys::mrb_value)s that store an owned copy of an [`Rc`] smart
/// pointer.
///
/// **Warning**: This free function assumes the `data` pointer is an
/// `Rc<RefCell<T>>`. If that assumption does not hold, this function has
/// undefined behavior and may result in a segfault.
pub unsafe extern "C" fn rust_data_free<T>(_mrb: *mut sys::mrb_state, data: *mut c_void) {
    // Implicitly dropped by going out of scope
    mem::transmute::<*mut c_void, Rc<RefCell<T>>>(data);
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
/// s = 'mruby crate'
/// s.start_with?('mruby')
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
        spec: Rc<RefCell<class::Spec>>,
    },
    /// Reference to a Ruby `Module` enclosing scope.
    Module {
        /// Shared copy of the underlying [module definition](module::Spec).
        spec: Rc<RefCell<module::Spec>>,
    },
}

impl EnclosingRubyScope {
    /// Factory for [`EnclosingRubyScope::Class`] that clones an `Rc` smart
    /// pointer wrapped [`class::Spec`].
    ///
    /// This function is useful when extracting an enclosing scope from the
    /// class registry:
    ///
    /// ```rust
    /// use mruby::def::EnclosingRubyScope;
    /// use mruby::interpreter::Interpreter;
    ///
    /// struct Fixnum;
    /// struct Inner;
    ///
    /// let interp = Interpreter::create().expect("mrb init");
    /// let mut api = interp.borrow_mut();
    /// if let Some(scope) = api.class_spec::<Fixnum>().map(EnclosingRubyScope::class) {
    ///     api.def_class::<Inner>("Inner", Some(scope), None);
    /// }
    /// ```
    ///
    /// Which defines this Ruby `Class`:
    ///
    /// ```ruby
    /// class Fixnum
    ///   class Inner
    ///   end
    /// end
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn class(spec: Rc<RefCell<class::Spec>>) -> Self {
        EnclosingRubyScope::Class {
            spec: Rc::clone(&spec),
        }
    }

    /// Factory for [`EnclosingRubyScope::Module`] that clones an `Rc` smart
    /// pointer wrapped [`module::Spec`].
    ///
    /// This function is useful when extracting an enclosing scope from the
    /// module registry:
    ///
    /// ```rust
    /// use mruby::def::EnclosingRubyScope;
    /// use mruby::interpreter::Interpreter;
    ///
    /// struct Kernel;
    /// struct Inner;
    ///
    /// let interp = Interpreter::create().expect("mrb init");
    /// let mut api = interp.borrow_mut();
    /// if let Some(scope) = api.module_spec::<Kernel>().map(EnclosingRubyScope::module) {
    ///     api.def_class::<Inner>("Inner", Some(scope), None);
    /// }
    /// ```
    ///
    /// Which defines this Ruby `Class`:
    ///
    /// ```ruby
    /// module Kernel
    ///   class Inner
    ///   end
    /// end
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn module(spec: Rc<RefCell<module::Spec>>) -> Self {
        EnclosingRubyScope::Module {
            spec: Rc::clone(&spec),
        }
    }

    /// Resolve the [`RClass *`](sys::RClass) of the wrapped [`ClassLike`].
    ///
    /// Return [`None`] if the `ClassLike` has no [`EnclosingRubyScope`].
    ///
    /// The current implemention results in recursive calls to this function
    /// for each enclosing scope.
    pub fn rclass(&self, interp: &Mrb) -> Option<*mut sys::RClass> {
        match self {
            EnclosingRubyScope::Class { spec } => spec.borrow().rclass(interp),
            EnclosingRubyScope::Module { spec } => spec.borrow().rclass(interp),
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
            EnclosingRubyScope::Class { spec } => spec.borrow().fqname(),
            EnclosingRubyScope::Module { spec } => spec.borrow().fqname(),
        }
    }
}

impl Eq for EnclosingRubyScope {}

impl PartialEq for EnclosingRubyScope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                EnclosingRubyScope::Class { spec: this },
                EnclosingRubyScope::Class { spec: other },
            ) => this == other,
            (
                EnclosingRubyScope::Module { spec: this },
                EnclosingRubyScope::Module { spec: other },
            ) => this == other,
            _ => false,
        }
    }
}

impl Hash for EnclosingRubyScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            EnclosingRubyScope::Class { spec } => spec.borrow().hash(state),
            EnclosingRubyScope::Module { spec } => spec.borrow().hash(state),
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
    fn define(&self, interp: &Mrb) -> Result<*mut sys::RClass, MrbError>;
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

    fn enclosing_scope(&self) -> Option<EnclosingRubyScope>;

    fn rclass(&self, interp: &Mrb) -> Option<*mut sys::RClass>;

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
    use crate::interpreter::Interpreter;

    #[test]
    fn fqname() {
        struct Root; // A
        struct ModuleUnderRoot; // A::B
        struct ClassUnderRoot; // A::C
        struct ClassUnderModule; // A::B::D
        struct ModuleUnderClass; // A::C::E
        struct ClassUnderClass; // A::C::F

        // Setup: define module and class hierarchy
        let interp = Interpreter::create().expect("mrb init");
        {
            let mut api = interp.borrow_mut();
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

        let api = interp.borrow();
        api.module_spec::<Root>()
            .unwrap()
            .borrow()
            .define(&interp)
            .expect("def module");
        api.module_spec::<ModuleUnderRoot>()
            .unwrap()
            .borrow()
            .define(&interp)
            .expect("def module");
        api.class_spec::<ClassUnderRoot>()
            .unwrap()
            .borrow()
            .define(&interp)
            .expect("def class");
        api.class_spec::<ClassUnderModule>()
            .unwrap()
            .borrow()
            .define(&interp)
            .expect("def class");
        api.module_spec::<ModuleUnderClass>()
            .unwrap()
            .borrow()
            .define(&interp)
            .expect("def module");
        api.class_spec::<ClassUnderClass>()
            .unwrap()
            .borrow()
            .define(&interp)
            .expect("def class");

        let spec = api.module_spec::<Root>().expect("Root not defined");
        assert_eq!(&spec.borrow().fqname(), "A");
        assert_eq!(&format!("{}", spec.borrow()), "mruby module spec -- A");
        let spec = api
            .module_spec::<ModuleUnderRoot>()
            .expect("ModuleUnderRoot not defined");
        assert_eq!(&spec.borrow().fqname(), "A::B");
        assert_eq!(&format!("{}", spec.borrow()), "mruby module spec -- A::B");
        let spec = api
            .class_spec::<ClassUnderRoot>()
            .expect("ClassUnderRoot not defined");
        assert_eq!(&spec.borrow().fqname(), "A::C");
        assert_eq!(&format!("{}", spec.borrow()), "mruby class spec -- A::C");
        let spec = api
            .class_spec::<ClassUnderModule>()
            .expect("ClassUnderModule not defined");
        assert_eq!(&spec.borrow().fqname(), "A::B::D");
        assert_eq!(&format!("{}", spec.borrow()), "mruby class spec -- A::B::D");
        let spec = api
            .module_spec::<ModuleUnderClass>()
            .expect("ModuleUnderClass not defined");
        assert_eq!(&spec.borrow().fqname(), "A::C::E");
        assert_eq!(
            &format!("{}", spec.borrow()),
            "mruby module spec -- A::C::E"
        );
        let spec = api
            .class_spec::<ClassUnderClass>()
            .expect("ClassUnderClass not defined");
        assert_eq!(&spec.borrow().fqname(), "A::C::F");
        assert_eq!(&format!("{}", spec.borrow()), "mruby class spec -- A::C::F");
    }

    mod functional {
        use crate::convert::TryFromMrb;
        use crate::def::{ClassLike, Define};
        use crate::eval::MrbEval;
        use crate::interpreter::Interpreter;
        use crate::sys;

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
            let interp = Interpreter::create().expect("mrb init");
            let (cls, module) = {
                let mut api = interp.borrow_mut();
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
                    r#"
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
                .eval("DefineMethodTestClass.new.value")
                .expect("eval");
            let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
            assert_eq!(result, 64);
            let result = interp.eval("DefineMethodTestClass.value").expect("eval");
            let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
            assert_eq!(result, 8);
            let result = interp.eval("DefineMethodTestModule.value").expect("eval");
            let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
            assert_eq!(result, 27);
            let result = interp.eval("DynamicTestClass.new.value").expect("eval");
            let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
            assert_eq!(result, 64);
            let result = interp.eval("DynamicTestClass.value").expect("eval");
            let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
            assert_eq!(result, 8);
            let result = interp.eval("DynamicTestModule.value").expect("eval");
            let result = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
            assert_eq!(result, 27);
        }
    }
}
