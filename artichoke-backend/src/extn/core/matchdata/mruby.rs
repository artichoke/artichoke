use crate::extn::core::matchdata::{self, trampoline};
use crate::extn::prelude::*;
use crate::sys;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<matchdata::MatchData>() {
        return Ok(());
    }
    let spec = class::Spec::new(
        "MatchData",
        None,
        Some(def::rust_data_free::<matchdata::MatchData>),
    )?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_method("begin", artichoke_matchdata_begin, sys::mrb_args_req(1))?
        .add_method(
            "captures",
            artichoke_matchdata_captures,
            sys::mrb_args_none(),
        )?
        .add_method(
            "[]",
            artichoke_matchdata_element_reference,
            sys::mrb_args_req_and_opt(1, 1),
        )?
        .add_method("length", artichoke_matchdata_length, sys::mrb_args_none())?
        .add_method(
            "named_captures",
            artichoke_matchdata_named_captures,
            sys::mrb_args_none(),
        )?
        .add_method("names", artichoke_matchdata_names, sys::mrb_args_none())?
        .add_method("offset", artichoke_matchdata_offset, sys::mrb_args_req(1))?
        .add_method(
            "post_match",
            artichoke_matchdata_post_match,
            sys::mrb_args_none(),
        )?
        .add_method(
            "pre_match",
            artichoke_matchdata_pre_match,
            sys::mrb_args_none(),
        )?
        .add_method("regexp", artichoke_matchdata_regexp, sys::mrb_args_none())?
        .add_method("size", artichoke_matchdata_length, sys::mrb_args_none())?
        .add_method("string", artichoke_matchdata_string, sys::mrb_args_none())?
        .add_method("to_a", artichoke_matchdata_to_a, sys::mrb_args_none())?
        .add_method("to_s", artichoke_matchdata_to_s, sys::mrb_args_none())?
        .add_method("end", artichoke_matchdata_end, sys::mrb_args_req(1))?
        .define()?;
    interp.def_class::<matchdata::MatchData>(spec)?;
    let _ = interp.eval(&include_bytes!("matchdata.rb")[..])?;
    trace!("Patched MatchData onto interpreter");
    Ok(())
}

unsafe extern "C" fn artichoke_matchdata_begin(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let begin = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let begin = Value::new(&interp, begin);
    let result = trampoline::begin(&mut interp, value, begin);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_captures(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::captures(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_element_reference(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let (elem, len) = mrb_get_args!(mrb, required = 1, optional = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let elem = Value::new(&interp, elem);
    let len = len.map(|len| Value::new(&interp, len));
    let result = trampoline::element_reference(&mut interp, value, elem, len);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_end(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let end = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let end = Value::new(&interp, end);
    let result = trampoline::end(&mut interp, value, end);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_length(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::length(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_named_captures(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::named_captures(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_names(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::names(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_offset(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let offset = mrb_get_args!(mrb, required = 1);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let offset = Value::new(&interp, offset);
    let result = trampoline::offset(&mut interp, value, offset);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_post_match(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::post_match(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_pre_match(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::pre_match(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_regexp(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::regexp(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_string(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::string(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_to_a(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::to_a(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_matchdata_to_s(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::to_s(&mut interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
