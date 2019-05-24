//! This integration test checks for memory leaks that stem from not
//! deallocating `MrbApi` objects, embedded `mrb_value` data pointers, and
//! linked Rust data.
//!
//! The test exposes a `Container` class to mruby which is initialized with a
//! 1MB `String`. The test creates a new mruby interpreter, loads the Container
//! class into the interpreter, and initializes one instance `ITERATIONS` times.
//!
//! If resident memory increases more than 10MB during the test, we likely are
//! leaking memory.
//!
//! This test fails before commit
//! `34ee3ddc1c5f4eb1d20f19dd772b0ca348391b2f` with a fairly massive leak.

#[macro_use]
extern crate mruby;

use mruby::def::{ClassLike, Define};
use mruby::eval::MrbEval;
use mruby::file::MrbFile;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::load::MrbLoadSources;
use mruby::sys;
use mruby::MrbError;
use std::cell::RefCell;
use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::rc::Rc;

mod leak;

use leak::LeakDetector;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 10;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: String,
}

impl MrbFile for Container {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        extern "C" fn free(_mrb: *mut sys::mrb_state, data: *mut c_void) {
            unsafe {
                // Implictly dropped by going out of scope
                let _ = mem::transmute::<*mut c_void, Rc<RefCell<Container>>>(data);
            }
        }

        extern "C" fn initialize(
            mrb: *mut sys::mrb_state,
            mut slf: sys::mrb_value,
        ) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);

                let string = mem::uninitialized::<*const std::os::raw::c_char>();
                let argspec = CString::new(sys::specifiers::CSTRING).expect("argspec");
                sys::mrb_get_args(mrb, argspec.as_ptr(), &string);
                let string = CStr::from_ptr(string).to_string_lossy().to_string();
                let cont = Container { inner: string };
                let data = Rc::new(RefCell::new(cont));
                let ptr = mem::transmute::<Rc<RefCell<Container>>, *mut c_void>(data);

                let spec = class_spec_or_raise!(interp, Container);
                let borrow = spec.borrow();
                sys::mrb_sys_data_init(&mut slf, ptr, borrow.data_type());

                slf
            }
        }

        let spec = {
            let mut api = interp.borrow_mut();
            let spec = api.def_class::<Self>("Container", None, Some(free));
            spec.borrow_mut()
                .add_method("initialize", initialize, sys::mrb_args_req(1));
            spec.borrow_mut().mrb_value_is_rust_backed(true);
            spec
        };
        spec.borrow().define(&interp)?;
        Ok(())
    }
}

#[test]
fn rust_backed_mrb_value_smart_pointer_leak() {
    LeakDetector::new("smart pointer", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let interp = Interpreter::create().expect("mrb init");
        interp
            .def_file_for_type::<_, Container>("container")
            .expect("def file");

        let code = "require 'container'; Container.new('a' * 1024 * 1024)";
        let result = interp.eval(code);
        assert_eq!(true, result.is_ok());
        drop(interp);
    });
}
