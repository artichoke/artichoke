//! FFI glue between the Rust trampolines and the mruby C interpreter.

use std::ffi::CStr;

use crate::extn::core::math::trampoline;
use crate::extn::core::math::{self, DomainError, Math};
use crate::extn::prelude::*;

const MATH_CSTR: &CStr = cstr::cstr!("Math");
const DOMAIN_ERROR_CSTR: &CStr = cstr::cstr!("DomainError");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<Math>() {
        return Ok(());
    }

    let spec = module::Spec::new(interp, "Math", MATH_CSTR, None)?;
    module::Builder::for_spec(interp, &spec)
        .add_module_method("acos", math_acos, sys::mrb_args_req(1))?
        .add_module_method("acosh", math_acosh, sys::mrb_args_req(1))?
        .add_module_method("asin", math_asin, sys::mrb_args_req(1))?
        .add_module_method("asinh", math_asinh, sys::mrb_args_req(1))?
        .add_module_method("atan", math_atan, sys::mrb_args_req(1))?
        .add_module_method("atan2", math_atan2, sys::mrb_args_req(2))?
        .add_module_method("atanh", math_atanh, sys::mrb_args_req(1))?
        .add_module_method("cbrt", math_cbrt, sys::mrb_args_req(1))?
        .add_module_method("cos", math_cos, sys::mrb_args_req(1))?
        .add_module_method("cosh", math_cosh, sys::mrb_args_req(1))?
        .add_module_method("erf", math_erf, sys::mrb_args_req(1))?
        .add_module_method("erfc", math_erfc, sys::mrb_args_req(1))?
        .add_module_method("exp", math_exp, sys::mrb_args_req(1))?
        .add_module_method("frexp", math_frexp, sys::mrb_args_req(1))?
        .add_module_method("gamma", math_gamma, sys::mrb_args_req(1))?
        .add_module_method("hypot", math_hypot, sys::mrb_args_req(2))?
        .add_module_method("ldexp", math_ldexp, sys::mrb_args_req(2))?
        .add_module_method("lgamma", math_lgamma, sys::mrb_args_req(1))?
        .add_module_method("log", math_log, sys::mrb_args_req_and_opt(1, 1))?
        .add_module_method("log10", math_log10, sys::mrb_args_req(1))?
        .add_module_method("log2", math_log2, sys::mrb_args_req(1))?
        .add_module_method("sin", math_sin, sys::mrb_args_req(1))?
        .add_module_method("sinh", math_sinh, sys::mrb_args_req(1))?
        .add_module_method("sqrt", math_sqrt, sys::mrb_args_req(1))?
        .add_module_method("tan", math_tan, sys::mrb_args_req(1))?
        .add_module_method("tanh", math_tanh, sys::mrb_args_req(1))?
        .define()?;

    let domainerror = class::Spec::new(
        "DomainError",
        DOMAIN_ERROR_CSTR,
        Some(EnclosingRubyScope::module(&spec)),
        None,
    )?;
    class::Builder::for_spec(interp, &domainerror)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<DomainError>(domainerror)?;

    interp.def_module::<Math>(spec)?;
    let e = interp.convert_mut(math::E);
    interp.define_module_constant::<Math>("E", e)?;
    let pi = interp.convert_mut(math::PI);
    interp.define_module_constant::<Math>("PI", pi)?;

    Ok(())
}

unsafe extern "C" fn math_acos(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::acos(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_acosh(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::acosh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_asin(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::asin(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_asinh(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::asinh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_atan(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::atan(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_atan2(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (value, other) = mrb_get_args!(mrb, required = 2);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let other = Value::from(other);
    let result = trampoline::atan2(&mut guard, value, other).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_atanh(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::atanh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_cbrt(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::cbrt(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_cos(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::cos(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_cosh(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::cosh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_erf(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::erf(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_erfc(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::erfc(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_exp(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::exp(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_frexp(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::frexp(&mut guard, value).and_then(|(fraction, exponent)| {
        let fraction = guard.convert_mut(fraction);
        let exponent = guard.convert(exponent);
        guard.try_convert_mut(&[fraction, exponent][..])
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_gamma(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::gamma(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_hypot(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (value, other) = mrb_get_args!(mrb, required = 2);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let other = Value::from(other);
    let result = trampoline::hypot(&mut guard, value, other).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_ldexp(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (fraction, exponent) = mrb_get_args!(mrb, required = 2);
    unwrap_interpreter!(mrb, to => guard);
    let fraction = Value::from(fraction);
    let exponent = Value::from(exponent);
    let result = trampoline::ldexp(&mut guard, fraction, exponent).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_lgamma(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::lgamma(&mut guard, value).and_then(|(result, sign)| {
        let result = guard.convert_mut(result);
        let sign = guard.convert(sign);
        guard.try_convert_mut(&[result, sign][..])
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_log(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (value, base) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let base = base.map(Value::from);
    let result = trampoline::log(&mut guard, value, base).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_log10(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::log10(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_log2(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::log2(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_sin(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::sin(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_sinh(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::sinh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_sqrt(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::sqrt(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_tan(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::tan(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn math_tanh(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(value);
    let result = trampoline::tanh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
