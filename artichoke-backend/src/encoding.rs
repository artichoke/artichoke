use crate::convert::{BoxUnboxVmValue, Immediate, UnboxedValueGuard};
use crate::prelude::*;
use crate::sys;
use crate::value::Value;
use artichoke_core::value::Value as _;
pub use spinoso_string::Encoding;
use std::ffi::{c_void, CStr};

const ENCODING_CSTR: &CStr = qed::const_cstr_from_str!("Encoding\0");

pub const RUBY_TYPE: &str = "Encoding";
pub type Spec = Encoding;

pub const DATA_TYPE: sys::mrb_data_type = sys::mrb_data_type {
    struct_name: ENCODING_CSTR.as_ptr(),
    dfree: None,
};

impl BoxUnboxVmValue for Encoding {
    type Unboxed = Self;
    type Guarded = Immediate<Self::Unboxed>;

    const RUBY_TYPE: &'static str = "Encoding";

    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error> {
        let _ = interp;

        // Make sure we have a Symbol otherwise extraction will fail.
        // This check is critical to the safety of accessing the `value` union.
        if value.ruby_type() != Ruby::Encoding {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let value = value.inner();

        // TODO: Safety is not super great
        let encoding_id = value.value.i as i8;
        let encoding = unsafe { std::mem::transmute(encoding_id) };
        Ok(UnboxedValueGuard::new(Immediate::new(encoding)))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        let _ = interp;

        let encoding_id = value as i8;
        let obj = unsafe { sys::mrb_sys_new_encoding(i64::from(encoding_id)) };
        Ok(Value::from(obj))
    }

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error> {
        let _ = value;
        let _ = into;
        let _ = interp;
        Err(Fatal::from("Encodings are immutable and cannot be reinitialized").into())
    }

    fn free(data: *mut c_void) {
        // this function is never called. `Encoding` is `Copy`/immediate and does
        // not have a destructor registered in the class registry.
        let _ = data;
    }
}
