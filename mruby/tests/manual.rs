#[macro_use]
extern crate log;

use mruby::*;
use std::cell::RefCell;
use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: i64,
}

impl MrbFile for Container {
    fn require(interp: Mrb) {
        extern "C" fn free(_mrb: *mut sys::mrb_state, data: *mut c_void) {
            unsafe {
                debug!("preparing to free Container instance");
                // Implictly dropped by going out of scope
                let inner = mem::transmute::<*mut c_void, Rc<RefCell<Container>>>(data);
                debug!(
                    "freeing Container instance with value: {}",
                    inner.borrow().inner
                );
            }
        }

        extern "C" fn initialize(
            mrb: *mut sys::mrb_state,
            mut slf: sys::mrb_value,
        ) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);
                let mut api = interp.borrow_mut();

                let int = mem::uninitialized::<sys::mrb_int>();
                let argspec = CString::new(sys::specifiers::INTEGER).expect("argspec");
                sys::mrb_get_args(mrb, argspec.as_ptr(), &int);
                let cont = Container { inner: int };
                let data = Rc::new(RefCell::new(cont));
                debug!("Storing `Container` refcell in self instance: {:?}", data);
                let ptr = mem::transmute::<Rc<RefCell<Container>>, *mut c_void>(data);

                debug!(
                    "interpreter strong ref count = {}",
                    Rc::strong_count(&interp)
                );
                let data_type = api.get_or_create_data_type("Container", Some(free));
                sys::mrb_sys_data_init(&mut slf, ptr, data_type);

                slf
            }
        }

        extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);
                let mut api = interp.borrow_mut();
                let data_type = api.get_or_create_data_type("Container", Some(free));

                debug!(
                    "pulled mrb_data_type from user data with class: {:?}",
                    CStr::from_ptr((*data_type).struct_name).to_string_lossy()
                );
                let ptr = sys::mrb_data_get_ptr(mrb, slf, data_type);
                let data = mem::transmute::<*mut c_void, Rc<RefCell<Container>>>(ptr);
                let clone = Rc::clone(&data);
                let cont = clone.borrow();

                let value = unwrap_or_raise!(interp, Value::try_from_mrb(&interp, cont.inner));
                mem::forget(data);
                value
            }
        }

        unsafe {
            // this `CString` needs to stay in scope for the life of the mruby
            // interpreter, otherwise `mrb_close` will segfault.
            let class = CString::new("Container").expect("Container class");
            let mrb_class = sys::mrb_define_class(
                interp.borrow().mrb,
                class.as_ptr(),
                (*interp.borrow().mrb).object_class,
            );
            sys::mrb_sys_set_instance_tt(mrb_class, sys::mrb_vtype::MRB_TT_DATA);

            let initialize_method = CString::new("initialize").expect("initialize method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                sys::mrb_args_req(1),
            );

            let value_method = CString::new("value").expect("value method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                value_method.as_ptr(),
                Some(value),
                sys::mrb_args_none(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_rust_backed_ruby_class() {
        env_logger::Builder::from_env("MRUBY_LOG").init();

        let mut interp = Interpreter::create().expect("mrb init");
        interp.def_file_for_type::<_, Container>("container");

        let code = "require 'container'; Container.new(15).value";
        let result = interp.eval(code).expect("no exceptions");
        let cint = unsafe { i64::try_from_mrb(&interp, result).expect("convert") };
        assert_eq!(cint, 15);

        drop(interp);
    }
}
