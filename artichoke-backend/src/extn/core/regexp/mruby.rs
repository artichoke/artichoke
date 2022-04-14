use std::ffi::CStr;

use super::{trampoline, Flags, Regexp};
use crate::extn::prelude::*;

const REGEXP_CSTR: &CStr = qed::const_cstr_from_str!("Regexp\0");
static REGEXP_RUBY_SOURCE: &[u8] = include_bytes!("regexp.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Regexp>() {
        return Ok(());
    }

    let spec = class::Spec::new("Regexp", REGEXP_CSTR, None, Some(def::box_unbox_free::<Regexp>))?;
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
    interp.def_class::<Regexp>(spec)?;

    interp.eval(REGEXP_RUBY_SOURCE)?;

    // Declare class constants
    let ignorecase = interp.convert(Flags::IGNORECASE.bits());
    interp.define_class_constant::<Regexp>("IGNORECASE", ignorecase)?;
    let extended = interp.convert(Flags::EXTENDED.bits());
    interp.define_class_constant::<Regexp>("EXTENDED", extended)?;
    let multiline = interp.convert(Flags::MULTILINE.bits());
    interp.define_class_constant::<Regexp>("MULTILINE", multiline)?;
    let fixed_encoding = interp.convert(Flags::FIXEDENCODING.bits());
    interp.define_class_constant::<Regexp>("FIXEDENCODING", fixed_encoding)?;
    let no_encoding = interp.convert(Flags::NOENCODING.bits());
    interp.define_class_constant::<Regexp>("NOENCODING", no_encoding)?;

    Ok(())
}

unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, options, encoding) = mrb_get_args!(mrb, required = 1, optional = 2);
    unwrap_interpreter!(mrb, to => guard);
    let slf = Value::from(slf);
    let pattern = Value::from(pattern);
    let options = options.map(Value::from);
    let encoding = encoding.map(Value::from);
    let result = trampoline::initialize(&mut guard, pattern, options, encoding, slf);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn compile(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    // Call `mrb_obj_new` instead of allocating an object of class `slf` and
    // delegating to `trampoline::initialize` to handle cases where subclasses
    // override initialize.
    if let Ok(argslen) = sys::mrb_int::try_from(args.len()) {
        sys::mrb_obj_new(mrb, sys::mrb_sys_class_ptr(slf), argslen, args.as_ptr())
    } else {
        sys::mrb_sys_nil_value()
    }
}

unsafe extern "C" fn escape(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let pattern = Value::from(pattern);
    let result = trampoline::escape(&mut guard, pattern);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn union(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let args = args.iter().copied().map(Value::from);
    let result = trampoline::union(&mut guard, args);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn match_q(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, pos) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let pattern = Value::from(pattern);
    let pos = pos.map(Value::from);
    let result = trampoline::is_match(&mut guard, value, pattern, pos);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, pos, block) = mrb_get_args!(mrb, required = 1, optional = 1, &block);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let pattern = Value::from(pattern);
    let pos = pos.map(Value::from);
    let result = trampoline::match_(&mut guard, value, pattern, pos, block);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::eql(&mut guard, value, other);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn case_compare(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let pattern = Value::from(pattern);
    let result = trampoline::case_compare(&mut guard, value, pattern);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn match_operator(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let pattern = Value::from(pattern);
    let result = trampoline::match_operator(&mut guard, value, pattern);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn casefold(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::is_casefold(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn fixed_encoding(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::is_fixed_encoding(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::hash(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::inspect(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn named_captures(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::named_captures(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::names(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::options(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn source(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::source(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::to_s(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
