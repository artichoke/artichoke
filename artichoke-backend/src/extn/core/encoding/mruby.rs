use std::ffi::CStr;

use super::{trampoline, trampoline::AVAILABLE_ENCODINGS, Encoding};
use crate::extn::prelude::*;

const ENCODING_CSTR: &CStr = qed::const_cstr_from_str!("Encoding\0");
static ENCODING_RUBY_SOURCE: &[u8] = include_bytes!("encoding.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Encoding>() {
        return Ok(());
    }

    let spec = class::Spec::new("Encoding", ENCODING_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec)
        .value_is_rust_object()
        .add_self_method("aliases", encoding_self_aliases, sys::mrb_args_none())?
        .add_self_method("compatible?", encoding_self_compatible, sys::mrb_args_req(2))?
        .add_self_method("find", encoding_self_find, sys::mrb_args_req(1))?
        .add_self_method("list", encoding_self_list, sys::mrb_args_none())?
        .add_self_method("locale_charmap", encoding_self_locale_charmap, sys::mrb_args_none())?
        .add_self_method("name_list", encoding_self_name_list, sys::mrb_args_none())?
        .add_method("ascii_compatible?", encoding_ascii_compatible, sys::mrb_args_none())?
        .add_method("dummy?", encoding_dummy, sys::mrb_args_none())?
        .add_method("inspect", encoding_inspect, sys::mrb_args_none())?
        .add_method("name", encoding_name, sys::mrb_args_none())?
        .add_method("names", encoding_names, sys::mrb_args_none())?
        .add_method("replicate", encoding_replicate, sys::mrb_args_req(1))?
        .add_method("to_s", encoding_name, sys::mrb_args_none())?
        .define()?;

    interp.def_class::<Encoding>(spec)?;
    interp.eval(ENCODING_RUBY_SOURCE)?;

    // Setup the constants for the encodings. Multiple constants point to the
    // same object in memory.
    //
    // ```irb
    // 3.1.2 > Encoding::UTF_8.names
    // => ["UTF-8", "CP65001", "locale", "external", "filesystem"]
    // 3.1.2 > Encoding::UTF_8.object_id == Encoding::CP65001.object_id
    // => true
    // ```
    for encoding in AVAILABLE_ENCODINGS {
        interp.def_encoding(encoding)?;
    }

    Ok(())
}

unsafe extern "C" fn encoding_self_aliases(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::aliases(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_self_compatible(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let (lhs, rhs) = mrb_get_args!(mrb, required = 2);
    unwrap_interpreter!(mrb, to => guard);
    let lhs = Value::from(lhs);
    let rhs = Value::from(rhs);
    let result = trampoline::compatible(&mut guard, lhs, rhs);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_self_find(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let encoding = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(encoding);
    let result = trampoline::find(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_self_list(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::list(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_self_locale_charmap(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::locale_charmap(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_self_name_list(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let result = trampoline::name_list(&mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_ascii_compatible(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let mut encoding = Value::from(slf);
    let result = trampoline::ascii_compatible(&mut guard, &mut encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_dummy(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let mut encoding = Value::from(slf);
    let result = trampoline::dummy(&mut guard, &mut encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let mut encoding = Value::from(slf);
    let result = trampoline::inspect(&mut guard, &mut encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_name(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let mut encoding = Value::from(slf);
    let result = trampoline::name(&mut guard, &mut encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let mut encoding = Value::from(slf);
    let result = trampoline::names(&mut guard, &mut encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_replicate(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let target = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let target = Value::from(target);
    let result = trampoline::replicate(&mut guard, encoding, target);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
