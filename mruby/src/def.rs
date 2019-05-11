use std::ffi::{c_void, CString};
use std::fmt;
use std::rc::Rc;

use crate::class;
use crate::interpreter::{Mrb, MrbError};
use crate::module;
use crate::sys;

// Types
pub type Free = unsafe extern "C" fn(mrb: *mut sys::mrb_state, data: *mut c_void);
pub type Method =
    unsafe extern "C" fn(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Parent {
    Class { spec: Rc<class::Spec> },
    Module { spec: Rc<module::Spec> },
}

impl Parent {
    pub fn rclass(&self, interp: Mrb) -> *mut sys::RClass {
        match self {
            Parent::Class { spec } => spec.rclass(interp),
            Parent::Module { spec } => spec.rclass(interp),
        }
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

    fn rclass(&self, interp: Mrb) -> *mut sys::RClass;

    fn fqname(&self) -> String {
        if let Some(parent) = self.parent() {
            let parentfq = match parent {
                Parent::Class { spec } => spec.fqname(),
                Parent::Module { spec } => spec.fqname(),
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
            api.def_module::<Root>("A", None);
            let spec = api.module_spec::<Root>();
            api.def_module::<ModuleUnderRoot>(
                "B",
                Some(Parent::Module {
                    spec: Rc::clone(&spec),
                }),
            );
            api.def_class::<ClassUnderRoot>(
                "C",
                Some(Parent::Module {
                    spec: Rc::clone(&spec),
                }),
                None,
            );
            let spec = api.module_spec::<ModuleUnderRoot>();
            api.def_class::<ClassUnderModule>(
                "D",
                Some(Parent::Module {
                    spec: Rc::clone(&spec),
                }),
                None,
            );
            let spec = api.class_spec::<ClassUnderRoot>();
            api.def_module::<ModuleUnderClass>(
                "E",
                Some(Parent::Class {
                    spec: Rc::clone(&spec),
                }),
            );
            api.def_class::<ClassUnderClass>(
                "F",
                Some(Parent::Class {
                    spec: Rc::clone(&spec),
                }),
                None,
            );
        }

        let api = interp.borrow();
        api.module_spec::<Root>()
            .define(&interp)
            .expect("def module");
        api.module_spec::<ModuleUnderRoot>()
            .define(&interp)
            .expect("def module");
        api.class_spec::<ClassUnderRoot>()
            .define(&interp)
            .expect("def class");
        api.class_spec::<ClassUnderModule>()
            .define(&interp)
            .expect("def class");
        api.module_spec::<ModuleUnderClass>()
            .define(&interp)
            .expect("def module");
        api.class_spec::<ClassUnderClass>()
            .define(&interp)
            .expect("def class");

        let spec = api.module_spec::<Root>();
        assert_eq!(&spec.fqname(), "A");
        assert_eq!(&format!("{}", spec), "mruby module spec -- A");
        let spec = api.module_spec::<ModuleUnderRoot>();
        assert_eq!(&spec.fqname(), "A::B");
        assert_eq!(&format!("{}", spec), "mruby module spec -- A::B");
        let spec = api.class_spec::<ClassUnderRoot>();
        assert_eq!(&spec.fqname(), "A::C");
        assert_eq!(&format!("{}", spec), "mruby class spec -- A::C");
        let spec = api.class_spec::<ClassUnderModule>();
        assert_eq!(&spec.fqname(), "A::B::D");
        assert_eq!(&format!("{}", spec), "mruby class spec -- A::B::D");
        let spec = api.module_spec::<ModuleUnderClass>();
        assert_eq!(&spec.fqname(), "A::C::E");
        assert_eq!(&format!("{}", spec), "mruby module spec -- A::C::E");
        let spec = api.class_spec::<ClassUnderClass>();
        assert_eq!(&spec.fqname(), "A::C::F");
        assert_eq!(&format!("{}", spec), "mruby class spec -- A::C::F");
    }

    mod functional {
        use crate::convert::TryFromMrb;
        use crate::def::{ClassLike, Define};
        use crate::interpreter::{Interpreter, MrbApi};
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
            {
                let mut api = interp.borrow_mut();
                api.def_class::<Class>("DefineMethodTestClass", None, None);
                api.def_module::<Module>("DefineMethodTestModule", None);
            }
            {
                let mut api = interp.borrow_mut();
                let spec = api.class_spec_mut::<Class>();
                spec.add_method("value", value, sys::mrb_args_none());
                spec.add_self_method("value", value, sys::mrb_args_none());
            }
            {
                let mut api = interp.borrow_mut();
                let spec = api.module_spec_mut::<Module>();
                spec.add_method("value", value, sys::mrb_args_none());
                spec.add_self_method("value", value, sys::mrb_args_none());
            }
            {
                let api = interp.borrow();
                api.class_spec::<Class>()
                    .define(&interp)
                    .expect("class install");
                api.module_spec::<Module>()
                    .define(&interp)
                    .expect("module install");
            }

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
