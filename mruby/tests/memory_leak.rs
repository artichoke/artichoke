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
//! This test fails when reverting commit
//! `34ee3ddc1c5f4eb1d20f19dd772b0ca348391b2f` with a fairly massive leak and
//! the following message:
//!
//! ```txt
//! running 1 test
//! test tests::check_leaks ... FAILED
//!
//! failures:
//!
//! ---- tests::check_leaks stdout ----
//! thread 'tests::check_leaks' panicked at 'Plausible memory leak!
//! After 2000 iterations, usage before: 1228800, usage after: 4094840832', mruby/tests/memory_leak.rs:75:9
//! note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
//!
//!
//! failures:
//!     tests::check_leaks
//!
//! test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
//! ```

use mruby::*;
use mruby_sys::*;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: String,
}

impl MrbFile for Container {
    fn require(interp: Mrb) {
        extern "C" fn free(_mrb: *mut mrb_state, data: *mut ::std::ffi::c_void) {
            unsafe {
                // Implictly dropped by going out of scope
                let _ = std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<Container>>>(data);
            }
        }

        extern "C" fn initialize(mrb: *mut mrb_state, mut slf: mrb_value) -> mrb_value {
            unsafe {
                let string = std::mem::uninitialized::<*const std::os::raw::c_char>();
                let argspec = CString::new(specifiers::CSTRING).expect("argspec");
                mrb_get_args(mrb, argspec.as_ptr(), &string);
                let string = CStr::from_ptr(string).to_string_lossy().to_string();
                let cont = Container { inner: string };
                let data = Rc::new(RefCell::new(cont));
                let ptr =
                    std::mem::transmute::<Rc<RefCell<Container>>, *mut std::ffi::c_void>(data);

                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                let mut api = interp.borrow_mut();
                let data_type = api.get_or_create_data_type("Container", Some(free));
                mrb_sys_data_init(&mut slf, ptr, data_type);

                slf
            }
        }

        unsafe {
            let mrb = { interp.borrow().mrb() };
            // this `CString` needs to stay in scope for the life of the mruby
            // interpreter, otherwise `mrb_close` will segfault.
            let class = CString::new("Container").expect("Container class");
            let mrb_class = mrb_define_class(mrb, class.as_ptr(), (*mrb).object_class);
            mrb_sys_set_instance_tt(mrb_class, mrb_vtype::MRB_TT_DATA);

            let initialize_method = CString::new("initialize").expect("initialize method");
            mrb_define_method(
                mrb,
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                mrb_args_req(1),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEAK_TOLERANCE: i64 = 1024 * 1024 * 10;
    const ITERATIONS: usize = 2000;

    #[test]
    fn check_leaks() {
        let start_mem = resident_memsize();
        for _ in 0..ITERATIONS {
            eval();
        }
        let end_mem = resident_memsize();
        assert!(
            end_mem <= start_mem + LEAK_TOLERANCE,
            "Plausible memory leak!\nAfter {} iterations, usage before: {}, usage after: {}",
            ITERATIONS,
            start_mem,
            end_mem
        );
    }

    fn resident_memsize() -> i64 {
        let mut out: libc::rusage = unsafe { std::mem::zeroed() };
        assert!(unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut out) } == 0);
        out.ru_maxrss
    }

    fn eval() {
        let interp = Interpreter::create().expect("mrb init");
        {
            let mut borrow = interp.borrow_mut();
            borrow.def_file_for_type::<_, Container>("container");
        }

        unsafe {
            let (mrb, context) = { (interp.borrow().mrb(), interp.borrow().ctx()) };
            // Create a Container with a 1MB String
            let code = "require 'container'; Container.new('a' * 1024 * 1024)";
            mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let api = interp.borrow();

            let exception = Value::new(mrb_sys_get_current_exception(api.mrb()));
            let exception = <Option<String>>::try_from_mrb(&api, exception).expect("convert");
            assert_eq!(None, exception);
        }
        drop(interp);
    }
}
