//! [ruby/spec](https://github.com/ruby/spec) compliant implementation of
//! [`Regexp`](https://ruby-doc.org/core-2.6.3/Regexp.html).
//!
//! Each function on `Regexp` is implemented as its own module which contains
//! the `Args` struct for invoking the function.

use onig::{self, Syntax};
use regex;
use std::hash::{Hash, Hasher};
use std::mem;
use std::rc::Rc;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::exception::{RubyException, RuntimeError, SyntaxError, TypeError};
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
            interp.convert(self.literal_pattern.as_str()).inner(),
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
        let interp = unwrap_interpreter!(mrb);
        let result = initialize::Args::extract(&interp)
            .and_then(|args| initialize::method(&interp, args, slf));
        match result {
            Ok(value) => value.inner(),
            Err(initialize::Error::NoImplicitConversionToString) => {
                TypeError::raise(interp, "no implicit conversion into String");
                unreachable!("raise unwinds the stack with longjmp");
            }
            Err(initialize::Error::Syntax) => {
                SyntaxError::raise(interp, "Failed to parse Regexp pattern");
                unreachable!("raise unwinds the stack with longjmp");
            }
            Err(initialize::Error::Unicode) => {
                RuntimeError::raise(interp, "Pattern is invalid UTF-8");
                unreachable!("raise unwinds the stack with longjmp");
            }
            Err(initialize::Error::Fatal) => {
                RuntimeError::raise(interp, "Fatal Regexp#initialize error");
                unreachable!("raise unwinds the stack with longjmp");
            }
        }
    }

    unsafe extern "C" fn compile(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let mut args = <mem::MaybeUninit<*const sys::mrb_value>>::uninit();
        let mut count = <mem::MaybeUninit<sys::mrb_int>>::uninit();
        sys::mrb_get_args(
            mrb,
            b"*\0".as_ptr() as *const i8,
            args.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        sys::mrb_obj_new(
            mrb,
            sys::mrb_sys_class_ptr(slf),
            count.assume_init(),
            args.assume_init(),
        )
    }

    unsafe extern "C" fn escape(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let result = escape::Args::extract(&interp).and_then(|args| escape::method(&interp, &args));
        match result {
            Ok(result) => result.inner(),
            Err(escape::Error::NoImplicitConversionToString) => {
                TypeError::raise(interp, "no implicit conversion into String")
            }
            Err(escape::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp::escape error"),
        }
    }

    unsafe extern "C" fn union(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let args = union::Args::extract(&interp);
        let result = union::method(&interp, args, slf);
        match result {
            Ok(result) => result.inner(),
            Err(union::Error::NoImplicitConversionToString) => {
                TypeError::raise(interp, "no implicit conversion into String")
            }
            Err(union::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp::union error"),
        }
    }

    unsafe extern "C" fn match_q(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result =
            match_q::Args::extract(&interp).and_then(|args| match_q::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(match_q::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#match? error"),
            Err(match_q::Error::PosType) => {
                TypeError::raise(interp, "No implicit conversion into Integer")
            }
            Err(match_q::Error::StringType) => {
                TypeError::raise(interp, "No implicit conversion into String")
            }
        }
    }

    unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        interp.0.borrow_mut();
        let value = Value::new(&interp, slf);
        let result =
            match_::Args::extract(&interp).and_then(|args| match_::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(match_::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#match error"),
            Err(match_::Error::PosType) => {
                TypeError::raise(interp, "No implicit conversion into Integer")
            }
            Err(match_::Error::StringType) => {
                TypeError::raise(interp, "No implicit conversion into String")
            }
        }
    }

    unsafe extern "C" fn eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let args = eql::Args::extract(&interp);
        let value = Value::new(&interp, slf);
        match eql::method(&interp, args, &value) {
            Ok(result) => result.inner(),
            Err(eql::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#== error"),
        }
    }

    unsafe extern "C" fn case_compare(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = case_compare::Args::extract(&interp)
            .and_then(|args| case_compare::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(case_compare::Error::NoImplicitConversionToString) => interp.convert(false).inner(),
            Err(case_compare::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal Regexp#=== error")
            }
        }
    }

    unsafe extern "C" fn match_operator(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        let result = match_operator::Args::extract(&interp)
            .and_then(|args| match_operator::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(match_operator::Error::NoImplicitConversionToString) => {
                interp.convert(false).inner()
            }
            Err(match_operator::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal Regexp#=== error")
            }
        }
    }

    unsafe extern "C" fn casefold(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match casefold::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(casefold::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal Regexp#casefold? error")
            }
        }
    }

    unsafe extern "C" fn fixed_encoding(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match fixed_encoding::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(fixed_encoding::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal Regexp#fixed_encoding? error")
            }
        }
    }

    unsafe extern "C" fn hash(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match hash::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(hash::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#hash error"),
        }
    }

    unsafe extern "C" fn inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match inspect::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(inspect::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#inspect error"),
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
            Err(named_captures::Error::Fatal) => {
                RuntimeError::raise(interp, "fatal Regexp#named_captures error")
            }
        }
    }

    unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match names::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(names::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#names error"),
        }
    }

    unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match options::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(options::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#options error"),
        }
    }

    unsafe extern "C" fn source(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match source::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(source::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#source error"),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let value = Value::new(&interp, slf);
        match to_s::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(to_s::Error::Fatal) => RuntimeError::raise(interp, "fatal Regexp#to_s error"),
        }
    }
}
