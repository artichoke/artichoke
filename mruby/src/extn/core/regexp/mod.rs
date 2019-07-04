use onig::{Regex, RegexOptions, Region, SearchOptions, Syntax};
use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::mem;
use std::rc::Rc;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::MrbEval;
use crate::extn::core::error::{RubyException, RuntimeError, SyntaxError, TypeError};
use crate::extn::core::matchdata::MatchData;
use crate::sys;
use crate::value::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::{Mrb, MrbError};

mod args;
pub mod case_compare;
pub mod casefold;
pub mod eql;
pub mod escape;
pub mod initialize;
pub mod names;
pub mod syntax;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.eval(include_str!("regexp.rb"))?;
    let regexp =
        interp
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
        .add_method("=~", Regexp::equal_squiggle, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("casefold?", Regexp::casefold, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("eql?", Regexp::eql, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("inspect", Regexp::inspect, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("match?", Regexp::is_match, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("match", Regexp::match_, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("names", Regexp::names, sys::mrb_args_none());
    regexp.borrow_mut().add_method(
        "named_captures",
        Regexp::named_captures,
        sys::mrb_args_none(),
    );
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
    // TODO: Add proper constant defs to class::Spec and undo this hack.
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Options {
    multiline: bool,
    ignore_case: bool,
    extended: bool,
}

impl Options {
    fn flags(self) -> RegexOptions {
        let mut bits = RegexOptions::REGEX_OPTION_NONE;
        if self.multiline {
            bits |= RegexOptions::REGEX_OPTION_MULTILINE;
        }
        if self.ignore_case {
            bits |= RegexOptions::REGEX_OPTION_IGNORECASE;
        }
        if self.extended {
            bits |= RegexOptions::REGEX_OPTION_EXTEND;
        }
        bits
    }

    fn as_literal_string(self) -> &'static str {
        match (self.multiline, self.ignore_case, self.extended) {
            (true, true, true) => "mix",
            (true, true, false) => "mi",
            (true, false, true) => "mx",
            (true, false, false) => "m",
            (false, true, true) => "ix",
            (false, true, false) => "i",
            (false, false, true) => "x",
            (false, false, false) => "",
        }
    }

    fn as_onig_string(self) -> &'static str {
        match (self.multiline, self.ignore_case, self.extended) {
            (true, true, true) => "mix",
            (true, true, false) => "mi-x",
            (true, false, true) => "mx-i",
            (true, false, false) => "m-ix",
            (false, true, true) => "ix-m",
            (false, true, false) => "i-mx",
            (false, false, true) => "x-mi",
            (false, false, false) => "-mix",
        }
    }

    fn from_value(interp: &Mrb, value: sys::mrb_value) -> Result<Self, MrbError> {
        // If options is an Integer, it should be one or more of the constants
        // Regexp::EXTENDED, Regexp::IGNORECASE, and Regexp::MULTILINE, or-ed
        // together. Otherwise, if options is not nil or false, the regexp will
        // be case insensitive.
        if let Ok(options) = unsafe { i64::try_from_mrb(&interp, Value::new(&interp, value)) } {
            // Only deal with Regexp opts
            let options = options & !Regexp::ALL_ENCODING_OPTS;
            if options & Regexp::ALL_REGEXP_OPTS != options {
                return Err(MrbError::Exec("Invalid Regexp flags".to_owned()));
            }
            Ok(Self {
                ignore_case: options & Regexp::IGNORECASE > 0,
                extended: options & Regexp::EXTENDED > 0,
                multiline: options & Regexp::MULTILINE > 0,
            })
        } else if let Ok(options) =
            unsafe { <Option<bool>>::try_from_mrb(&interp, Value::new(&interp, value)) }
        {
            match options {
                Some(false) | None => Ok(Self::default()),
                _ => Ok(Self::ignore_case()),
            }
        } else if let Ok(options) =
            unsafe { <Option<String>>::try_from_mrb(&interp, Value::new(&interp, value)) }
        {
            if let Some(options) = options {
                let mut opts = Self::default();
                opts.ignore_case = options.contains('i');
                opts.multiline = options.contains('m');
                opts.extended = options.contains('x');
                Ok(opts)
            } else {
                Ok(Self::default())
            }
        } else {
            Ok(Self::ignore_case())
        }
    }

    fn from_pattern(pattern: &str, mut opts: Self) -> (String, Self) {
        let orig_opts = opts;
        let mut chars = pattern.chars();
        let mut enabled = true;
        let mut pat_buf = String::new();
        let mut pointer = 0;
        match chars.next() {
            None => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string());
                pat_buf.push(':');
                pat_buf.push(')');
                return (pat_buf, opts);
            }
            Some(token) if token != '(' => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string());
                pat_buf.push(':');
                pat_buf.push_str(pattern);
                pat_buf.push(')');
                return (pat_buf, opts);
            }
            _ => (),
        };
        pointer += 1;
        match chars.next() {
            None => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string());
                pat_buf.push(':');
                pat_buf.push_str(pattern);
                pat_buf.push(')');
                return (pat_buf, opts);
            }
            Some(token) if token != '?' => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string());
                pat_buf.push(':');
                pat_buf.push_str(pattern);
                pat_buf.push(')');
                return (pat_buf, opts);
            }
            _ => (),
        };
        pointer += 1;
        for token in chars {
            pointer += 1;
            match token {
                '-' => enabled = false,
                'i' => {
                    opts.ignore_case = enabled;
                }
                'm' => {
                    opts.multiline = enabled;
                }
                'x' => {
                    opts.extended = enabled;
                }
                ':' => break,
                _ => {
                    pat_buf.push_str("(?");
                    pat_buf.push_str(opts.as_onig_string());
                    pat_buf.push(':');
                    pat_buf.push_str(pattern);
                    pat_buf.push(')');
                    return (pat_buf, opts);
                }
            }
        }
        let mut chars = pattern[pointer..].chars();
        let mut token = chars.next();
        let mut nest = 1;
        while token.is_some() {
            match token {
                Some(token) if token == '(' => nest += 1,
                Some(token) if token == ')' => {
                    nest -= 1;
                    if nest == 0 && chars.next().is_some() {
                        return (
                            format!("(?{}:{})", orig_opts.as_onig_string(), pattern),
                            orig_opts,
                        );
                    }
                    break;
                }
                _ => (),
            }
            token = chars.next();
        }

        (
            format!("(?{}:{}", opts.as_onig_string(), &pattern[pointer..]),
            opts,
        )
    }

    fn ignore_case() -> Self {
        let mut opts = Self::default();
        opts.ignore_case = true;
        opts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Fixed,
    No,
    None,
}

