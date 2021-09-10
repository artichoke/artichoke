use crate::extn::core::float::Float;
use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

// ```c
// MRB_API mrb_float mrb_to_flo(mrb_state *mrb, mrb_value x);
// ```
#[no_mangle]
unsafe extern "C" fn mrb_to_flo(mrb: *mut sys::mrb_state, value: sys::mrb_value) -> sys::mrb_float {
    unwrap_interpreter!(mrb, to => guard, or_else = 0.0);
    let value = Value::from(value);
    let result = value
        .try_convert_into::<Float>(&guard)
        .map(Float::as_f64)
        .or_else(|_| value.try_convert_into::<Integer>(&guard).map(Integer::as_f64));
    match result {
        Ok(flt) => flt,
        Err(exception) => error::raise(guard, exception),
    }
}
