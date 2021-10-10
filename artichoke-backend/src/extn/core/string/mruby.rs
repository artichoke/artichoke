use std::ffi::CStr;

use crate::extn::core::string::{self, trampoline};
use crate::extn::prelude::*;

const STRING_CSTR: &CStr = cstr::cstr!("String");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<string::String>() {
        return Ok(());
    }
    let spec = class::Spec::new("String", STRING_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("*", string_mul, sys::mrb_args_req(1))?
        .add_method("+", string_add, sys::mrb_args_req(1))?
        .add_method("<<", string_push, sys::mrb_args_req(1))?
        .add_method("<=>", string_cmp_rocket, sys::mrb_args_req(1))?
        .add_method("==", string_equals_equals, sys::mrb_args_req(1))?
        .add_method("[]", string_aref, sys::mrb_args_req(1))?
        .add_method("[]=", string_aset, sys::mrb_args_any())?
        .add_method("ascii_only?", string_ascii_only, sys::mrb_args_none())?
        .add_method("b", string_b, sys::mrb_args_none())?
        .add_method("bytes", string_bytes, sys::mrb_args_none())? // This does not support the deprecated block form
        .add_method("bytesize", string_bytesize, sys::mrb_args_none())?
        .add_method("byteslice", string_byteslice, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("capitalize", string_capitalize, sys::mrb_args_any())?
        .add_method("capitalize!", string_capitalize_bang, sys::mrb_args_any())?
        .add_method("casecmp", string_casecmp_ascii, sys::mrb_args_req(1))?
        .add_method("casecmp?", string_casecmp_unicode, sys::mrb_args_req(1))?
        .add_method("center", string_center, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("chars", string_chars, sys::mrb_args_none())? // This does not support the deprecated block form
        .add_method("chomp", string_chomp, sys::mrb_args_opt(1))?
        .add_method("chomp!", string_chomp_bang, sys::mrb_args_opt(1))?
        .add_method("chop", string_chop, sys::mrb_args_none())?
        .add_method("chop!", string_chop_bang, sys::mrb_args_none())?
        .add_method("chr", string_chr, sys::mrb_args_none())?
        .add_method("clear", string_clear, sys::mrb_args_none())?
        .add_method("codepoints", string_codepoints, sys::mrb_args_none())? // This does not support the deprecated block form
        .add_method("concat", string_concat, sys::mrb_args_any())?
        .add_method("downcase", string_downcase, sys::mrb_args_any())?
        .add_method("downcase!", string_downcase_bang, sys::mrb_args_any())?
        .add_method("empty?", string_empty, sys::mrb_args_none())?
        .add_method("eql?", string_eql, sys::mrb_args_req(1))?
        .add_method("getbyte", string_getbyte, sys::mrb_args_req(1))?
        .add_method("hash", string_hash, sys::mrb_args_none())?
        .add_method("include?", string_include, sys::mrb_args_req(1))?
        .add_method("index", string_index, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("initialize", string_initialize, sys::mrb_args_opt(1))? // TODO: support encoding and capacity kwargs
        .add_method("initialize_copy", string_initialize_copy, sys::mrb_args_req(1))?
        .add_method("inspect", string_inspect, sys::mrb_args_none())?
        .add_method("intern", string_intern, sys::mrb_args_none())?
        .add_method("length", string_length, sys::mrb_args_none())?
        .add_method("ord", string_ord, sys::mrb_args_none())?
        .add_method("replace", string_replace, sys::mrb_args_req(1))?
        .add_method("reverse", string_reverse, sys::mrb_args_none())?
        .add_method("reverse!", string_reverse_bang, sys::mrb_args_none())?
        .add_method("scan", string_scan, sys::mrb_args_req(1))?
        .add_method("setbyte", string_setbyte, sys::mrb_args_req(2))?
        .add_method("size", string_length, sys::mrb_args_none())?
        .add_method("slice", string_aref, sys::mrb_args_req(1))?
        .add_method("split", string_split, sys::mrb_args_opt(2))?
        .add_method("to_f", string_to_f, sys::mrb_args_none())?
        .add_method("to_i", string_to_i, sys::mrb_args_opt(1))?
        .add_method("to_s", string_to_s, sys::mrb_args_none())?
        .add_method("to_str", string_to_str, sys::mrb_args_none())?
        .add_method("to_sym", string_intern, sys::mrb_args_none())?
        .add_method("upcase", string_upcase, sys::mrb_args_any())?
        .add_method("upcase!", string_upcase_bang, sys::mrb_args_any())?
        .define()?;
    interp.def_class::<string::String>(spec)?;
    // interp.eval(&include_bytes!("string.rb")[..])?;
    trace!("Patched String onto interpreter");
    Ok(())
}

unsafe extern "C" fn string_mul(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::mul(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_add(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::add(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_push(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::push(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_cmp_rocket(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::cmp_rocket(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_equals_equals(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::equals_equals(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_ascii_only(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::is_ascii_only(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_b(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::b(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_bytes(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::bytes(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_bytesize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::bytesize(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_capitalize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::capitalize(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_capitalize_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::capitalize_bang(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_casecmp_ascii(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::casecmp_ascii(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_casecmp_unicode(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::casecmp_unicode(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_center(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (width, padstr) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let width = Value::from(width);
    let padstr = padstr.map(Value::from);
    let result = trampoline::center(&mut guard, value, width, padstr);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::eql(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::inspect(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::length(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_ord(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::ord(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_scan(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, block) = mrb_get_args!(mrb, required = 1, &block);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let pattern = Value::from(pattern);
    let result = trampoline::scan(&mut guard, value, pattern, block);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    // TODO: dup `slf` when slf is a subclass of `String`.
    slf
}
