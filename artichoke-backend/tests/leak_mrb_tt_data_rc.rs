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

use artichoke_backend::class;
use artichoke_backend::convert::RustBackedValue;
use artichoke_backend::def;
use artichoke_backend::exception::{self, Exception, RubyException};
use artichoke_backend::extn::core::exception::Fatal;
use artichoke_backend::sys;
use artichoke_backend::value::Value;
use artichoke_backend::{Artichoke, Eval, File, LoadSources, ValueLike};

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

    type Error = Exception;

    fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
        let spec = class::Spec::new("Container", None, Some(def::rust_data_free::<Self>))?;
        class::Builder::for_spec(interp, &spec)
            .value_is_rust_object()
            .add_method("initialize", Self::initialize, sys::mrb_args_req(1))?
            .define()?;
        interp.0.borrow_mut().def_class::<Self>(spec);
        Ok(())
    }
}

#[test]
fn rust_backed_mrb_value_smart_pointer_leak() {
    leak::Detector::new("smart pointer", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let mut interp = artichoke_backend::interpreter().expect("init");
        interp
            .def_file_for_type::<Container>(b"container")
            .expect("def file");

        let code = b"require 'container'; Container.new('a' * 1024 * 1024)";
        let result = interp.eval(code);
        assert_eq!(true, result.is_ok());
        interp.close();
    });
}
