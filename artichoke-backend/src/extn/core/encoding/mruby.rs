use std::ffi::CStr;

use super::{trampoline, Encoding};
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
        .add_self_method("default_external", encoding_self_default_external, sys::mrb_args_none())?
        .add_self_method(
            "default_external=",
            encoding_self_default_external_set,
            sys::mrb_args_none(),
        )?
        .add_self_method("default_internal", encoding_self_default_internal, sys::mrb_args_none())?
        .add_self_method(
            "default_internal=",
            encoding_self_default_internal_set,
            sys::mrb_args_none(),
        )?
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
        .add_method("to_s", encoding_to_s, sys::mrb_args_none())?
        .define()?;

    interp.def_class::<Encoding>(spec)?;
    interp.eval(ENCODING_RUBY_SOURCE)?;

    // Def all the constants
    let encoding_utf8 = Encoding::Utf8;
    let encoding_utf8 = Encoding::alloc_value(encoding_utf8, interp)
        .map_err(|_| NotDefinedError::class_constant("Encoding::UTF_8_RS"))?;
    interp.define_class_constant::<Encoding>("UTF_8_RS", encoding_utf8)?;

    Ok(())
}

unsafe extern "C" fn encoding_self_aliases(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_compatible(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_default_external(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_default_external_set(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_default_internal(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_default_internal_set(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_find(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_list(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_locale_charmap(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_self_name_list(_: *mut sys::mrb_state, _: sys::mrb_value) -> sys::mrb_value {
    todo!()
}

unsafe extern "C" fn encoding_ascii_compatible(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::ascii_compatible(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_dummy(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::dummy(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::inspect(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_name(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::name(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::names(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_replicate(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::replicate(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn encoding_to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let encoding = Value::from(slf);
    let result = trampoline::to_s(&mut guard, encoding);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
