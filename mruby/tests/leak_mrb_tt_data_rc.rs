#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

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

use mruby::convert::{RustBackedValue, TryFromMrb};
use mruby::def::{rust_data_free, ClassLike, Define};
use mruby::eval::MrbEval;
use mruby::file::MrbFile;
use mruby::interpreter::{Interpreter, Mrb, MrbApi};
use mruby::load::MrbLoadSources;
use mruby::sys;
use mruby::value::Value;
use mruby::MrbError;
use std::io::Write;
use std::mem;

mod leak;

use leak::LeakDetector;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 10;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: String,
}

impl RustBackedValue for Container {}

impl Container {
    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        struct Args {
            inner: String,
        }

        impl Args {
            unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
                let inner = mem::uninitialized::<sys::mrb_value>();
                let mut argspec = vec![];
                argspec
                    .write_all(sys::specifiers::OBJECT.as_bytes())
                    .map_err(|_| MrbError::ArgSpec)?;
                argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(interp.borrow().mrb, argspec.as_ptr() as *const i8, &inner);
                let inner = Value::new(interp, inner);
                let inner =
                    String::try_from_mrb(&interp, inner).map_err(MrbError::ConvertToRust)?;
                Ok(Self { inner })
            }
        }

        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, Args::extract(&interp), interp.nil().inner());

        let container = Self { inner: args.inner };
        unwrap_value_or_raise!(interp, container.try_into_ruby(&interp, Some(slf)))
    }
}
impl MrbFile for Container {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        let spec = {
            let mut api = interp.borrow_mut();
            let spec = api.def_class::<Self>("Container", None, Some(rust_data_free::<Self>));
            spec.borrow_mut()
                .add_method("initialize", Self::initialize, sys::mrb_args_req(1));
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
