//! [ruby/spec](https://github.com/ruby/spec) compliant implementation of
//! [`Regexp`](https://ruby-doc.org/core-2.6.3/Regexp.html).
//!
//! Each function on `Regexp` is implemented as its own module which contains
//! the `Args` struct for invoking the function.

use onig::{self, Syntax};
use regex;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::exception;
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub mod enc;
pub mod opts;
pub mod syntax;

pub mod case_compare;
pub mod casefold;
pub mod eql;
pub mod escape;
pub mod fixed_encoding;
pub mod hash;
pub mod initialize;
pub mod inspect;
pub mod match_;
pub mod match_operator;
pub mod match_q;
pub mod named_captures;
pub mod names;
pub mod options;
pub mod source;
pub mod to_s;
pub mod union;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.eval(include_str!("regexp.rb"))?;
    let regexp =
        interp
            .0
            .borrow_mut()
            .def_class::<Regexp>("Regexp", None, Some(rust_data_free::<Regexp>));
    regexp.borrow_mut().mrb_value_is_rust_backed(true);
    regexp.borrow_mut().add_method(
        "initialize",
        Regexp::initialize,
        sys::mrb_args_req_and_opt(1, 2),
    );
    regexp
        .borrow_mut()
        .add_self_method("compile", Regexp::compile, sys::mrb_args_rest());
    regexp
        .borrow_mut()
        .add_self_method("escape", Regexp::escape, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_self_method("quote", Regexp::escape, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_self_method("union", Regexp::union, sys::mrb_args_rest());
    regexp
        .borrow_mut()
        .add_method("==", Regexp::eql, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("===", Regexp::case_compare, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("=~", Regexp::match_operator, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("casefold?", Regexp::casefold, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("eql?", Regexp::eql, sys::mrb_args_req(1));
    regexp.borrow_mut().add_method(
        "fixed_encoding?",
        Regexp::fixed_encoding,
        sys::mrb_args_none(),
    );
    regexp
        .borrow_mut()
        .add_method("hash", Regexp::hash, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("inspect", Regexp::inspect, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("match?", Regexp::match_q, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("match", Regexp::match_, sys::mrb_args_req_and_opt(1, 1));
    regexp.borrow_mut().add_method(
        "named_captures",
        Regexp::named_captures,
        sys::mrb_args_none(),
    );
    regexp
        .borrow_mut()
        .add_method("names", Regexp::names, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("options", Regexp::options, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("source", Regexp::source, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("to_s", Regexp::to_s, sys::mrb_args_none());
    regexp.borrow().define(&interp)?;
    // TODO: Add proper constant defs to class::Spec, see GH-158.
    interp.eval(format!(
        "class Regexp; IGNORECASE = {}; EXTENDED = {}; MULTILINE = {}; FIXEDENCODING = {}; NOENCODING = {}; end",
        Regexp::IGNORECASE,
        Regexp::EXTENDED,
        Regexp::MULTILINE,
        Regexp::FIXEDENCODING,
        Regexp::NOENCODING,
    ))?;
    Ok(())
}

#[derive(Debug)]
pub enum Backend {
    Onig(onig::Regex),
    Rust(regex::Regex),
}

#[derive(Debug, Clone, Default)]
pub struct Regexp {
    literal_pattern: String,
    pattern: String,
    literal_options: opts::Options,
    options: opts::Options,
    encoding: enc::Encoding,
    pub regex: Rc<Option<Backend>>,
}

impl Hash for Regexp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.literal_pattern.hash(state);
        self.literal_options.hash(state);
    }
}

impl RustBackedValue for Regexp {
    fn new_obj_args(&self, interp: &Artichoke) -> Vec<sys::mrb_value> {
        let literal_options =
            // use try_convert to support 32-bit Int.
            interp.try_convert(self.literal_options.flags().bits())
                .unwrap()
                .inner();
        vec![
            interp.convert(self.literal_pattern.as_bytes()).inner(),
            literal_options,
            interp.convert(self.encoding.flags()).inner(),
        ]
    }
}

impl Regexp {
    pub const IGNORECASE: Int = 1;
    pub const EXTENDED: Int = 2;
    pub const MULTILINE: Int = 4;

    pub const ALL_REGEXP_OPTS: Int = Self::IGNORECASE | Self::EXTENDED | Self::MULTILINE;

    pub const FIXEDENCODING: Int = 16;
    pub const NOENCODING: Int = 32;

    pub const ALL_ENCODING_OPTS: Int = Self::FIXEDENCODING | Self::NOENCODING;

    pub fn new(
        literal_pattern: String,
        pattern: String,
        literal_options: opts::Options,
        options: opts::Options,
        encoding: enc::Encoding,
    ) -> Option<Self> {
        let regex = Backend::Onig(
            onig::Regex::with_options(&pattern, options.flags(), Syntax::ruby()).ok()?,
        );
        let regex = Rc::new(Some(regex));
        let regexp = Self {
            literal_pattern,
            pattern,
            literal_options,
            options,
            encoding,
            regex,
        };
        Some(regexp)
    }

    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let (pattern, options, encoding) = mrb_get_args!(mrb, required = 1, optional = 2);
        let interp = unwrap_interpreter!(mrb);
        let result = initialize::Args::extract(
            &interp,
            Value::new(&interp, pattern),
            options.map(|options| Value::new(&interp, options)),
            encoding.map(|encoding| Value::new(&interp, encoding)),
        )
        .and_then(|args| initialize::method(&interp, args, Some(slf)));
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn compile(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let args = mrb_get_args!(mrb, *args);
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
        let result = escape::Args::extract(&interp, Value::new(&interp, pattern))
            .and_then(|args| escape::method(&interp, &args));
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
        let result = union::method(&interp, args.as_slice());
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn match_q(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let (pattern, pos) = mrb_get_args!(mrb, required = 1, optional = 1);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = match_q::Args::extract(
            &interp,
            Value::new(&interp, pattern),
            pos.map(|pos| Value::new(&interp, pos)),
        )
        .and_then(|args| match_q::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let (pattern, pos, block) = mrb_get_args!(mrb, required = 1, optional = 1, &block);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = match_::Args::extract(
            &interp,
            Value::new(&interp, pattern),
            pos.map(|pos| Value::new(&interp, pos)),
            block,
        )
        .and_then(|args| match_::method(&interp, args, &value));
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
        let result = eql::method(&interp, &value, &other);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn case_compare(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let pattern = mrb_get_args!(mrb, required = 1);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let args = case_compare::Args::extract(Value::new(&interp, pattern));
        let result = case_compare::method(&interp, args, &value);
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
        let result = match_operator::Args::extract(&interp, Value::new(&interp, pattern))
            .and_then(|args| match_operator::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn casefold(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = casefold::method(&interp, &value);
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
        let result = fixed_encoding::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = hash::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = inspect::method(&interp, &value);
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
        let result = named_captures::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = names::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = options::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn source(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = source::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        mrb_get_args!(mrb, none);
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = to_s::method(&interp, &value);
        match result {
            Ok(result) => result.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }
}
