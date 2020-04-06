#![deny(clippy::all)]
#![deny(clippy::pedantic)]

#[macro_use]
extern crate artichoke_backend;

use artichoke_backend::class;
use artichoke_backend::convert::RustBackedValue;
use artichoke_backend::def;
use artichoke_backend::exception::Exception;
use artichoke_backend::sys;
use artichoke_backend::types::Int;
use artichoke_backend::value::Value;
use artichoke_backend::{Artichoke, Convert, Eval, File, LoadSources, ValueLike};

#[derive(Clone, Debug, Default)]
struct Container {
    inner: i64,
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
        let mut interp = unwrap_interpreter!(mrb);
        let inner = Value::new(&interp, inner);
        let inner = inner.try_into::<Int>(&interp).unwrap_or_default();
        let container = Box::new(Self { inner });
        container
            .try_into_ruby(&interp, Some(slf))
            .unwrap_or_else(|_| interp.convert(None::<Value>))
            .inner()
    }

    unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        if let Ok(data) = Box::<Self>::try_from_ruby(&interp, &value) {
            let borrow = data.borrow();
            interp.convert(borrow.inner).inner()
        } else {
            interp.convert(None::<Value>).inner()
        }
    }
}

impl File for Container {
    type Artichoke = Artichoke;

    type Error = Exception;

    fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
        let spec = class::Spec::new("Container", None, Some(def::rust_data_free::<Box<Self>>))?;
        class::Builder::for_spec(interp, &spec)
            .value_is_rust_object()
            .add_method("initialize", Self::initialize, sys::mrb_args_req(1))?
            .add_method("value", Self::value, sys::mrb_args_none())?
            .define()?;
        interp.0.borrow_mut().def_class::<Box<Self>>(spec);
        Ok(())
    }
}

#[test]
fn define_rust_backed_ruby_class() {
    let mut interp = artichoke_backend::interpreter().unwrap();
    interp
        .def_file_for_type::<Container>(b"container.rb")
        .unwrap();

    let _ = interp.eval(b"require 'container'").unwrap();
    let result = interp.eval(b"Container.new(15).value").unwrap();
    let result = result.try_into::<Int>(&interp).unwrap();
    assert_eq!(result, 15);
    // Ensure Rc is cloned correctly and still points to valid memory.
    let result = interp.eval(b"Container.new(105).value").unwrap();
    let result = result.try_into::<Int>(&interp).unwrap();
    assert_eq!(result, 105);
}
