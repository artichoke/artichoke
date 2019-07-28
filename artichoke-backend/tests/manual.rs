#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

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
                let inner = i64::try_from_mrb(&interp, inner).map_err(MrbError::ConvertToRust)?;
                Ok(Self { inner })
            }
        }

        let interp = unwrap_interpreter!(mrb);
        Args::extract(&interp)
            .and_then(|args| {
                let container = Box::new(Self { inner: args.inner });
                container.try_into_ruby(&interp, Some(slf))
            })
            .unwrap_or_else(|_| Value::from_mrb(&interp, None::<Value>))
            .inner()
    }

    unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        if let Ok(data) = <Box<Self>>::try_from_ruby(&interp, &Value::new(&interp, slf)) {
            let borrow = data.borrow();
            Value::from_mrb(&interp, borrow.inner).inner()
        } else {
            Value::from_mrb(&interp, None::<Value>).inner()
        }
    }
}

impl MrbFile for Container {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        let spec = {
            let mut api = interp.borrow_mut();
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
    let interp = artichoke_backend::interpreter().expect("mrb init");
    interp
        .def_file_for_type::<_, Container>("container.rb")
        .expect("def file");

    interp.eval("require 'container'").expect("require");
    let result = interp.eval("Container.new(15).value").expect("eval");
    assert_eq!(unsafe { i64::try_from_mrb(&interp, result) }, Ok(15));
    // Ensure Rc is cloned correctly and still points to valid memory.
    let result = interp.eval("Container.new(15).value").expect("eval");
    assert_eq!(unsafe { i64::try_from_mrb(&interp, result) }, Ok(15));
}
