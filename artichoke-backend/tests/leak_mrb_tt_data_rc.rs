#![deny(clippy::all)]
#![deny(clippy::pedantic)]

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

use artichoke_backend::extn::prelude::*;

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

unsafe extern "C" fn container_initialize(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let inner = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let inner = Value::new(&guard, inner);
    let inner = inner.try_into_mut::<String>(&mut guard).unwrap_or_default();
    let container = Container { inner };
    let result = container.try_into_ruby(&mut guard, Some(slf));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

impl File for Container {
    type Artichoke = Artichoke;

    type Error = Exception;

    fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
        let spec = class::Spec::new("Container", None, Some(def::rust_data_free::<Self>))?;
        class::Builder::for_spec(interp, &spec)
            .value_is_rust_object()
            .add_method("initialize", container_initialize, sys::mrb_args_req(1))?
            .define()?;
        interp.def_class::<Self>(spec)?;
        Ok(())
    }
}

#[test]
fn rust_backed_mrb_value_smart_pointer_leak() {
    // dummy interp, we will instantiate a fresh interp for each detector loop.
    let mut interp = artichoke_backend::interpreter().unwrap();
    leak::Detector::new("smart pointer", &mut interp)
        .with_iterations(ITERATIONS)
        .with_tolerance(LEAK_TOLERANCE)
        .check_leaks_with_finalizer(
            |_| {
                let mut interp = artichoke_backend::interpreter().unwrap();
                interp
                    .def_file_for_type::<_, Container>("container")
                    .unwrap();

                let code = b"require 'container'; Container.new('a' * 1024 * 1024)";
                let result = interp.eval(code);
                assert_eq!(true, result.is_ok());
                interp.close();
            },
            |interp| interp.full_gc(),
        );
}