impl Encoding {
    fn flags(self) -> i64 {
        match self {
            Encoding::Fixed => Regexp::FIXEDENCODING,
            Encoding::No => Regexp::NOENCODING,
            Encoding::None => 0,
        }
    }

    fn as_literal_string(self) -> &'static str {
        match self {
            Encoding::Fixed | Encoding::None => "",
            Encoding::No => "n",
        }
    }

    fn from_value(
        interp: &Mrb,
        value: sys::mrb_value,
        from_options: bool,
    ) -> Result<Self, MrbError> {
        if let Ok(encoding) = unsafe { i64::try_from_mrb(&interp, Value::new(&interp, value)) } {
            // Only deal with Encoding opts
            let encoding = encoding & !Regexp::ALL_REGEXP_OPTS;
            if encoding == Regexp::FIXEDENCODING {
                Ok(Encoding::Fixed)
            } else if encoding == Regexp::NOENCODING {
                Ok(Encoding::No)
            } else if encoding == 0 {
                Ok(Self::default())
            } else {
                return Err(MrbError::Exec("Invalid Regexp encoding".to_owned()));
            }
        } else if let Ok(encoding) =
            unsafe { String::try_from_mrb(&interp, Value::new(&interp, value)) }
        {
            if encoding.contains('u') && encoding.contains('n') {
                return Err(MrbError::Exec("Invalid Regexp encoding".to_owned()));
            }
            let mut enc = vec![];
            for flag in encoding.chars() {
                if flag == 'u' || flag == 's' || flag == 'e' {
                    enc.push(Encoding::Fixed);
                } else if flag == 'n' {
                    enc.push(Encoding::No);
                } else if from_options && (flag == 'i' || flag == 'm' || flag == 'x' || flag == 'o')
                {
                    continue;
                } else {
                    return Err(MrbError::Exec("Invalid Regexp encoding".to_owned()));
                }
            }
            if enc.len() > 1 {
                return Err(MrbError::Exec("Invalid Regexp encoding".to_owned()));
            }
            Ok(enc.pop().unwrap_or_default())
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::None
    }
}

