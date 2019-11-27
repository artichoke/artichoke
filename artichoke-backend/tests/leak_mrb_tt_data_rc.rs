#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

//! This integration test checks for memory leaks that stem from not
//! deallocating `ArtichokeApi` objects, embedded `mrb_value` data pointers, and
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

use artichoke_backend::convert::RustBackedValue;
use artichoke_backend::def::{rust_data_free, ClassLike, Define};
use artichoke_backend::extn::core::exception::{self, Fatal, RubyException};
use artichoke_backend::sys;
use artichoke_backend::value::Value;
use artichoke_backend::{Artichoke, ArtichokeError};
use artichoke_core::eval::Eval;
use artichoke_core::file::File;
use artichoke_core::load::LoadSources;
use artichoke_core::value::Value as _;

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 15;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: String,
}

impl RustBackedValue for Container {
    fn ruby_type_name() -> &'static str {
        "Container"
    }
}

impl Container {
    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let inner = mrb_get_args!(mrb, required = 1);
        let interp = unwrap_interpreter!(mrb);
        let inner = Value::new(&interp, inner);
        let inner = inner.try_into::<&str>().unwrap_or_else(|_| "").to_owned();
        let container = Self { inner };
        let result: Result<Value, Box<dyn RubyException>> =
            if let Ok(result) = container.try_into_ruby(&interp, Some(slf)) {
                Ok(result)
            } else {
                Err(Box::new(Fatal::new(
                    &interp,
                    "Unable to intialize Container Ruby Value",
                )))
            };
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }
}

impl File for Container {
    type Artichoke = Artichoke;

    fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
        let spec = {
            let mut api = interp.0.borrow_mut();
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
        let interp = artichoke_backend::interpreter().expect("init");
        interp
            .def_file_for_type::<Container>(b"container")
            .expect("def file");

        let code = b"require 'container'; Container.new('a' * 1024 * 1024)";
        let result = interp.eval(code);
        assert_eq!(true, result.is_ok());
        interp.close();
    });
}
