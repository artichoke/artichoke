use std::convert::TryFrom;

use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::exception;
use crate::extn::core::regexp;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.eval(&include_bytes!("regexp.rb")[..])?;
    let regexp = interp.0.borrow_mut().def_class::<regexp::Regexp>(
        "Regexp",
        None,
        Some(rust_data_free::<regexp::Regexp>),
    );
    regexp.borrow_mut().mrb_value_is_rust_backed(true);
    regexp
        .borrow_mut()
        .add_method("initialize", initialize, sys::mrb_args_req_and_opt(1, 2));
    regexp
        .borrow_mut()
        .add_self_method("compile", compile, sys::mrb_args_rest());
    regexp
        .borrow_mut()
        .add_self_method("escape", escape, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_self_method("quote", escape, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_self_method("union", union, sys::mrb_args_rest());
    regexp
        .borrow_mut()
        .add_method("==", eql, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("===", case_compare, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("=~", match_operator, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("casefold?", casefold, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("eql?", eql, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("fixed_encoding?", fixed_encoding, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("hash", hash, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("inspect", inspect, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("match?", match_q, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("match", match_, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("named_captures", named_captures, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("names", names, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("options", options, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("source", source, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("to_s", to_s, sys::mrb_args_none());
    regexp.borrow().define(&interp)?;
    // TODO: Add proper constant defs to class::Spec, see GH-27.
    interp.eval(format!(
        "class Regexp; IGNORECASE = {}; EXTENDED = {}; MULTILINE = {}; FIXEDENCODING = {}; NOENCODING = {}; end",
        regexp::IGNORECASE,
        regexp::EXTENDED,
        regexp::MULTILINE,
        regexp::FIXEDENCODING,
        regexp::NOENCODING,
    ))?;
    Ok(())
}

unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, options, encoding) = mrb_get_args!(mrb, required = 1, optional = 2);
    let interp = unwrap_interpreter!(mrb);
    let result = regexp::trampoline::initialize(
        &interp,
        Value::new(&interp, pattern),
        options.map(|options| Value::new(&interp, options)),
        encoding.map(|encoding| Value::new(&interp, encoding)),
        Some(Value::new(&interp, slf)),
    );
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn compile(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    // Call `mrb_obj_new` instead of allocating an object of class `slf` and
    // delegating to `regexp::trampoline::initialize` to handle cases where
    // subclasses override initialize.
    sys::mrb_obj_new(
        mrb,
        sys::mrb_sys_class_ptr(slf),
        Int::try_from(args.len()).unwrap_or_default(),
        args.as_ptr(),
    )
}

unsafe extern "C" fn escape(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    let interp = unwrap_interpreter!(mrb);
    let result = regexp::trampoline::escape(&interp, Value::new(&interp, pattern));
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn union(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
    let args = mrb_get_args!(mrb, *args);
    let interp = unwrap_interpreter!(mrb);
    let args = args
        .iter()
        .map(|arg| Value::new(&interp, *arg))
        .collect::<Vec<_>>();
    let result = regexp::trampoline::union(&interp, args.as_slice());
    drop(args);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn match_q(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, pos) = mrb_get_args!(mrb, required = 1, optional = 1);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::is_match(
        &interp,
        value,
        Value::new(&interp, pattern),
        pos.map(|pos| Value::new(&interp, pos)),
    );
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let (pattern, pos, block) = mrb_get_args!(mrb, required = 1, optional = 1, &block);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::match_(
        &interp,
        value,
        Value::new(&interp, pattern),
        pos.map(|pos| Value::new(&interp, pos)),
        block,
    );
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let other = Value::new(&interp, other);
    let result = regexp::trampoline::eql(&interp, value, other);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn case_compare(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::case_compare(&interp, value, Value::new(&interp, pattern));
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn match_operator(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let pattern = mrb_get_args!(mrb, required = 1);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::match_operator(&interp, value, Value::new(&interp, pattern));
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn casefold(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::is_casefold(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn fixed_encoding(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::is_fixed_encoding(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::hash(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::inspect(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn named_captures(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::named_captures(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::names(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::options(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn source(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::source(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = regexp::trampoline::to_s(&interp, value);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
