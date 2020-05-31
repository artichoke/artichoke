use crate::extn::core::math;
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_module_defined::<math::Math>() {
        return Ok(());
    }
    let spec = module::Spec::new(interp, "Math", None)?;
    module::Builder::for_spec(interp, &spec)
        .add_module_method("acos", artichoke_math_acos, sys::mrb_args_req(1))?
        .add_module_method("acosh", artichoke_math_acosh, sys::mrb_args_req(1))?
        .add_module_method("asin", artichoke_math_asin, sys::mrb_args_req(1))?
        .add_module_method("asinh", artichoke_math_asinh, sys::mrb_args_req(1))?
        .add_module_method("atan", artichoke_math_atan, sys::mrb_args_req(1))?
        .add_module_method("atan2", artichoke_math_atan2, sys::mrb_args_req(2))?
        .add_module_method("atanh", artichoke_math_atanh, sys::mrb_args_req(1))?
        .add_module_method("cbrt", artichoke_math_cbrt, sys::mrb_args_req(1))?
        .add_module_method("cos", artichoke_math_cos, sys::mrb_args_req(1))?
        .add_module_method("cosh", artichoke_math_cosh, sys::mrb_args_req(1))?
        .add_module_method("erf", artichoke_math_erf, sys::mrb_args_req(1))?
        .add_module_method("erfc", artichoke_math_erfc, sys::mrb_args_req(1))?
        .add_module_method("exp", artichoke_math_exp, sys::mrb_args_req(1))?
        .add_module_method("frexp", artichoke_math_frexp, sys::mrb_args_req(1))?
        .add_module_method("gamma", artichoke_math_gamma, sys::mrb_args_req(1))?
        .add_module_method("hypot", artichoke_math_hypot, sys::mrb_args_req(2))?
        .add_module_method("ldexp", artichoke_math_ldexp, sys::mrb_args_req(2))?
        .add_module_method("lgamma", artichoke_math_lgamma, sys::mrb_args_req(1))?
        .add_module_method("log", artichoke_math_log, sys::mrb_args_req_and_opt(1, 1))?
        .add_module_method("log10", artichoke_math_log10, sys::mrb_args_req(1))?
        .add_module_method("log2", artichoke_math_log2, sys::mrb_args_req(1))?
        .add_module_method("sin", artichoke_math_sin, sys::mrb_args_req(1))?
        .add_module_method("sinh", artichoke_math_sinh, sys::mrb_args_req(1))?
        .add_module_method("sqrt", artichoke_math_sqrt, sys::mrb_args_req(1))?
        .add_module_method("tan", artichoke_math_tan, sys::mrb_args_req(1))?
        .add_module_method("tanh", artichoke_math_tanh, sys::mrb_args_req(1))?
        .define()?;

    let domainerror =
        class::Spec::new("DomainError", Some(EnclosingRubyScope::module(&spec)), None)?;
    class::Builder::for_spec(interp, &domainerror)
        .with_super_class::<StandardError, _>("StandardError")?
        .define()?;
    interp.def_class::<math::DomainError>(domainerror)?;

    interp.def_module::<math::Math>(spec)?;
    let e = interp.convert_mut(math::E);
    interp.define_module_constant::<math::Math>("E", e)?;
    let pi = interp.convert_mut(math::PI);
    interp.define_module_constant::<math::Math>("PI", pi)?;
    Ok(())
}

unsafe extern "C" fn artichoke_math_acos(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::acos(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_acosh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::acosh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_asin(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::asin(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_asinh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::asinh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_atan(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::atan(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_atan2(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (value, other) = mrb_get_args!(mrb, required = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let other = Value::from(other);
    let result = math::atan2(&mut guard, value, other).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_atanh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::atanh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_cbrt(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::cbrt(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_cos(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::cos(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_cosh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::cosh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_erf(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::erf(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_erfc(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::erfc(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_exp(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::exp(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_frexp(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::frexp(&mut guard, value).map(|(fraction, exponent)| {
        let fraction = guard.convert_mut(fraction);
        let exponent = guard.convert(exponent);
        guard.convert_mut(&[fraction, exponent][..])
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_gamma(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::gamma(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_hypot(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (value, other) = mrb_get_args!(mrb, required = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let other = Value::from(other);
    let result = math::hypot(&mut guard, value, other).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_ldexp(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (fraction, exponent) = mrb_get_args!(mrb, required = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let fraction = Value::from(fraction);
    let exponent = Value::from(exponent);
    let result =
        math::ldexp(&mut guard, fraction, exponent).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_lgamma(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::lgamma(&mut guard, value).map(|(result, sign)| {
        let result = guard.convert_mut(result);
        let sign = guard.convert(sign);
        guard.convert_mut(&[result, sign][..])
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_log(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (value, base) = mrb_get_args!(mrb, required = 1, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let base = base.map(Value::from);
    let result = math::log(&mut guard, value, base).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_log10(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::log10(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_log2(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::log2(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_sin(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::sin(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_sinh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::sinh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_sqrt(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::sqrt(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_tan(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::tan(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

unsafe extern "C" fn artichoke_math_tanh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let value = Value::from(value);
    let result = math::tanh(&mut guard, value).map(|result| guard.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}
