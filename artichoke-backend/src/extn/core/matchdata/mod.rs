//! [ruby/spec](https://github.com/ruby/spec) compliant implementation of
//! [`MatchData`](https://ruby-doc.org/core-2.6.3/MatchData.html).
//!
//! Each function on `MatchData` is implemented as its own module which contains
//! the `Args` struct for invoking the function.
//!
//! [`MatchData#==`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-3D-3D),
//! [`MatchData#eql?`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-eql-3F),
//! [`MatchData#inspect`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-inspect),
//! and
//! [`MatchData#values_at`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-values_at)
//! are
//! [implemented in Ruby](https://github.com/artichoke/artichoke/blob/master/artichoke-backend/src/extn/core/matchdata/matchdata.rb).

use crate::convert::{Convert, RustBackedValue};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::error::{IndexError, RubyException, RuntimeError, TypeError};
use crate::extn::core::regexp::{Backend, Regexp};
use crate::sys;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub mod begin;
pub mod captures;
pub mod element_reference;
pub mod end;
pub mod length;
pub mod named_captures;
pub mod names;
pub mod offset;
pub mod post_match;
pub mod pre_match;
pub mod regexp;
pub mod string;
pub mod to_a;
pub mod to_s;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let match_data = interp.borrow_mut().def_class::<MatchData>(
        "MatchData",
        None,
        Some(rust_data_free::<MatchData>),
    );
    match_data.borrow_mut().mrb_value_is_rust_backed(true);
    interp.eval(include_str!("matchdata.rb"))?;
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
        .add_method("names", MatchData::names, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("offset", MatchData::offset, sys::mrb_args_req(1));
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
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result =
            begin::Args::extract(&interp).and_then(|args| begin::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(begin::Error::NoMatch) | Err(begin::Error::NoGroup) => sys::mrb_sys_nil_value(),
            Err(begin::Error::IndexType) => TypeError::raise(interp, "Unexpected capture group"),
            Err(begin::Error::Fatal) => RuntimeError::raise(interp, "fatal MatchData#begin error"),
        }
    }

    unsafe extern "C" fn captures(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match captures::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(captures::Error::NoMatch) => sys::mrb_sys_nil_value(),
            Err(captures::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#captures error")
            }
        }
    }

    unsafe extern "C" fn element_reference(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let num_captures = match Self::try_from_ruby(&interp, &Value::new(&interp, slf)) {
            Ok(data) => {
                if let Some(regex) = (*data.borrow().regexp.regex).as_ref() {
                    let Backend::Onig(regex) = regex;
                    regex.captures_len()
                } else {
                    0
                }
            }
            Err(_) => return sys::mrb_sys_nil_value(),
        };
        let value = Value::new(&interp, slf);
        let result = element_reference::Args::extract(&interp, num_captures)
            .and_then(|args| element_reference::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(element_reference::Error::NoMatch) => sys::mrb_sys_nil_value(),
            Err(element_reference::Error::NoGroup(name)) => {
                IndexError::raisef(interp, "undefined group name reference: %S", vec![name])
            }
            Err(element_reference::Error::IndexType)
            | Err(element_reference::Error::LengthType) => {
                TypeError::raise(interp, "Unexpected element reference")
            }
            Err(element_reference::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#[] error")
            }
        }
    }

    unsafe extern "C" fn end(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result =
            end::Args::extract(&interp).and_then(|args| end::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(end::Error::NoMatch) | Err(end::Error::NoGroup) => sys::mrb_sys_nil_value(),
            Err(end::Error::IndexType) => TypeError::raise(interp, "Unexpected capture group"),
            Err(end::Error::Fatal) => RuntimeError::raise(interp, "fatal MatchData#begin error"),
        }
    }

    unsafe extern "C" fn length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match length::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(length::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#length error")
            }
        }
    }

    unsafe extern "C" fn named_captures(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match named_captures::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(named_captures::Error::NoMatch) => sys::mrb_sys_nil_value(),
            Err(named_captures::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#named_captures error")
            }
        }
    }

    unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match names::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(names::Error::Fatal) => RuntimeError::raise(interp, "fatal MatchData#names error"),
        }
    }

    unsafe extern "C" fn offset(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result =
            offset::Args::extract(&interp).and_then(|args| offset::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(offset::Error::NoMatch) | Err(offset::Error::NoGroup) => {
                Value::convert(&interp, vec![None::<Value>, None::<Value>]).inner()
            }
            Err(offset::Error::IndexType) => TypeError::raise(interp, "Unexpected capture group"),
            Err(offset::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#offset error")
            }
        }
    }

    unsafe extern "C" fn post_match(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match post_match::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(post_match::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#post_match error")
            }
        }
    }

    unsafe extern "C" fn pre_match(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match pre_match::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(pre_match::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#pre_match error")
            }
        }
    }

    unsafe extern "C" fn regexp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match regexp::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(regexp::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#regexp error")
            }
        }
    }

    unsafe extern "C" fn string(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match string::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(string::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal MatchData#string error")
            }
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_a(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match to_a::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(to_a::Error::NoMatch) => sys::mrb_sys_nil_value(),
            Err(to_a::Error::Fatal) => RuntimeError::raise(interp, "fatal MatchData#to_a error"),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match to_s::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(to_s::Error::NoMatch) => Value::convert(&interp, [0_u8; 0].as_ref()).inner(),
            Err(to_s::Error::Fatal) => RuntimeError::raise(interp, "fatal MatchData#to_s error"),
        }
    }
}
