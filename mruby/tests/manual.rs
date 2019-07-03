#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate mruby;

use mruby::convert::{FromMrb, RustBackedValue, TryFromMrb};
use mruby::def::{rust_data_free, ClassLike, Define};
use mruby::eval::MrbEval;
use mruby::file::MrbFile;
use mruby::interpreter::{Interpreter, MrbApi};
use mruby::load::MrbLoadSources;
use mruby::sys;
use mruby::value::Value;
use mruby::{Mrb, MrbError};
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
                let inner = mem::uninitialized::<sys::mrb_value>();
                let mut argspec = vec![];
                argspec
                    .write_all(sys::specifiers::OBJECT.as_bytes())
                    .map_err(|_| MrbError::ArgSpec)?;
                argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(interp.borrow().mrb, argspec.as_ptr() as *const i8, &inner);
                let inner = Value::new(interp, inner);
                let inner = i64::try_from_mrb(&interp, inner).map_err(MrbError::ConvertToRust)?;
                Ok(Self { inner })
            }
        }

        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, Args::extract(&interp), interp.nil().inner());

        let container = Box::new(Self { inner: args.inner });
        unwrap_value_or_raise!(interp, container.try_into_ruby(&interp, Some(slf)))
    }

    unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let data = unwrap_or_raise!(
            interp,
            <Box<Self>>::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        Value::from_mrb(&interp, borrow.inner).inner()
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
    let interp = Interpreter::create().expect("mrb init");
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
