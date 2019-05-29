use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::AsRef;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::def::{ClassLike, Define, Free, Method, Parent};
use crate::interpreter::Mrb;
use crate::method;
use crate::sys;
use crate::value::Value;
use crate::MrbError;

pub struct Spec {
    name: String,
    cstring: CString,
    data_type: sys::mrb_data_type,
    methods: HashSet<method::Spec>,
    parent: Option<Parent>,
    super_class: Option<Rc<RefCell<Spec>>>,
    is_mrb_tt_data: bool,
}

impl Spec {
    pub fn new<T>(name: T, parent: Option<Parent>, free: Option<Free>) -> Self
    where
        T: AsRef<str>,
    {
        let cstr = CString::new(name.as_ref()).expect("name for data type");
        let data_type = sys::mrb_data_type {
            struct_name: cstr.as_ptr(),
            dfree: free,
        };
        Self {
            name: name.as_ref().to_owned(),
            cstring: cstr,
            data_type,
            methods: HashSet::new(),
            parent,
            super_class: None,
            is_mrb_tt_data: false,
        }
    }

    pub fn value(&self, interp: &Mrb) -> Option<Value> {
        let rclass = self.rclass(interp)?;
        let module = unsafe { sys::mrb_sys_class_value(rclass) };
        Some(Value::new(interp, module))
    }

    pub fn data_type(&self) -> &sys::mrb_data_type {
        &self.data_type
    }

    pub fn mrb_value_is_rust_backed(&mut self, is_mrb_tt_data: bool) {
        self.is_mrb_tt_data = is_mrb_tt_data;
    }

    pub fn with_super_class(&mut self, super_class: Rc<RefCell<Self>>) {
        self.super_class = Some(super_class);
    }
}

impl ClassLike for Spec {
    fn add_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec) {
        let spec = method::Spec::new(method::Type::Instance, name, method, args);
        self.methods.insert(spec);
    }

    fn add_self_method(&mut self, name: &str, method: Method, args: sys::mrb_aspec) {
        let spec = method::Spec::new(method::Type::Class, name, method, args);
        self.methods.insert(spec);
    }

    fn cstring(&self) -> &CString {
        &self.cstring
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> Option<Parent> {
        self.parent.clone()
    }

    fn rclass(&self, interp: &Mrb) -> Option<*mut sys::RClass> {
        let mrb = interp.borrow().mrb;
        if let Some(ref parent) = self.parent {
            if let Some(parent) = parent.rclass(interp) {
                if unsafe { sys::mrb_class_defined_under(mrb, parent, self.cstring.as_ptr()) } == 0
                {
                    // parent exists and class is NOT defined under parent
                    None
                } else {
                    // parent exists class is defined under parent
                    Some(unsafe { sys::mrb_class_get_under(mrb, parent, self.cstring().as_ptr()) })
                }
            } else {
                // parent does not exist
                None
            }
        } else if unsafe { sys::mrb_class_defined(mrb, self.cstring.as_ptr()) } == 0 {
            // class does NOT exist in root namespace
            None
        } else {
            // class exists in root namespace
            Some(unsafe { sys::mrb_class_get(mrb, self.cstring().as_ptr()) })
        }
    }
}

impl fmt::Debug for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)?;
        if self.data_type.dfree.is_some() {
            write!(f, " -- with free func")?;
        }
        Ok(())
    }
}

impl fmt::Display for Spec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mruby class spec -- {}", self.fqname())
    }
}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
        self.parent().hash(state);
    }
}

impl Eq for Spec {}

