use crate::convert::Convert;
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub mod core;
pub mod stdlib;

pub const RUBY_PLATFORM: &str = "x86_64-unknown-artichoke";
pub const INPUT_RECORD_SEPARATOR: &str = "\n";

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let mrb = interp.0.borrow().mrb;
    unsafe {
        let ruby_platform = Value::convert(interp, RUBY_PLATFORM);
        sys::mrb_define_global_const(
            mrb,
            b"RUBY_PLATFORM\0".as_ptr() as *const i8,
            ruby_platform.inner(),
        );
        let ruby_description = Value::convert(interp, sys::mruby_sys_version(true));
        sys::mrb_define_global_const(
            mrb,
            b"RUBY_DESCRIPTION\0".as_ptr() as *const i8,
            ruby_description.inner(),
        );
        let input_record_separator = Value::convert(interp, INPUT_RECORD_SEPARATOR);
        sys::mrb_gv_set(
            mrb,
            interp.0.borrow_mut().sym_intern("$/"),
            input_record_separator.inner(),
        );
    }
    core::init(interp)?;
    stdlib::init(interp)?;
    Ok(())
}
