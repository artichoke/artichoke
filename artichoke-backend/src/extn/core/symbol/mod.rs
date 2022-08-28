use std::ffi::c_void;

use crate::convert::{Immediate, UnboxedValueGuard};
use crate::extn::prelude::*;

pub mod ffi;
pub(in crate::extn) mod mruby;
pub(super) mod trampoline;

#[doc(inline)]
pub use spinoso_symbol::Symbol;

impl BoxUnboxVmValue for Symbol {
    type Unboxed = Self;
    type Guarded = Immediate<Self::Unboxed>;

    const RUBY_TYPE: &'static str = "Symbol";

    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error> {
        let _ = interp;

        // Make sure we have a Symbol otherwise extraction will fail.
        // This check is critical to the safety of accessing the `value` union.
        if value.ruby_type() != Ruby::Symbol {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let value = value.inner();
        // SAFETY: The above check on the data type ensures the `value` union
        // holds a `u32` in the `sym` variant.
        let symbol_id = value.value.sym;
        Ok(UnboxedValueGuard::new(Immediate::new(symbol_id.into())))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        let _ = interp;

        let symbol_id = u32::from(value);
        let obj = unsafe { sys::mrb_sys_new_symbol(symbol_id) };
        Ok(Value::from(obj))
    }

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error> {
        let _ = value;
        let _ = into;
        let _ = interp;
        Err(Fatal::from("Symbols are immutable and cannot be reinitialized").into())
    }

    fn free(data: *mut c_void) {
        // this function is never called. `Symbol` is `Copy`/immediate and does
        // not have a destructor registered in the class registry.
        let _ = data;
    }
}
