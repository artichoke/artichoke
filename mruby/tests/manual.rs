use log::{debug, info, warn};
use mruby::*;
use mruby_sys::*;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
#[repr(C)]
struct Container {
    inner: i64,
}

impl File for Container {
    fn require(mrb: &Mrb) {
        extern "C" fn free(_mrb: *mut mrb_state, mut data: *mut ::std::ffi::c_void) {
            unsafe {
                let inner = &mut data as *mut _ as *mut Rc<RefCell<Container>>;
                debug!("freeing Container instance: {}", (*inner).borrow().inner);
                // TODO: Find out what the right thing to do here is
                #[allow(clippy::drop_copy)]
                std::mem::drop(inner);
            }
        }

        extern "C" fn initialize(mrb: *mut mrb_state, mut slf: mrb_value) -> mrb_value {
            unsafe {
                let cont = Container { inner: 15 };
                let data = Rc::new(RefCell::new(cont));
                debug!("Storing this data in self instance: {:?}", data);

                let ptr: *mut std::ffi::c_void = std::mem::transmute(data);
                let data_type = (*mrb).ud as *mut _ as *mut mrb_data_type;
                mrb_sys_data_init(&mut slf, ptr, data_type);

                slf
            }
        }

        extern "C" fn value(mrb: *mut mrb_state, slf: mrb_value) -> mrb_value {
            unsafe {
                let ptr = (*mrb).ud;
                // make sure we don't panic if we have a null pointer
                if ptr.is_null() {
                    warn!("Got a null pointer from mrb_state->ud");
                    info!("Attempting to recover by returning nil");
                    return Value::try_from_mrb(mrb, None as Option<bool>)
                        .expect("nil")
                        .inner();
                }

                let data_type = ptr as *const mrb_data_type;
                debug!(
                    "pulled mrb_data_type from user data with class: {:?}",
                    // TODO: figure out why this is poiinting to garbage
                    CStr::from_ptr((*data_type).struct_name).to_string_lossy()
                );
                let mut ptr = mrb_data_get_ptr(mrb, slf, data_type);
                let data = &mut ptr as *mut _ as *mut Rc<RefCell<Container>>;

                match Value::try_from_mrb(mrb, (*data).borrow().inner) {
                    Ok(value) => value.inner(),
                    Err(err) => {
                        // could not convert Container->inner to mrb_value.
                        // This should be unreachable since inner is an i64 and
                        // conversion between i64 and Value always succeeds.
                        let eclass = CString::new("RuntimeError").expect("eclass");
                        let message = CString::new(format!("{}", err)).expect("message");
                        mrb_sys_raise(mrb, eclass.as_ptr(), message.as_ptr());
                        Value::try_from_mrb(mrb, None as Option<i64>)
                            .expect("nil")
                            .inner()
                    }
                }
            }
        }

        unsafe {
            let class = CString::new("Container").expect("Container class");
            let mrb_class = mrb_define_class(
                mrb.inner().expect("mrb open"),
                class.as_ptr(),
                (*mrb.inner().expect("mrb open")).object_class,
            );
            mrb_sys_set_instance_tt(mrb_class, mrb_vtype_MRB_TT_DATA);

            let mut data_type = mrb_data_type {
                struct_name: class.as_ptr(),
                dfree: Some(free),
            };
            let ptr = &mut data_type as *mut _ as *mut std::ffi::c_void;
            (*mrb.inner().expect("mrb open")).ud = ptr;

            let initialize_method = CString::new("initialize").expect("initialize method");
            mrb_define_method(
                mrb.inner().expect("mrb open"),
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                // TODO: expose arg count c functions
                0,
            );

            let value_method = CString::new("value").expect("value method");
            mrb_define_method(
                mrb.inner().expect("mrb open"),
                mrb_class,
                value_method.as_ptr(),
                Some(value),
                // TODO: expose arg count c functions
                0,
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

        let mrb = Mrb::new().expect("mrb init");
        Container::require(&mrb);

        unsafe {
            let context = mrbc_context_new(mrb.inner().expect("mrb open"));
            let code = "Container.new.value";
            let result = mrb_load_nstring_cxt(
                mrb.inner().expect("mrb open"),
                code.as_ptr() as *const i8,
                code.len(),
                context,
            );
            let result = Value::new(result);
            let exception = Value::new(mrb_sys_get_current_exception(
                mrb.inner().expect("mrb open"),
            ));

            assert_eq!(
                "NilClass<nil>",
                exception.to_s_debug(mrb.inner().expect("mrb open"))
            );
            let exception =
                <Option<String>>::try_from_mrb(mrb.inner().expect("mrb open"), exception)
                    .expect("convert");
            assert_eq!(None, exception);
            let cint = i64::try_from_mrb(mrb.inner().expect("mrb open"), result).expect("convert");
            assert_eq!(cint, 15);

            mrbc_context_free(mrb.inner().expect("mrb open"), context);
        }
    }
}
