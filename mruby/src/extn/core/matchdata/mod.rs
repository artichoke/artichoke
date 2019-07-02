use crate::convert::RustBackedValue;
use crate::def::{rust_data_free, ClassLike, Define};
use crate::extn::core::error::{IndexError, RubyException, RuntimeError, TypeError};
use crate::extn::core::regexp::Regexp;
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::Value;
use crate::MrbError;

mod begin;
mod captures;
mod element_reference;
mod end;
mod length;
mod named_captures;
mod post_match;
mod pre_match;
mod regexp;
mod string;
mod to_a;
mod to_s;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    let match_data = interp.borrow_mut().def_class::<MatchData>(
        "MatchData",
        None,
        Some(rust_data_free::<MatchData>),
    );
    match_data.borrow_mut().mrb_value_is_rust_backed(true);
    match_data
        .borrow_mut()
        .add_method("begin", MatchData::begin, sys::mrb_args_req(1));
    match_data
        .borrow_mut()
        .add_method("captures", MatchData::captures, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("[]", MatchData::element_reference, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("length", MatchData::length, sys::mrb_args_none());
    match_data.borrow_mut().add_method(
        "named_captures",
        MatchData::named_captures,
        sys::mrb_args_none(),
    );
    match_data
        .borrow_mut()
        .add_method("post_match", MatchData::post_match, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("pre_match", MatchData::pre_match, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("regexp", MatchData::regexp, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("size", MatchData::length, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("string", MatchData::string, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("to_a", MatchData::to_a, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("to_s", MatchData::to_s, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("end", MatchData::end, sys::mrb_args_req(1));
    match_data.borrow().define(&interp)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Region {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
pub struct MatchData {
    string: String,
    regexp: Regexp,
    region: Region,
}

impl RustBackedValue for MatchData {}

impl MatchData {
    pub fn new(string: &str, regexp: Regexp, start: usize, end: usize) -> Self {
        let region = Region { start, end };
        Self {
            string: string.to_owned(),
            regexp,
            region,
        }
    }

    pub fn set_region(&mut self, start: usize, end: usize) {
        self.region = Region { start, end };
    }

    unsafe extern "C" fn begin(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        let result =
            begin::Args::extract(&interp).and_then(|args| begin::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(begin::Error::NoMatch) | Err(begin::Error::NoGroup) => interp.nil().inner(),
            Err(begin::Error::IndexType) => TypeError::raise(&interp, "Unexpected capture group"),
            Err(begin::Error::Fatal) => RuntimeError::raise(&interp, "fatal MatchData#begin error"),
        }
    }

    unsafe extern "C" fn captures(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match captures::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(captures::Error::NoMatch) => interp.nil().inner(),
            Err(captures::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#captures error")
            }
        }
    }

    unsafe extern "C" fn element_reference(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let num_captures = match MatchData::try_from_ruby(&interp, &Value::new(&interp, slf)) {
            Ok(data) => data.borrow().regexp.regex.captures_len(),
            Err(_) => return interp.nil().inner(),
        };
        let value = Value::new(&interp, slf);
        let result = element_reference::Args::extract(&interp, num_captures)
            .and_then(|args| element_reference::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(element_reference::Error::NoMatch) => interp.nil().inner(),
            Err(element_reference::Error::NoGroup(name)) => IndexError::raise(
                &interp,
                &format!("undefined group name reference: {}", name),
            ),
            Err(element_reference::Error::IndexType)
            | Err(element_reference::Error::LengthType) => {
                TypeError::raise(&interp, "Unexpected element reference")
            }
            Err(element_reference::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#[] error")
            }
        }
    }

    unsafe extern "C" fn end(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        let result =
            end::Args::extract(&interp).and_then(|args| end::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(end::Error::NoMatch) | Err(end::Error::NoGroup) => interp.nil().inner(),
            Err(end::Error::IndexType) => TypeError::raise(&interp, "Unexpected capture group"),
            Err(end::Error::Fatal) => RuntimeError::raise(&interp, "fatal MatchData#begin error"),
        }
    }

    unsafe extern "C" fn length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match length::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(length::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#length error")
            }
        }
    }

    unsafe extern "C" fn named_captures(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match named_captures::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(named_captures::Error::NoMatch) => interp.nil().inner(),
            Err(named_captures::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#named_captures error")
            }
        }
    }

    unsafe extern "C" fn post_match(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match post_match::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(post_match::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#post_match error")
            }
        }
    }

    unsafe extern "C" fn pre_match(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match pre_match::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(pre_match::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#pre_match error")
            }
        }
    }

    unsafe extern "C" fn regexp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match regexp::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(regexp::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#regexp error")
            }
        }
    }

    unsafe extern "C" fn string(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match string::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(string::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal MatchData#string error")
            }
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_a(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match to_a::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(to_a::Error::NoMatch) => interp.nil().inner(),
            Err(to_a::Error::Fatal) => RuntimeError::raise(&interp, "fatal MatchData#to_a error"),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match to_s::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(to_s::Error::NoMatch) => interp.string("").inner(),
            Err(to_s::Error::Fatal) => RuntimeError::raise(&interp, "fatal MatchData#to_s error"),
        }
    }
}
