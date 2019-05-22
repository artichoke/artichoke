use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::class;
use crate::interpreter::Mrb;
use crate::module;
use crate::sys;
use crate::MrbError;

// Types
pub type Free = unsafe extern "C" fn(mrb: *mut sys::mrb_state, data: *mut c_void);
pub type Method =
    unsafe extern "C" fn(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;

#[derive(Clone, Debug)]
pub enum Parent {
    Class { spec: Rc<RefCell<class::Spec>> },
    Module { spec: Rc<RefCell<module::Spec>> },
}

impl Parent {
    pub fn rclass(&self, interp: Mrb) -> Option<*mut sys::RClass> {
        match self {
            Parent::Class { spec } => spec.borrow().rclass(interp),
            Parent::Module { spec } => spec.borrow().rclass(interp),
        }
    }

    pub fn fqname(&self) -> String {
        match self {
            Parent::Class { spec } => spec.borrow().fqname(),
            Parent::Module { spec } => spec.borrow().fqname(),
        }
    }
}

impl Eq for Parent {}

impl PartialEq for Parent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Parent::Class { spec: self_spec }, Parent::Class { spec: other_spec }) => {
                self_spec == other_spec
            }
            (Parent::Module { spec: self_spec }, Parent::Module { spec: other_spec }) => {
                self_spec == other_spec
            }
            _ => false,
        }
    }
}

impl Hash for Parent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Parent::Class { spec } => spec.borrow().hash(state),
            Parent::Module { spec } => spec.borrow().hash(state),
        };
    }
}

/// `Define` trait allows a type to install classes, modules, and
/// methods into an mruby interpreter.
pub trait Define
where
    Self: ClassLike,
{
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

    fn parent(&self) -> Option<Parent>;

    fn rclass(&self, interp: Mrb) -> Option<*mut sys::RClass>;

    fn fqname(&self) -> String {
        if let Some(parent) = self.parent() {
            let parentfq = match parent {
                Parent::Class { spec } => spec.borrow().fqname(),
                Parent::Module { spec } => spec.borrow().fqname(),
            };
            format!("{}::{}", parentfq, self.name())
        } else {
            self.name().to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::def::{ClassLike, Define, Parent};
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
                Some(Parent::Module {
                    spec: Rc::clone(&root),
                }),
            );
            let cls_under_root = api.def_class::<ClassUnderRoot>(
                "C",
                Some(Parent::Module {
                    spec: Rc::clone(&root),
                }),
                None,
            );
            let _cls_under_mod = api.def_class::<ClassUnderModule>(
                "D",
                Some(Parent::Module {
                    spec: Rc::clone(&mod_under_root),
                }),
                None,
            );
            let _mod_under_cls = api.def_module::<ModuleUnderClass>(
                "E",
                Some(Parent::Class {
                    spec: Rc::clone(&cls_under_root),
                }),
            );
            let _cls_under_cls = api.def_class::<ClassUnderClass>(
                "F",
                Some(Parent::Class {
                    spec: Rc::clone(&cls_under_root),
                }),
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