impl PartialEq for Spec {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Define for Spec {
    fn define(&self, interp: &Mrb) -> Result<*mut sys::RClass, MrbError> {
        let mrb = interp.borrow().mrb;
        let super_class = if let Some(ref spec) = self.super_class {
            spec.borrow()
                .rclass(interp)
                .ok_or_else(|| MrbError::NotDefined(spec.borrow().fqname()))?
        } else {
            unsafe { (*mrb).object_class }
        };
        let rclass = if let Some(ref parent) = self.parent {
            let parent = parent
                .rclass(interp)
                .ok_or_else(|| MrbError::NotDefined(parent.fqname()))?;
            unsafe {
                sys::mrb_define_class_under(mrb, parent, self.cstring().as_ptr(), super_class)
            }
        } else {
            unsafe { sys::mrb_define_class(mrb, self.cstring().as_ptr(), super_class) }
        };
        for method in &self.methods {
            unsafe {
                method.define(&interp, rclass)?;
            }
        }
        // If a `Spec` defines a `Class` whose isntances own a pointer to a
        // Rust object, mark them as `MRB_TT_DATA`.
        if self.is_mrb_tt_data {
            unsafe {
                sys::mrb_sys_set_instance_tt(rclass, sys::mrb_vtype::MRB_TT_DATA);
            }
        }
        Ok(rclass)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::class::Spec;
    use crate::convert::TryFromMrb;
    use crate::def::{ClassLike, Define, Parent};
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::module;

    #[test]
    fn super_class() {
        let interp = Interpreter::create().expect("mrb init");
        let standard_error = Rc::new(RefCell::new(Spec::new("StandardError", None, None)));
        let spec = {
            let mut api = interp.borrow_mut();
            let spec = api.def_class::<()>("RustError", None, None);
            spec.borrow_mut()
                .with_super_class(Rc::clone(&standard_error));
            spec
        };
        spec.borrow().define(&interp).expect("class install");

        let result = interp
            .eval("RustError.new.is_a?(StandardError)")
            .expect("eval");
        let result = unsafe { bool::try_from_mrb(&interp, result).expect("convert") };
        assert!(result, "RustError instances are instance of StandardError");
        let result = interp.eval("RustError < StandardError").expect("eval");
        let result = unsafe { bool::try_from_mrb(&interp, result).expect("convert") };
        assert!(result, "RustError inherits from StandardError");
    }

    #[test]
    fn refcell_allows_mutable_class_specs_after_attached_as_parent() {
        struct BaseClass;
        struct SubClass;
        let interp = Interpreter::create().expect("mrb init");
        let (base, sub) = {
            let mut api = interp.borrow_mut();
            let base = api.def_class::<BaseClass>("BaseClass", None, None);
            let sub = api.def_class::<SubClass>("SubClass", None, None);
            sub.borrow_mut().with_super_class(Rc::clone(&base));
            (base, sub)
        };
        base.borrow().define(&interp).expect("def class");
        sub.borrow().define(&interp).expect("def class");
        {
            let api = interp.borrow();
            // this should not panic
            let _ = api.class_spec::<BaseClass>().unwrap().borrow_mut();
            let _ = api.class_spec::<SubClass>().unwrap().borrow_mut();
        }
    }

    #[test]
    fn rclass_for_undef_root_class() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = Spec::new("Foo", None, None);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_undef_nested_class() {
        let interp = Interpreter::create().expect("mrb init");
        let parent = module::Spec::new("Kernel", None);
        let parent = Parent::module(Rc::new(RefCell::new(parent)));
        let spec = Spec::new("Foo", Some(parent), None);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_root_class() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = Spec::new("StandardError", None, None);
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_class() {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .eval("module Foo; class Bar; end; end")
            .expect("eval");
        let parent = module::Spec::new("Foo", None);
        let parent = Parent::module(Rc::new(RefCell::new(parent)));
        let spec = Spec::new("Bar", Some(parent), None);
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_class_under_class() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("class Foo; class Bar; end; end").expect("eval");
        let parent = Spec::new("Foo", None, None);
        let parent = Parent::class(Rc::new(RefCell::new(parent)));
        let spec = Spec::new("Bar", Some(parent), None);
        assert!(spec.rclass(&interp).is_some());
    }
}
