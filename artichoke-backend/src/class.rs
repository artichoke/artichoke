use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::def::{ClassLike, Define, EnclosingRubyScope, Free, Method};
use crate::method;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub struct Spec {
    name: String,
    cstring: CString,
    data_type: sys::mrb_data_type,
    methods: HashSet<method::Spec>,
    enclosing_scope: Option<EnclosingRubyScope>,
    super_class: Option<Rc<RefCell<Spec>>>,
    is_mrb_tt_data: bool,
    memoized_rclass: RefCell<Option<*mut sys::RClass>>,
}

impl Spec {
    pub fn new<T>(name: T, enclosing_scope: Option<EnclosingRubyScope>, free: Option<Free>) -> Self
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
            enclosing_scope,
            super_class: None,
            is_mrb_tt_data: false,
            memoized_rclass: RefCell::new(None),
        }
    }

    pub fn new_instance(&self, interp: &Artichoke, args: &[Value]) -> Option<Value> {
        let rclass = self.rclass(interp)?;
        let args = args.iter().map(Value::inner).collect::<Vec<_>>();
        let arglen = Int::try_from(args.len()).unwrap_or_default();
        let value = unsafe {
            sys::mrb_obj_new(
                interp.borrow().mrb,
                rclass,
                arglen,
                args.as_ptr() as *const sys::mrb_value,
            )
        };
        Some(Value::new(interp, value))
    }

    pub fn value(&self, interp: &Artichoke) -> Option<Value> {
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

    fn enclosing_scope(&self) -> Option<EnclosingRubyScope> {
        self.enclosing_scope.clone()
    }

    fn rclass(&self, interp: &Artichoke) -> Option<*mut sys::RClass> {
        if let Some(rclass) = *self.memoized_rclass.borrow() {
            return Some(rclass);
        }
        let mrb = interp.borrow().mrb;
        if let Some(ref scope) = self.enclosing_scope {
            if let Some(scope) = scope.rclass(interp) {
                if unsafe { sys::mrb_class_defined_under(mrb, scope, self.cstring.as_ptr()) } != 0 {
                    // Enclosing scope exists Class IS defined under the
                    // enclosing scope.
                    self.memoized_rclass.replace(Some(unsafe {
                        sys::mrb_class_get_under(mrb, scope, self.cstring().as_ptr())
                    }));
                }
            }
        } else if unsafe { sys::mrb_class_defined(mrb, self.cstring.as_ptr()) } != 0 {
            // Class exists in root scope.
            self.memoized_rclass.replace(Some(unsafe {
                sys::mrb_class_get(mrb, self.cstring().as_ptr())
            }));
        }
        if let Some(rclass) = *self.memoized_rclass.borrow() {
            Some(rclass)
        } else {
            None
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
        write!(f, "artichoke class spec -- {}", self.fqname())
    }
}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
        self.enclosing_scope().hash(state);
    }
}

impl Eq for Spec {}

impl PartialEq for Spec {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Define for Spec {
    fn define(&self, interp: &Artichoke) -> Result<*mut sys::RClass, ArtichokeError> {
        let mrb = interp.borrow().mrb;
        let super_class = if let Some(ref spec) = self.super_class {
            spec.borrow()
                .rclass(interp)
                .ok_or_else(|| ArtichokeError::NotDefined(spec.borrow().fqname()))?
        } else {
            unsafe { (*mrb).object_class }
        };
        let rclass = if let Some(rclass) = self.rclass(interp) {
            rclass
        } else if let Some(ref scope) = self.enclosing_scope {
            let scope = scope
                .rclass(interp)
                .ok_or_else(|| ArtichokeError::NotDefined(scope.fqname()))?;
            unsafe { sys::mrb_define_class_under(mrb, scope, self.cstring().as_ptr(), super_class) }
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
        self.memoized_rclass.replace(Some(rclass));
        Ok(rclass)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::class::Spec;
    use crate::convert::TryConvert;
    use crate::def::{ClassLike, Define, EnclosingRubyScope};
    use crate::eval::Eval;
    use crate::module;

    #[test]
    fn super_class() {
        let interp = crate::interpreter().expect("init");
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
        let result = unsafe { bool::try_convert(&interp, result).expect("convert") };
        assert!(result, "RustError instances are instance of StandardError");
        let result = interp.eval("RustError < StandardError").expect("eval");
        let result = unsafe { bool::try_convert(&interp, result).expect("convert") };
        assert!(result, "RustError inherits from StandardError");
    }

    #[test]
    fn refcell_allows_mutable_class_specs_after_attached_as_enclosing_scope() {
        struct BaseClass;
        struct SubClass;
        let interp = crate::interpreter().expect("init");
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
        let interp = crate::interpreter().expect("init");
        let spec = Spec::new("Foo", None, None);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_undef_nested_class() {
        let interp = crate::interpreter().expect("init");
        let scope = module::Spec::new("Kernel", None);
        let scope = EnclosingRubyScope::module(Rc::new(RefCell::new(scope)));
        let spec = Spec::new("Foo", Some(scope), None);
        assert!(spec.rclass(&interp).is_none());
    }

    #[test]
    fn rclass_for_root_class() {
        let interp = crate::interpreter().expect("init");
        let spec = Spec::new("StandardError", None, None);
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_class() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval("module Foo; class Bar; end; end")
            .expect("eval");
        let spec = module::Spec::new("Foo", None);
        let spec = EnclosingRubyScope::module(Rc::new(RefCell::new(spec)));
        let spec = Spec::new("Bar", Some(spec), None);
        assert!(spec.rclass(&interp).is_some());
    }

    #[test]
    fn rclass_for_nested_class_under_class() {
        let interp = crate::interpreter().expect("init");
        interp.eval("class Foo; class Bar; end; end").expect("eval");
        let spec = Spec::new("Foo", None, None);
        let spec = EnclosingRubyScope::class(Rc::new(RefCell::new(spec)));
        let spec = Spec::new("Bar", Some(spec), None);
        assert!(spec.rclass(&interp).is_some());
    }
}