#[derive(Debug, Clone)]
pub struct Regexp {
    literal_pattern: String,
    pattern: String,
    literal_options: Options,
    options: Options,
    encoding: Encoding,
    pub regex: Rc<Regex>,
}

impl Default for Regexp {
    fn default() -> Self {
        Self {
            literal_pattern: String::default(),
            pattern: String::default(),
            literal_options: Options::default(),
            options: Options::default(),
            encoding: Encoding::default(),
            regex: Rc::new(unsafe { mem::uninitialized::<Regex>() }),
        }
    }
}

impl RustBackedValue for Regexp {
    fn new_obj_args(&self, interp: &Mrb) -> Vec<sys::mrb_value> {
        vec![
            Value::from_mrb(interp, self.literal_pattern.as_str()).inner(),
            Value::from_mrb(interp, self.literal_options.flags().bits()).inner(),
            Value::from_mrb(interp, self.encoding.flags()).inner(),
        ]
    }
}

impl Regexp {
    // TODO: expose these consts on the Regexp class in Ruby land.
    pub const IGNORECASE: i64 = 1;
    pub const EXTENDED: i64 = 2;
    pub const MULTILINE: i64 = 4;
    // mruby does not support the `o` flag: Perform #{} interpolation only once
    pub const ALL_REGEXP_OPTS: i64 = Self::IGNORECASE | Self::EXTENDED | Self::MULTILINE;

    pub const FIXEDENCODING: i64 = 16;
    pub const NOENCODING: i64 = 32;
    pub const ALL_ENCODING_OPTS: i64 = Self::FIXEDENCODING | Self::NOENCODING;

