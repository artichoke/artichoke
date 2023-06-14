use crate::sys;
pub use spinoso_string::Encoding;
use std::ffi::CStr;

const ENCODING_CSTR: &CStr = qed::const_cstr_from_str!("Encoding\0");

pub const RUBY_TYPE: &str = "Encoding";
pub type Spec = Encoding;

pub const DATA_TYPE: sys::mrb_data_type = sys::mrb_data_type {
    struct_name: ENCODING_CSTR.as_ptr(),
    dfree: None,
};
