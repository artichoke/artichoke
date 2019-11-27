#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate artichoke_backend;

use artichoke_backend::convert::{Convert, RustBackedValue};
use artichoke_backend::def::{rust_data_free, ClassLike, Define};
use artichoke_backend::sys;
use artichoke_backend::types::Int;
use artichoke_backend::value::Value;
use artichoke_backend::{Artichoke, ArtichokeError};
use artichoke_core::eval::Eval;
use artichoke_core::file::File;
use artichoke_core::load::LoadSources;
use artichoke_core::value::Value as ValueLike;

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
        let interp = unwrap_interpreter!(mrb);
        let inner = Value::new(&interp, inner);
        inner
            .try_into::<Int>()
            .and_then(|inner| {
                let container = Box::new(Self { inner });
                container.try_into_ruby(&interp, Some(slf))
            })
            .unwrap_or_else(|_| interp.convert(None::<Value>))
            .inner()
    }

    unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        if let Ok(data) = <Box<Self>>::try_from_ruby(&interp, &Value::new(&interp, slf)) {
            let borrow = data.borrow();
            interp.convert(borrow.inner).inner()
        } else {
            interp.convert(None::<Value>).inner()
        }
    }
}

impl File for Container {
    type Artichoke = Artichoke;

    fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
        let spec = {
            let mut api = interp.0.borrow_mut();
            let spec =
                api.def_class::<Box<Self>>("Container", None, Some(rust_data_free::<Box<Self>>));
            spec.borrow_mut()
                .add_method("initialize", Self::initialize, sys::mrb_args_req(1));
            spec.borrow_mut()
                .add_method("value", Self::value, sys::mrb_args_none());
            spec.borrow_mut().mrb_value_is_rust_backed(true);
            spec
        };
        spec.borrow().define(&interp)?;
        Ok(())
    }
}

#[test]
fn define_rust_backed_ruby_class() {
    let interp = artichoke_backend::interpreter().expect("init");
    interp
        .def_file_for_type::<Container>(b"container.rb")
        .expect("def file");

    interp.eval(b"require 'container'").expect("require");
    let result = interp.eval(b"Container.new(15).value").expect("eval");
    assert_eq!(result.try_into::<Int>(), Ok(15));
    // Ensure Rc is cloned correctly and still points to valid memory.
    let result = interp.eval(b"Container.new(15).value").expect("eval");
    assert_eq!(result.try_into::<Int>(), Ok(15));
}
