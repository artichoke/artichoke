use std::convert::TryFrom;

use crate::extn::core::regexp;
use crate::extn::prelude::*;
use crate::sys;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<regexp::Regexp>() {
        return Ok(());
    }
    let spec = class::Spec::new("Regexp", None, Some(def::rust_data_free::<regexp::Regexp>))?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_method("initialize", initialize, sys::mrb_args_req_and_opt(1, 2))?
        .add_self_method("compile", compile, sys::mrb_args_rest())?
        .add_self_method("escape", escape, sys::mrb_args_req(1))?
        .add_self_method("quote", escape, sys::mrb_args_req(1))?
        .add_self_method("union", union, sys::mrb_args_rest())?
        .add_method("==", eql, sys::mrb_args_req(1))?
        .add_method("===", case_compare, sys::mrb_args_req(1))?
        .add_method("=~", match_operator, sys::mrb_args_req(1))?
        .add_method("casefold?", casefold, sys::mrb_args_none())?
        .add_method("eql?", eql, sys::mrb_args_req(1))?
        .add_method("fixed_encoding?", fixed_encoding, sys::mrb_args_none())?
        .add_method("hash", hash, sys::mrb_args_none())?
        .add_method("inspect", inspect, sys::mrb_args_none())?
        .add_method("match?", match_q, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("named_captures", named_captures, sys::mrb_args_none())?
        .add_method("match", match_, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("names", names, sys::mrb_args_none())?
        .add_method("options", options, sys::mrb_args_none())?
        .add_method("source", source, sys::mrb_args_none())?
        .add_method("to_s", to_s, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<regexp::Regexp>(spec)?;
    let _ = interp.eval(&include_bytes!("regexp.rb")[..])?;
    let ignorecase = interp.convert(regexp::IGNORECASE);
    interp.define_class_constant::<regexp::Regexp>("IGNORECASE", ignorecase)?;
    let extended = interp.convert(regexp::EXTENDED);
    interp.define_class_constant::<regexp::Regexp>("EXTENDED", extended)?;
    let multiline = interp.convert(regexp::MULTILINE);
    interp.define_class_constant::<regexp::Regexp>("MULTILINE", multiline)?;
    let fixed_encoding = interp.convert(regexp::FIXEDENCODING);
    interp.define_class_constant::<regexp::Regexp>("FIXEDENCODING", fixed_encoding)?;
    let no_encoding = interp.convert(regexp::NOENCODING);
    interp.define_class_constant::<regexp::Regexp>("NOENCODING", no_encoding)?;
    trace!("Patched Regexp onto interpreter");
    Ok(())
}

unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, options, encoding) = mrb_get_args!(mrb, required = 1, optional = 2);
    let mut interp = unwrap_interpreter!(mrb);
    let into = Value::new(&interp, slf);
    let pattern = Value::new(&interp, pattern);
    let options = options.map(|options| Value::new(&interp, options));
    let encoding = encoding.map(|encoding| Value::new(&interp, encoding));
    let result =
        regexp::trampoline::initialize(&mut interp, pattern, options, encoding, Some(into));
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn compile(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    // Call `mrb_obj_new` instead of allocating an object of class `slf` and
    // delegating to `regexp::trampoline::initialize` to handle cases where
    // subclasses override initialize.
    if let Ok(argslen) = Int::try_from(args.len()) {
        sys::mrb_obj_new(mrb, sys::mrb_sys_class_ptr(slf), argslen, args.as_ptr())
    } else {
        sys::mrb_sys_nil_value()
    }
}

unsafe extern "C" fn escape(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let pattern = Value::new(&interp, pattern);
    let result = regexp::trampoline::escape(&mut interp, pattern);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn union(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    let mut interp = unwrap_interpreter!(mrb);
    let args = args
        .iter()
        .copied()
        .map(|arg| Value::new(&interp, arg))
        .collect::<Vec<_>>();
    let result = regexp::trampoline::union(&mut interp, args);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn match_q(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, pos) = mrb_get_args!(mrb, required = 1, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let pattern = Value::new(&interp, pattern);
    let pos = pos.map(|pos| Value::new(&interp, pos));
    let result = regexp::trampoline::is_match(&mut interp, value, pattern, pos);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, pos, block) = mrb_get_args!(mrb, required = 1, optional = 1, &block);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let pattern = Value::new(&interp, pattern);
    let pos = pos.map(|pos| Value::new(&interp, pos));
    let result = regexp::trampoline::match_(&mut interp, value, pattern, pos, block);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let other = Value::new(&interp, other);
    let result = regexp::trampoline::eql(&mut interp, value, other);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn case_compare(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let pattern = Value::new(&interp, pattern);
    let result = regexp::trampoline::case_compare(&mut interp, value, pattern);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn match_operator(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let pattern = Value::new(&interp, pattern);
    let result = regexp::trampoline::match_operator(&mut interp, value, pattern);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn casefold(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::is_casefold(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn fixed_encoding(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::is_fixed_encoding(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::hash(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::inspect(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn named_captures(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::named_captures(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::names(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::options(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn source(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::source(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}

unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::to_s(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp.into_inner(), exception),
    }
}
