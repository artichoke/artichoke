#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate artichoke_backend;

use artichoke_backend::convert::{Convert, RustBackedValue};
use artichoke_backend::def::{rust_data_free, ClassLike, Define};
use artichoke_backend::eval::Eval;
use artichoke_backend::file::File;
use artichoke_backend::load::LoadSources;
use artichoke_backend::sys;
use artichoke_backend::value::Value;
use artichoke_backend::{Artichoke, ArtichokeError};
use std::mem;

#[derive(Clone, Debug, Default)]
struct Container {
    inner: i64,
}

impl RustBackedValue for Container {}

impl Container {
    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        struct Args {
            inner: i64,
        }

        impl Args {
            const ARGSPEC: &'static [u8] = b"o\0";

            unsafe fn extract(interp: &Artichoke) -> Result<Self, ArtichokeError> {
                let inner = <mem::MaybeUninit<sys::mrb_value>>::uninit();
                sys::mrb_get_args(
                    interp.0.borrow().mrb,
                    Self::ARGSPEC.as_ptr() as *const i8,
                    &inner,
                );
                let inner = Value::new(interp, inner.assume_init());
                let inner = inner.try_into::<i64>()?;
                Ok(Self { inner })
            }
        }

        let interp = unwrap_interpreter!(mrb);
        Args::extract(&interp)
            .and_then(|args| {
                let container = Box::new(Self { inner: args.inner });
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
    fn require(interp: Artichoke) -> Result<(), ArtichokeError> {
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
        .def_file_for_type::<_, Container>("container.rb")
        .expect("def file");

    interp.eval("require 'container'").expect("require");
    let result = interp.eval("Container.new(15).value").expect("eval");
    assert_eq!(result.try_into::<i64>(), Ok(15));
    // Ensure Rc is cloned correctly and still points to valid memory.
    let result = interp.eval("Container.new(15).value").expect("eval");
    assert_eq!(result.try_into::<i64>(), Ok(15));
}
