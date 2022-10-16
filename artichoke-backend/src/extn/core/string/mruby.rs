use std::ffi::CStr;

use crate::extn::core::string::{self, trampoline};
use crate::extn::prelude::*;

const STRING_CSTR: &CStr = qed::const_cstr_from_str!("String\0");
static STRING_RUBY_SOURCE: &[u8] = include_bytes!("string.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<string::String>() {
        return Ok(());
    }

    let spec = class::Spec::new("String", STRING_CSTR, None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("*", string_mul, sys::mrb_args_req(1))?
        .add_method("+", string_add, sys::mrb_args_req(1))?
        .add_method("<<", string_append, sys::mrb_args_req(1))?
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
        .add_method("end_with?", string_end_with, sys::mrb_args_rest())?
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
        .add_method("rindex", string_rindex, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("scan", string_scan, sys::mrb_args_req(1))?
        .add_method("setbyte", string_setbyte, sys::mrb_args_req(2))?
        .add_method("size", string_length, sys::mrb_args_none())?
        .add_method("slice", string_aref, sys::mrb_args_req(1))?
        .add_method("slice!", string_slice_bang, sys::mrb_args_req(1))?
        .add_method("split", string_split, sys::mrb_args_opt(2))?
        .add_method("start_with?", string_start_with, sys::mrb_args_rest())?
        .add_method("to_f", string_to_f, sys::mrb_args_none())?
        .add_method("to_i", string_to_i, sys::mrb_args_opt(1))?
        .add_method("to_s", string_to_s, sys::mrb_args_none())?
        .add_method("to_sym", string_intern, sys::mrb_args_none())?
        .add_method("upcase", string_upcase, sys::mrb_args_any())?
        .add_method("upcase!", string_upcase_bang, sys::mrb_args_any())?
        .add_method("valid_encoding?", string_valid_encoding, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<string::String>(spec)?;
    interp.eval(STRING_RUBY_SOURCE)?;

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

unsafe extern "C" fn string_append(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::append(&mut guard, value, other);
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

unsafe extern "C" fn string_aref(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (first, second) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let first = Value::from(first);
    let second = second.map(Value::from);
    let result = trampoline::aref(&mut guard, value, first, second);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_aset(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::aset(&mut guard, value);
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

unsafe extern "C" fn string_byteslice(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (index, length) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let index = Value::from(index);
    let length = length.map(Value::from);
    let result = trampoline::byteslice(&mut guard, value, index, length);
    match result {
        Ok(value) if value.is_nil() => value.inner(),
        Ok(value) => {
            let rclass = sys::mrb_sys_class_of_value(mrb, slf);
            let value = value.inner();
            let target_rbasic = value.value.p.cast::<sys::RBasic>();

            // Copy `RClass` from source class to newly allocated `Array`.
            (*target_rbasic).c = rclass;

            value
        }
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

unsafe extern "C" fn string_chars(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::chars(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_chomp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let separator = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let separator = separator.map(Value::from);
    let result = trampoline::chomp(&mut guard, value, separator);
    match result {
        Ok(value) => {
            let rclass = sys::mrb_sys_class_of_value(mrb, slf);
            let value = value.inner();
            let target_rbasic = value.value.p.cast::<sys::RBasic>();

            // Copy `RClass` from source class to newly allocated `Array`.
            (*target_rbasic).c = rclass;

            value
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_chomp_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let separator = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let separator = separator.map(Value::from);
    let result = trampoline::chomp_bang(&mut guard, value, separator);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_chop(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::chop(&mut guard, value);
    match result {
        Ok(value) => {
            let rclass = sys::mrb_sys_class_of_value(mrb, slf);
            let value = value.inner();
            let target_rbasic = value.value.p.cast::<sys::RBasic>();

            // Copy `RClass` from source class to newly allocated `Array`.
            (*target_rbasic).c = rclass;

            value
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_chop_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::chop_bang(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_chr(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::chr(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_clear(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::clear(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_codepoints(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::codepoints(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_concat(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::concat(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_downcase(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::downcase(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_downcase_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::downcase_bang(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_empty(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::is_empty(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_end_with(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let prefixes = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);

    for prefix in prefixes.iter().map(|&other| Value::from(other)) {
        match trampoline::end_with(&mut guard, value, prefix) {
            Ok(true) => return guard.convert(true).inner(),
            Ok(false) => {}
            Err(exception) => error::raise(guard, exception),
        }
    }
    guard.convert(false).inner()
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

unsafe extern "C" fn string_getbyte(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let index = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let index = Value::from(index);
    let result = trampoline::getbyte(&mut guard, value, index);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::hash(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_include(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::include(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_index(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (needle, offset) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let needle = Value::from(needle);
    let offset = offset.map(Value::from);
    let result = trampoline::index(&mut guard, value, needle, offset);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let s = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let s = s.map(Value::from);
    let result = trampoline::initialize(&mut guard, value, s);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_initialize_copy(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::initialize_copy(&mut guard, value, other);
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

unsafe extern "C" fn string_intern(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::intern(&mut guard, value);
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

unsafe extern "C" fn string_replace(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let other = Value::from(other);
    let result = trampoline::replace(&mut guard, value, other);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_reverse(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::reverse(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_reverse_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::reverse_bang(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_rindex(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (needle, offset) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let needle = Value::from(needle);
    let offset = offset.map(Value::from);
    let result = trampoline::rindex(&mut guard, value, needle, offset);
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

unsafe extern "C" fn string_setbyte(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (index, byte) = mrb_get_args!(mrb, required = 2);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let index = Value::from(index);
    let byte = Value::from(byte);
    let result = trampoline::setbyte(&mut guard, value, index, byte);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_slice_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::slice_bang(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_split(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::split(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_start_with(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let prefixes = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);

    for prefix in prefixes.iter().map(|&other| Value::from(other)) {
        match trampoline::start_with(&mut guard, value, prefix) {
            Ok(true) => return guard.convert(true).inner(),
            Ok(false) => {}
            Err(exception) => error::raise(guard, exception),
        }
    }
    guard.convert(false).inner()
}

unsafe extern "C" fn string_to_f(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::to_f(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_to_i(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let base = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let base = base.map(Value::from);
    let result = trampoline::to_i(&mut guard, value, base);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    // TODO: dup `slf` when self is a subclass of `String`.
    let result = trampoline::to_s(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_upcase(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::upcase(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_upcase_bang(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::upcase_bang(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn string_valid_encoding(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let value = Value::from(slf);
    let result = trampoline::is_valid_encoding(&mut guard, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}