    pub fn new(
        literal_pattern: String,
        pattern: String,
        literal_options: Options,
        options: Options,
        encoding: Encoding,
    ) -> Option<Self> {
        let regex = Rc::new(Regex::with_options(&pattern, options.flags(), Syntax::ruby()).ok()?);
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
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            initialize::Args::extract(&interp),
            sys::mrb_sys_nil_value()
        );
        match initialize::method(&interp, slf, args) {
            Ok(value) => value.inner(),
            // Err(initialize::Error::Syntax) => SyntaxError::raise(&interp, ""),
            Err(initialize::Error::NoImplicitConversionToString) => {
                TypeError::raise(&interp, "no implicit conversion into String");
                unwrap_value_or_raise!(interp, Self::default().try_into_ruby(&interp, Some(slf)))
            }
            Err(initialize::Error::Syntax) => {
                SyntaxError::raise(&interp, "");
                unwrap_value_or_raise!(interp, Self::default().try_into_ruby(&interp, Some(slf)))
            }
            _ => {
                RuntimeError::raise(&interp, "");
                unwrap_value_or_raise!(interp, Self::default().try_into_ruby(&interp, Some(slf)))
            }
        }
    }

    unsafe extern "C" fn compile(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let args = mem::uninitialized::<*const sys::mrb_value>();
        let count = mem::uninitialized::<sys::mrb_int>();
        sys::mrb_get_args(mrb, b"*\0".as_ptr() as *const i8, &args, &count);
        sys::mrb_obj_new(mrb, sys::mrb_sys_class_ptr(slf), count, args)
    }

    unsafe extern "C" fn escape(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let result = escape::Args::extract(&interp).and_then(|args| escape::method(&interp, args));
        match result {
            Ok(result) => result.inner(),
            Err(escape::Error::BadType) => {
                TypeError::raise(&interp, "no implicit conversion into String")
            }
            Err(escape::Error::Fatal) => RuntimeError::raise(&interp, "fatal Regexp#escape error"),
        }
    }

    unsafe extern "C" fn union(
        mrb: *mut sys::mrb_state,
        mut _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let mut args = unwrap_or_raise!(
            interp,
            args::Rest::extract(&interp),
            sys::mrb_sys_nil_value()
        );
        let pattern = if args.rest.is_empty() {
            "(?!)".to_owned()
        } else {
            let patterns = if args.rest.len() == 1 {
                let arg = args.rest.remove(0);
                if arg.ruby_type() == Ruby::Array {
                    unwrap_or_raise!(
                        interp,
                        arg.try_into::<Vec<Value>>(),
                        sys::mrb_sys_nil_value()
                    )
                } else {
                    vec![arg]
                }
            } else {
                args.rest
            };
            let mut raw_patterns = vec![];
            for pattern in patterns {
                if let Ok(regexp) = Self::try_from_ruby(&interp, &pattern) {
                    raw_patterns.push(regexp.borrow().pattern.clone());
                } else if let Ok(Some(pattern)) =
                    pattern.funcall::<Option<String>, _, _>("to_str", &[])
                {
                    raw_patterns.push(syntax::escape(pattern.as_str()));
                } else {
                    return TypeError::raise(&interp, "no implicit conversion to String");
                }
            }
            raw_patterns.join("|")
        };

        // TODO: Preserve Regexp options per the docs if the args are Regexps.
        let literal_options = Options::default();
        let literal_pattern = pattern;
        let (pattern, options) = Options::from_pattern(literal_pattern.as_str(), literal_options);
        if let Some(data) = Self::new(
            literal_pattern,
            pattern,
            literal_options,
            options,
            Encoding::default(),
        ) {
            unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, None))
        } else {
            // Regexp is invalid.
            SyntaxError::raise(&interp, "malformed Regexp")
        }
    }

    unsafe extern "C" fn is_match(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Match::extract(&interp),
            sys::mrb_sys_nil_value()
        );

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return TypeError::raise(&interp, "No implicit conversion into String"),
            _ => return sys::mrb_sys_nil_value(),
        };

        let pos = args.pos.unwrap_or_default();
        let pos = if pos < 0 {
            let strlen = i64::try_from(string.len()).unwrap_or_default();
            let pos = strlen + pos;
            if pos < 0 {
                return sys::mrb_sys_nil_value();
            }
            usize::try_from(pos).expect("positive i64 must be usize")
        } else {
            usize::try_from(pos).expect("positive i64 must be usize")
        };
        // onig will panic if pos is beyond the end of string
        if pos > string.len() {
            return Value::from_mrb(&interp, false).inner();
        }
        let is_match = data.borrow().regex.search_with_options(
            string.as_str(),
            pos,
            string.len(),
            SearchOptions::SEARCH_OPTION_NONE,
            None,
        );
        Value::from_mrb(&interp, is_match.is_some()).inner()
    }

    unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Match::extract(&interp),
            sys::mrb_sys_nil_value()
        );

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return TypeError::raise(&interp, "No implicit conversion into String"),
            _ => {
                sys::mrb_gv_set(
                    mrb,
                    interp.borrow_mut().sym_intern("$~"),
                    sys::mrb_sys_nil_value(),
                );
                return sys::mrb_sys_nil_value();
            }
        };

        let pos = args.pos.unwrap_or_default();
        let pos = if pos < 0 {
            let strlen = i64::try_from(string.len()).unwrap_or_default();
            let pos = strlen + pos;
            if pos < 0 {
                return sys::mrb_sys_nil_value();
            }
            usize::try_from(pos).expect("positive i64 must be usize")
        } else {
            usize::try_from(pos).expect("positive i64 must be usize")
        };
        // onig will panic if pos is beyond the end of string
        if pos > string.len() {
            return sys::mrb_sys_nil_value();
        }
        let mut region = Region::new();
        let is_match = regexp.borrow().regex.search_with_options(
            string.as_str(),
            pos,
            string.len(),
            SearchOptions::SEARCH_OPTION_NONE,
            Some(&mut region),
        );
        let last_matched_string = if let Some((start, end)) = region.pos(0) {
            Value::from_mrb(&interp, string[start..end].to_owned()).inner()
        } else {
            sys::mrb_sys_nil_value()
        };
        sys::mrb_gv_set(
            mrb,
            interp.borrow_mut().sym_intern("$&"),
            last_matched_string,
        );
        let data = if is_match.is_some() {
            if let Some(captures) = regexp.borrow().regex.captures(&string[pos..]) {
                let num_regexp_globals_to_set = {
                    let num_previously_set_globals = interp.borrow().num_set_regexp_capture_globals;
                    cmp::max(num_previously_set_globals, captures.len())
                };
                for group in 1..=num_regexp_globals_to_set {
                    let value = Value::from_mrb(&interp, captures.at(group));
                    sys::mrb_gv_set(
                        mrb,
                        interp.borrow_mut().sym_intern(&format!("${}", group)),
                        value.inner(),
                    );
                }
                interp.borrow_mut().num_set_regexp_capture_globals = captures.len();
            }

            let data = MatchData::new(string.as_str(), regexp.borrow().clone(), 0, string.len());
            unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, None))
        } else {
            sys::mrb_sys_nil_value()
        };
        sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$~"), data);
        if let Some(block) = args.block {
            if sys::mrb_sys_value_is_nil(data) {
                sys::mrb_sys_nil_value()
            } else {
                sys::mrb_yield(mrb, block.inner(), data)
            }
        } else {
            data
        }
    }

    // TODO: Implement support for extracting named captures and assigning to
    // local variables.
    // See: https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-3D-7E
    unsafe extern "C" fn equal_squiggle(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Match::extract(&interp),
            sys::mrb_sys_nil_value()
        );

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return TypeError::raise(&interp, "No implicit conversion into String"),
            _ => return sys::mrb_sys_nil_value(),
        };

        let pos = args.pos.unwrap_or_default();
        let num_captures = regexp
            .borrow()
            .regex
            .captures(string.as_str())
            .map(|captures| captures.len())
            .unwrap_or_default();
        let pos = if pos < 0 {
            num_captures
                .checked_sub(usize::try_from(-pos).expect("positive i64 must be usize"))
                .unwrap_or_default()
        } else {
            usize::try_from(pos).expect("positive i64 must be usize")
        };
        // onig will panic if pos is beyond the end of string
        if pos > string.len() {
            return Value::from_mrb(&interp, false).inner();
        }
        let is_match = regexp.borrow().regex.search_with_options(
            string.as_str(),
            pos,
            string.len(),
            SearchOptions::SEARCH_OPTION_NONE,
            None,
        );
        if let Some(pos) = is_match {
            let pos = unwrap_or_raise!(interp, i64::try_from(pos), sys::mrb_sys_nil_value());
            Value::from_mrb(&interp, pos).inner()
        } else {
            sys::mrb_sys_nil_value()
        }
    }

    unsafe extern "C" fn eql(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = eql::Args::extract(&interp);
        let value = Value::new(&interp, slf);
        match eql::method(&interp, args, &value) {
            Ok(result) => result.inner(),
            Err(eql::Error::Fatal) => RuntimeError::raise(&interp, "fatal Regexp#== error"),
        }
    }

    unsafe extern "C" fn case_compare(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        let result = case_compare::Args::extract(&interp)
            .and_then(|args| case_compare::method(&interp, args, &value));
        match result {
            Ok(result) => result.inner(),
            Err(case_compare::Error::BadType) => Value::from_mrb(&interp, false).inner(),
            Err(case_compare::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal Regexp#=== error")
            }
        }
    }

    unsafe extern "C" fn casefold(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match casefold::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(casefold::Error::Fatal) => {
                RuntimeError::raise(&interp, "fatal Regexp#casefold? error")
            }
        }
    }

    unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let value = Value::new(&interp, slf);
        match names::method(&interp, &value) {
            Ok(result) => result.inner(),
            Err(names::Error::Fatal) => RuntimeError::raise(&interp, "fatal Regexp#names error"),
        }
    }

    unsafe extern "C" fn named_captures(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );

        let borrow = regexp.borrow();
        let mut map = HashMap::default();
        for (name, pos) in borrow.regex.capture_names() {
            map.insert(
                name.to_owned(),
                pos.iter().map(|pos| i64::from(*pos)).collect::<Vec<_>>(),
            );
        }
        Value::from_mrb(&interp, map).inner()
    }

    unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let borrow = regexp.borrow();
        Value::from_mrb(
            &interp,
            i64::from(borrow.literal_options.flags().bits()) | borrow.encoding.flags(),
        )
        .inner()
    }

    unsafe extern "C" fn source(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let s = regexp.borrow().literal_pattern.to_string();
        Value::from_mrb(&interp, s).inner()
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let s = regexp.borrow().pattern.to_string();
        Value::from_mrb(&interp, s).inner()
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn inspect(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            sys::mrb_sys_nil_value()
        );
        let s = format!(
            "/{}/{}{}",
            regexp.borrow().literal_pattern.as_str().replace("/", r"\/"),
            regexp.borrow().literal_options.as_literal_string(),
            regexp.borrow().encoding.as_literal_string()
        );
        Value::from_mrb(&interp, s).inner()
    }
}
