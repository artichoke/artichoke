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
extern crate artichoke_backend;

use artichoke_backend::convert::{Convert, RustBackedValue, TryConvert};
use artichoke_backend::def::{rust_data_free, ClassLike, Define};
use artichoke_backend::eval::MrbEval;
use artichoke_backend::file::MrbFile;
use artichoke_backend::load::MrbLoadSources;
use artichoke_backend::sys;
use artichoke_backend::value::Value;
use artichoke_backend::{Mrb, MrbError};
use std::io::Write;
use std::mem;

mod leak;

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
                let inner = <mem::MaybeUninit<sys::mrb_value>>::uninit();
                let mut argspec = vec![];
                // TODO: use a constant argspec, see GH-174.
                argspec
                    .write_all(sys::specifiers::OBJECT.as_bytes())
                    .map_err(|_| MrbError::ArgSpec)?;
                argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(interp.borrow().mrb, argspec.as_ptr() as *const i8, &inner);
                let inner = Value::new(interp, inner.assume_init());
                let inner =
                    String::try_from_mrb(&interp, inner).map_err(MrbError::ConvertToRust)?;
                Ok(Self { inner })
            }
        }

        let interp = unwrap_interpreter!(mrb);
        Args::extract(&interp)
            .and_then(|args| {
                let container = Self { inner: args.inner };
                container.try_into_ruby(&interp, Some(slf))
            })
            .unwrap_or_else(|_| Value::from_mrb(&interp, None::<Value>))
            .inner()
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
    leak::Detector::new("smart pointer", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let interp = artichoke_backend::interpreter().expect("mrb init");
        interp
            .def_file_for_type::<_, Container>("container")
            .expect("def file");

        let code = "require 'container'; Container.new('a' * 1024 * 1024)";
        let result = interp.eval(code);
        assert_eq!(true, result.is_ok());
        drop(interp);
    });
}
