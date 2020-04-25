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
        .with_super_class(interp.class_spec::<StandardError>()?)
        .define()?;
    interp.state.def_class::<math::DomainError>(domainerror)?;

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
    let value = Value::new(&interp, value);
    let result = math::acos(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_acosh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::acosh(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_asin(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::asin(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_asinh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::asinh(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_atan(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::atan(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_atan2(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (value, other) = mrb_get_args!(mrb, required = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let other = Value::new(&interp, other);
    let result = math::atan2(&mut interp, value, other).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_atanh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::atanh(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_cbrt(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::cbrt(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_cos(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::cos(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_cosh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::cosh(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_erf(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::erf(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_erfc(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::erfc(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_exp(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::exp(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_frexp(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::frexp(&mut interp, value).map(|(fraction, exponent)| {
        let fraction = interp.convert_mut(fraction);
        let exponent = interp.convert(exponent);
        interp.convert_mut(&[fraction, exponent][..])
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_gamma(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::gamma(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_hypot(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (value, other) = mrb_get_args!(mrb, required = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let other = Value::new(&interp, other);
    let result = math::hypot(&mut interp, value, other).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_ldexp(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (fraction, exponent) = mrb_get_args!(mrb, required = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let fraction = Value::new(&interp, fraction);
    let exponent = Value::new(&interp, exponent);
    let result =
        math::ldexp(&mut interp, fraction, exponent).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_lgamma(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::lgamma(&mut interp, value).map(|(result, sign)| {
        let result = interp.convert_mut(result);
        let sign = interp.convert(sign);
        interp.convert_mut(&[result, sign][..])
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_log(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let (value, base) = mrb_get_args!(mrb, required = 1, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let base = base.map(|base| Value::new(&interp, base));
    let result = math::log(&mut interp, value, base).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_log10(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::log10(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_log2(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::log2(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_sin(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::sin(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_sinh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::sinh(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_sqrt(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::sqrt(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_tan(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::tan(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_math_tanh(
    mrb: *mut sys::mrb_state,
    _slf: sys::mrb_value,
) -> sys::mrb_value {
    let value = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    let result = math::tanh(&mut interp, value).map(|result| interp.convert_mut(result));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
