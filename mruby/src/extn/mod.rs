use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::MrbError;

pub mod core;
pub mod stdlib;
pub mod test;

pub const RUBY_PLATFORM: &str = "x86_64-unknown-mruby";
pub const INPUT_RECORD_SEPARATOR: &str = "\n";

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    unsafe {
        let ruby_platform = interp.string(RUBY_PLATFORM);
        sys::mrb_define_global_const(
            interp.borrow().mrb,
            b"RUBY_PLATFORM\0".as_ptr() as *const i8,
            ruby_platform.inner(),
        );
        let ruby_description = interp.string(sys::mruby_sys_version(true));
        sys::mrb_define_global_const(
            interp.borrow().mrb,
            b"RUBY_DESCRIPTION\0".as_ptr() as *const i8,
            ruby_description.inner(),
        );
        let global_input_record_separator = "$/";
        let global_input_record_separator_name = sys::mrb_intern(
            interp.borrow().mrb,
            global_input_record_separator.as_ptr() as *const i8,
            global_input_record_separator.len(),
        );
        sys::mrb_gv_set(
            interp.borrow().mrb,
            global_input_record_separator_name,
            interp.string(INPUT_RECORD_SEPARATOR).inner(),
        );
    }
    core::patch(interp)?;
    stdlib::patch(interp)?;
    #[cfg(test)]
    test::init(interp)?;
    Ok(())
}
