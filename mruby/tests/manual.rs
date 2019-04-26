#[macro_use]
extern crate log;

use mruby::*;
use mruby_sys::*;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: i64,
}

impl MrbFile for Container {
    fn require(interp: Mrb) {
        extern "C" fn free(_mrb: *mut mrb_state, data: *mut ::std::ffi::c_void) {
            unsafe {
                debug!("preparing to free Container instance");
                // Implictly dropped by going out of scope
                let inner =
                    std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<Container>>>(data);
                debug!("freeing Container instance: {}", inner.borrow().inner);
            }
        }

        extern "C" fn initialize(mrb: *mut mrb_state, mut slf: mrb_value) -> mrb_value {
            unsafe {
                let int = std::mem::uninitialized::<mrb_int>();
                let argspec = CString::new(specifiers::INTEGER).expect("argspec");
                mrb_get_args(mrb, argspec.as_ptr(), &int);
                let cont = Container { inner: int };
                let data = Rc::new(RefCell::new(cont));
                debug!("Storing `Container` refcell in self instance: {:?}", data);
                let ptr =
                    std::mem::transmute::<Rc<RefCell<Container>>, *mut std::ffi::c_void>(data);

                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                debug!(
                    "interpreter strong ref count = {}",
                    Rc::strong_count(&interp)
                );
                let mut api = interp.borrow_mut();
                let data_type = api.get_or_create_data_type("Container", Some(free));
                mrb_sys_data_init(&mut slf, ptr, data_type);

                slf
            }
        }

        extern "C" fn value(mrb: *mut mrb_state, slf: mrb_value) -> mrb_value {
            unsafe {
                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                let mut api = interp.borrow_mut();
                let data_type = api.get_or_create_data_type("Container", Some(free));

                debug!(
                    "pulled mrb_data_type from user data with class: {:?}",
                    CStr::from_ptr((*data_type).struct_name).to_string_lossy()
                );
                let ptr = mrb_data_get_ptr(mrb, slf, data_type);
                let data =
                    std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<Container>>>(ptr);
                let clone = Rc::clone(&data);
                let cont = clone.borrow();

                let value = unwrap_or_raise!(api, Value::try_from_mrb(&api, cont.inner));
                std::mem::forget(data);
                value
            }
        }

        unsafe {
            let mrb = { interp.borrow().mrb() };
            // this `CString` needs to stay in scope for the life of the mruby
            // interpreter, otherwise `mrb_close` will segfault.
            let class = CString::new("Container").expect("Container class");
            let mrb_class = mrb_define_class(mrb, class.as_ptr(), (*mrb).object_class);
            mrb_sys_set_instance_tt(mrb_class, mrb_vtype_MRB_TT_DATA);

            let initialize_method = CString::new("initialize").expect("initialize method");
            mrb_define_method(
                mrb,
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                // TODO: expose arg count c functions
                mrb_args_req(1),
            );

            let value_method = CString::new("value").expect("value method");
            mrb_define_method(
                mrb,
                mrb_class,
                value_method.as_ptr(),
                Some(value),
                // TODO: expose arg count c functions
                mrb_args_none(),
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

        let interp = Interpreter::create().expect("mrb init");
        {
            let mut borrow = interp.borrow_mut();
            borrow.def_file_for_type::<_, Container>("container");
        }

        unsafe {
            let (mrb, context) = { (interp.borrow().mrb(), interp.borrow().ctx()) };
            let code = "require 'container'; Container.new(15).value";
            let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let api = interp.borrow();
            let result = Value::new(result);
            let exception = Value::new(mrb_sys_get_current_exception(api.mrb()));

            assert_eq!(exception.ruby_type(), Ruby::Nil);
            let exception = <Option<String>>::try_from_mrb(&api, exception).expect("convert");
            assert_eq!(None, exception);
            let cint = i64::try_from_mrb(&api, result).expect("convert");
            assert_eq!(cint, 15);
        }
        drop(interp);
    }
}
