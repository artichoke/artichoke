// These integration tests check for memory leaks that stem from not
// deallocating Rust objects that are embedded into `mrb_value` data pointers,
// and the linked Rust-native structs.
//
// The test exposes a `Container` class to mruby which is initialized with a
// 1MB `String`. The test creates a new mruby interpreter, loads the Container
// class into the interpreter, and initializes one instance `ITERATIONS` times.
//
//
// This test fails before commit
// `34ee3ddc1c5f4eb1d20f19dd772b0ca348391b2f` with a fairly massive leak.

use artichoke_backend::extn::prelude::*;

const ITERATIONS: usize = 100;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Container(String);

impl HeapAllocatedData for Container {
    const RUBY_TYPE: &'static str = "Container";
}

unsafe extern "C-unwind" fn container_initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let inner = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let slf = Value::from(slf);
    let inner = Value::from(inner);
    let inner = inner.try_convert_into_mut::<String>(&mut guard).unwrap_or_default();
    let container = Container(inner);
    let result = Container::box_into_value(container, slf, &mut guard);
    match result {
        Ok(value) => value.into(),
        Err(exception) => error::raise(guard, exception),
    }
}

impl File for Container {
    type Artichoke = Artichoke;

    type Error = Error;

    fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
        let spec = class::Spec::new(
            "Container",
            qed::const_cstr_from_str!("Container\0"),
            None,
            Some(def::box_unbox_free::<Self>),
        )?;
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
    for _ in 0..ITERATIONS {
        let mut interp = artichoke_backend::interpreter().unwrap();
        interp.def_file_for_type::<_, Container>("container").unwrap();

        let code = b"require 'container'; Container.new('a' * 1024 * 1024)";
        let result = interp.eval(code);
        result.unwrap();
        interp.close();
    }
}
