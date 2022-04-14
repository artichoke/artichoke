use std::ffi::CStr;

use crate::extn::core::matchdata::{self, trampoline};
use crate::extn::prelude::*;

const MATCH_DATA_CSTR: &CStr = qed::const_cstr_from_str!("MatchData\0");
static MATCH_DATA_RUBY_SOURCE: &[u8] = include_bytes!("matchdata.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<matchdata::MatchData>() {
        return Ok(());
    }

    let spec = class::Spec::new(
        "MatchData",
        MATCH_DATA_CSTR,
        None,
        Some(def::box_unbox_free::<matchdata::MatchData>),
    )?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_method("begin", matchdata_begin, sys::mrb_args_req(1))?
        .add_method("captures", matchdata_captures, sys::mrb_args_none())?
        .add_method("[]", matchdata_element_reference, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("length", matchdata_length, sys::mrb_args_none())?
        .add_method("named_captures", matchdata_named_captures, sys::mrb_args_none())?
        .add_method("names", matchdata_names, sys::mrb_args_none())?
        .add_method("offset", matchdata_offset, sys::mrb_args_req(1))?
        .add_method("post_match", matchdata_post_match, sys::mrb_args_none())?
        .add_method("pre_match", matchdata_pre_match, sys::mrb_args_none())?
        .add_method("regexp", matchdata_regexp, sys::mrb_args_none())?
        .add_method("size", matchdata_length, sys::mrb_args_none())?
        .add_method("string", matchdata_string, sys::mrb_args_none())?
        .add_method("to_a", matchdata_to_a, sys::mrb_args_none())?
        .add_method("to_s", matchdata_to_s, sys::mrb_args_none())?
        .add_method("end", matchdata_end, sys::mrb_args_req(1))?
        .define()?;
    interp.def_class::<matchdata::MatchData>(spec)?;
    interp.eval(MATCH_DATA_RUBY_SOURCE)?;

    Ok(())
}

unsafe extern "C" fn matchdata_begin(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let begin = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let begin = Value::from(begin);
    let result = trampoline::begin(&mut guard, value, begin);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_captures(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::captures(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_element_reference(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (elem, len) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let elem = Value::from(elem);
    let len = len.map(Value::from);
    let result = trampoline::element_reference(&mut guard, value, elem, len);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_end(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let end = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let end = Value::from(end);
    let result = trampoline::end(&mut guard, value, end);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::length(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_named_captures(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::named_captures(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::names(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_offset(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let offset = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let offset = Value::from(offset);
    let result = trampoline::offset(&mut guard, value, offset);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_post_match(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::post_match(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_pre_match(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::pre_match(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_regexp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::regexp(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_string(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::string(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_to_a(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::to_a(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn matchdata_to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::to_s(&mut guard, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
