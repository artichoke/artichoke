use mruby::*;
use mruby_sys::*;
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
#[repr(C)]
struct Container {
    inner: i64,
}

impl File for Container {
    fn require(mrb: Mrb) {
        extern "C" fn initialize(mrb: *mut mrb_state, mut slf: mrb_value) -> mrb_value {
            unsafe {
                let class = CString::new("Container").expect("Container class");
                let cont = Container::default();
                let mut rc = Rc::new(RefCell::new(cont));
                let ptr: *mut std::ffi::c_void = &mut rc as *mut _ as *mut std::ffi::c_void;

                let mut data_type = mrb_data_type {
                    struct_name: class.as_ptr(),
                    dfree: None,
                };

                mrb_sys_data_init(&mut slf, ptr, &mut data_type);
                slf
            }
        }

        extern "C" fn value(mrb: *mut mrb_state, slf: mrb_value) -> mrb_value {
            unsafe {
                let inner = 15;
                match Value::try_from_mrb(mrb, inner) {
                    Ok(value) => value.inner(),
                    Err(err) => {
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

            let initialize_method = CString::new("initialize").expect("initialize method");
            mrb_define_method(
                mrb.inner().expect("mrb open"),
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                0,
            );

            let value_method = CString::new("value").expect("value method");
            mrb_define_method(
                mrb.inner().expect("mrb open"),
                mrb_class,
                value_method.as_ptr(),
                Some(value),
                0,
            );
        }
    }
}

fn main() {
    let mrb = Mrb::new().expect("mrb init");
    //Container::require(mrb.clone());

    unsafe {
        let context = mrbc_context_new(mrb.inner().expect("mrb open"));
        let code = "Container.new.value";
        let result = mrb_load_nstring_cxt(
            mrb.inner().expect("mrb open"),
            code.as_ptr() as *const i8,
            code.len(),
            context,
        );
        assert_eq!(mrb_sys_fixnum_to_cint(result), 15);

        mrbc_context_free(mrb.inner().expect("mrb open"), context);
    }
}
