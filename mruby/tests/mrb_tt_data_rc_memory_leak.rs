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
//! test tests::rust_backed_mrb_value_smart_pointer_leak ... FAILED
//!
//! failures:
//!
//! ---- tests::rust_backed_mrb_value_smart_pointer_leak stdout ----
//! thread 'tests::rust_backed_mrb_value_smart_pointer_leak' panicked at 'Plausible memory leak!
//! After 2000 iterations, usage before: 1228800, usage after: 4094840832', mruby/tests/mrb_tt_data_rc_memory_leak.rs:75:9
//! note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
//!
//!
//! failures:
//!     tests::rust_backed_mrb_value_smart_pointer_leak
//!
//! test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
//! ```

use mruby::def::{ClassLike, Define};
use mruby::file::MrbFile;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::interpreter_or_raise;
use mruby::sys;
use std::cell::RefCell;
use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: String,
}

impl MrbFile for Container {
    fn require(interp: Mrb) {
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
                let api = interp.borrow_mut();

                let string = mem::uninitialized::<*const std::os::raw::c_char>();
                let argspec = CString::new(sys::specifiers::CSTRING).expect("argspec");
                sys::mrb_get_args(mrb, argspec.as_ptr(), &string);
                let string = CStr::from_ptr(string).to_string_lossy().to_string();
                let cont = Container { inner: string };
                let data = Rc::new(RefCell::new(cont));
                let ptr = mem::transmute::<Rc<RefCell<Container>>, *mut c_void>(data);

                let spec = api.class_spec::<Container>();
                sys::mrb_sys_data_init(&mut slf, ptr, spec.data_type());

                slf
            }
        }

        {
            let mut api = interp.borrow_mut();
            api.def_class::<Container>("Container", None, Some(free));
            let spec = api.class_spec_mut::<Self>();
            spec.add_method("initialize", initialize, sys::mrb_args_req(1));
            spec.mrb_value_is_rust_backed(true);
        }
        let api = interp.borrow();
        let spec = api.class_spec::<Self>();
        spec.define(Rc::clone(&interp)).expect("class install");
    }
}

#[cfg(test)]
mod tests {
    use mruby::interpreter::MrbApi;

    use super::*;

    const LEAK_TOLERANCE: i64 = 1024 * 1024 * 10;
    const ITERATIONS: usize = 2000;

    fn check_leaks<F>(mut execute: F)
    where
        F: FnMut() -> (),
    {
        let start_mem = resident_memsize();
        for _ in 0..ITERATIONS {
            execute();
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
        let mut out: libc::rusage = unsafe { mem::zeroed() };
        assert!(unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut out) } == 0);
        out.ru_maxrss
    }

    #[test]
    fn rust_backed_mrb_value_smart_pointer_leak() {
        check_leaks(|| {
            let mut interp = Interpreter::create().expect("mrb init");
            interp.def_file_for_type::<_, Container>("container");

            let code = "require 'container'; Container.new('a' * 1024 * 1024)";
            let result = interp.eval(code);
            assert_eq!(true, result.is_ok());
            drop(interp);
        })
    }
}
