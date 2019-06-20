#![allow(warnings)]
use onig::{Regex, RegexOptions, Region, SearchOptions, Syntax};
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::eval::MrbEval;
use crate::extn::core::error::{ArgumentError, RubyException, SyntaxError, TypeError};
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::types::Ruby;
use crate::value::{Value, ValueLike};
use crate::MrbError;

mod args;
mod syntax;

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
        .add_self_method("union", Regexp::union, sys::mrb_args_rest());
    regexp
        .borrow_mut()
        .add_method("match?", Regexp::is_match, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("match", Regexp::match_, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("=~", Regexp::equal_squiggle, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("==", Regexp::equal_equal, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("===", Regexp::equal_equal_equal, sys::mrb_args_req(1));
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
    regexp
        .borrow_mut()
        .add_method("inspect", Regexp::inspect, sys::mrb_args_none());
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
    let match_data = interp.borrow_mut().def_class::<MatchData>(
        "MatchData",
        None,
        Some(rust_data_free::<MatchData>),
    );
    match_data.borrow_mut().mrb_value_is_rust_backed(true);
    match_data
        .borrow_mut()
        .add_method("string", MatchData::string, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("regexp", MatchData::regexp, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("[]", MatchData::idx, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("begin", MatchData::begin, sys::mrb_args_req(1));
    match_data
        .borrow_mut()
        .add_method("end", MatchData::end, sys::mrb_args_req(1));
    match_data
        .borrow_mut()
        .add_method("length", MatchData::length, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("size", MatchData::length, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("captures", MatchData::captures, sys::mrb_args_none());
    match_data.borrow_mut().add_method(
        "named_captures",
        MatchData::named_captures,
        sys::mrb_args_none(),
    );
    match_data
        .borrow_mut()
        .add_method("pre_match", MatchData::pre_match, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("post_match", MatchData::post_match, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("to_a", MatchData::to_a, sys::mrb_args_none());
    match_data
        .borrow_mut()
        .add_method("to_s", MatchData::to_s, sys::mrb_args_none());
    match_data.borrow().define(&interp)?;
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

    fn as_literal_string(self) -> String {
        let mut buf = String::new();
        if self.multiline {
            buf.push('m');
        }
        if self.ignore_case {
            buf.push('i');
        }
        if self.extended {
            buf.push('x');
        }
        buf
    }

    fn as_onig_string(self) -> String {
        let mut buf = String::new();
        let mut pos = String::new();
        let mut neg = String::new();
        if self.multiline {
            pos.push('m');
        } else {
            neg.push('m');
        }
        if self.ignore_case {
            pos.push('i');
        } else {
            neg.push('i');
        }
        if self.extended {
            pos.push('x');
        } else {
            neg.push('x');
        }
        buf.push_str(pos.as_str());
        if !neg.is_empty() {
            buf.push('-');
            buf.push_str(neg.as_str());
        }
        buf
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
                ArgumentError::raise(&interp, "Invalid Regexp flags");
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
        let orig_opts = opts.clone();
        let mut chars = pattern.chars();
        let mut enabled = true;
        let mut pat_buf = String::new();
        let mut pointer = 0;
        match chars.next() {
            None => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string().as_str());
                pat_buf.push(')');
                return (pat_buf, opts);
            }
            Some(token) if token != '(' => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string().as_str());
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
                pat_buf.push_str(opts.as_onig_string().as_str());
                pat_buf.push(':');
                pat_buf.push_str(pattern);
                pat_buf.push(')');
                return (pat_buf, opts);
            }
            Some(token) if token != '?' => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.as_onig_string().as_str());
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
                    pat_buf.push_str(opts.as_onig_string().as_str());
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

    fn as_literal_string(self) -> String {
        match self {
            Encoding::Fixed => "",
            Encoding::No => "n",
            Encoding::None => "",
        }.to_owned()
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
                ArgumentError::raise(&interp, "Invalid Regexp encoding");
                return Err(MrbError::Exec("Invalid Regexp encoding".to_owned()));
            }
        } else if let Ok(encoding) =
            unsafe { String::try_from_mrb(&interp, Value::new(&interp, value)) }
        {
            if encoding.contains('u') && encoding.contains('n') {
                ArgumentError::raise(&interp, "Invalid Regexp encoding");
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
                    ArgumentError::raise(&interp, "Invalid Regexp encoding");
                    return Err(MrbError::Exec("Invalid Regexp encoding".to_owned()));
                }
            }
            if enc.len() > 1 {
                ArgumentError::raise(&interp, "Invalid Regexp encoding");
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

    pub fn regex(&self) -> Option<Regex> {
        Regex::with_options(&self.pattern, self.options.flags(), Syntax::ruby()).ok()
    }

    pub fn new(
        literal_pattern: String,
        pattern: String,
        literal_options: Options,
        options: Options,
        encoding: Encoding,
    ) -> Option<Self> {
        let regexp = Self {
            literal_pattern,
            pattern,
            literal_options,
            options,
            encoding,
        };
        if regexp.regex().is_none() {
            None
        } else {
            Some(regexp)
        }
    }

    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::RegexpNew::extract(&interp),
            interp.nil().inner()
        );
        let spec = class_spec_or_raise!(interp, Self);
        let regexp_class = unwrap_or_raise!(
            interp,
            spec.borrow()
                .rclass(&interp)
                .ok_or_else(|| MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        );
        let pattern_is_regexp =
            sys::mrb_obj_is_kind_of(interp.borrow().mrb, args.pattern.inner(), regexp_class) != 0;

        let pattern = if pattern_is_regexp {
            let regexp = unwrap_or_raise!(
                interp,
                Self::try_from_ruby(&interp, &Value::new(&interp, args.pattern.inner())),
                interp.nil().inner()
            );
            let borrow = regexp.borrow();
            borrow.pattern.clone()
        } else {
            unwrap_or_raise!(
                interp,
                args.pattern.try_into::<String>(),
                interp.nil().inner()
            )
        };
        let literal_options = args.options.unwrap_or_default();
        let literal_pattern = pattern;
        let (pattern, options) = Options::from_pattern(literal_pattern.as_str(), literal_options);
        if let Some(data) = Self::new(
            literal_pattern,
            pattern,
            literal_options,
            options,
            args.encoding.unwrap_or_default(),
        ) {
            unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, Some(slf)))
        } else {
            // Regexp is invalid.
            SyntaxError::raise(&interp, "malformed Regexp")
        }
    }

    unsafe extern "C" fn compile(
        mrb: *mut sys::mrb_state,
        mut _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::RegexpNew::extract(&interp),
            interp.nil().inner()
        );
        let spec = class_spec_or_raise!(interp, Self);
        let regexp_class = unwrap_or_raise!(
            interp,
            spec.borrow()
                .rclass(&interp)
                .ok_or_else(|| MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        );
        let pattern_is_regexp =
            sys::mrb_obj_is_kind_of(interp.borrow().mrb, args.pattern.inner(), regexp_class) != 0;

        let pattern = if pattern_is_regexp {
            let regexp = unwrap_or_raise!(
                interp,
                Self::try_from_ruby(&interp, &Value::new(&interp, args.pattern.inner())),
                interp.nil().inner()
            );
            let borrow = regexp.borrow();
            borrow.pattern.clone()
        } else {
            unwrap_or_raise!(
                interp,
                args.pattern.try_into::<String>(),
                interp.nil().inner()
            )
        };
        let literal_options = args.options.unwrap_or_default();
        let literal_pattern = pattern;
        let (pattern, options) = Options::from_pattern(literal_pattern.as_str(), literal_options);
        if let Some(data) = Self::new(
            literal_pattern,
            pattern,
            literal_options,
            options,
            args.encoding.unwrap_or_default(),
        ) {
            unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, None))
        } else {
            // Regexp is invalid.
            SyntaxError::raise(&interp, "malformed Regexp")
        }
    }

    unsafe extern "C" fn escape(
        mrb: *mut sys::mrb_state,
        mut _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Pattern::extract(&interp),
            interp.nil().inner()
        );
        let spec = class_spec_or_raise!(interp, Self);
        let regexp_class = unwrap_or_raise!(
            interp,
            spec.borrow()
                .value(&interp)
                .ok_or_else(|| MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        );

        let pattern = syntax::escape(args.pattern.as_str());
        unwrap_value_or_raise!(
            interp,
            regexp_class.funcall::<Value, _, _>("new", &[Value::from_mrb(&interp, pattern)])
        )
    }

    unsafe extern "C" fn union(
        mrb: *mut sys::mrb_state,
        mut _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let mut args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());
        let pattern = if args.rest.is_empty() {
            "(?!)".to_owned()
        } else {
            let patterns = if args.rest.len() == 1 {
                let arg = args.rest.remove(0);
                if arg.ruby_type() == Ruby::Array {
                    unwrap_or_raise!(interp, arg.try_into::<Vec<Value>>(), interp.nil().inner())
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
        let args = unwrap_or_raise!(interp, args::Match::extract(&interp), interp.nil().inner());

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return TypeError::raise(&interp, "No implicit conversion into String"),
            _ => return interp.nil().inner(),
        };

        let pos = args.pos.unwrap_or_default();
        let pos = if pos < 0 {
            let strlen = i64::try_from(string.len()).unwrap_or_default();
            let pos = strlen + pos;
            if pos < 0 {
                return interp.nil().inner();
            }
            usize::try_from(pos).expect("positive i64 must be usize")
        } else {
            usize::try_from(pos).expect("positive i64 must be usize")
        };
        // onig will panic if pos is beyond the end of string
        if pos > string.len() {
            return Value::from_mrb(&interp, false).inner();
        }
        let is_match = data.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                string.as_str(),
                pos,
                string.len(),
                SearchOptions::SEARCH_OPTION_NONE,
                None,
            )
        });
        Value::from_mrb(&interp, is_match.is_some()).inner()
    }

    unsafe extern "C" fn match_(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Match::extract(&interp), interp.nil().inner());

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return TypeError::raise(&interp, "No implicit conversion into String"),
            _ => {
                let global_last_match_captures = "$~";
                let global_match_captures_name = sys::mrb_intern(
                    interp.borrow().mrb,
                    global_last_match_captures.as_ptr() as *const i8,
                    global_last_match_captures.len(),
                );
                sys::mrb_gv_set(
                    interp.borrow().mrb,
                    global_match_captures_name,
                    interp.nil().inner(),
                );
                return interp.nil().inner();
            }
        };

        let pos = args.pos.unwrap_or_default();
        let pos = if pos < 0 {
            let strlen = i64::try_from(string.len()).unwrap_or_default();
            let pos = strlen + pos;
            if pos < 0 {
                return interp.nil().inner();
            }
            usize::try_from(pos).expect("positive i64 must be usize")
        } else {
            usize::try_from(pos).expect("positive i64 must be usize")
        };
        // onig will panic if pos is beyond the end of string
        if pos > string.len() {
            return interp.nil().inner();
        }
        let mut region = Region::new();
        let is_match = regexp.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                string.as_str(),
                pos,
                string.len(),
                SearchOptions::SEARCH_OPTION_NONE,
                Some(&mut region),
            )
        });
        let last_matched_string = if let Some((start, end)) = region.pos(0) {
            Value::from_mrb(&interp, string[start..end].to_owned()).inner()
        } else {
            interp.nil().inner()
        };
        let global_last_match_string = "$&";
        let global_match_string_name = sys::mrb_intern(
            interp.borrow().mrb,
            global_last_match_string.as_ptr() as *const i8,
            global_last_match_string.len(),
        );
        sys::mrb_gv_set(
            interp.borrow().mrb,
            global_match_string_name,
            last_matched_string,
        );
        let data = if is_match.is_some() {
            if let Some(captures) = regexp
                .borrow()
                .regex()
                .and_then(|regexp| regexp.captures(&string[pos..]))
            {
                for group in 1..=99 {
                    let value = Value::from_mrb(&interp, captures.at(group));
                    let global_capture_group = format!("${}", group);
                    let global_capture_group_name = sys::mrb_intern(
                        interp.borrow().mrb,
                        global_capture_group.as_ptr() as *const i8,
                        global_capture_group.len(),
                    );
                    sys::mrb_gv_set(
                        interp.borrow().mrb,
                        global_capture_group_name,
                        value.inner(),
                    );
                }
            }

            let data = MatchData {
                string,
                regexp: regexp.borrow().clone(),
                start_pos: pos,
            };
            unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, None))
        } else {
            interp.nil().inner()
        };
        let global_last_match_captures = "$~";
        let global_match_captures_name = sys::mrb_intern(
            interp.borrow().mrb,
            global_last_match_captures.as_ptr() as *const i8,
            global_last_match_captures.len(),
        );
        sys::mrb_gv_set(interp.borrow().mrb, global_match_captures_name, data);
        if let Some(block) = args.block {
            if sys::mrb_sys_value_is_nil(data) {
                interp.nil().inner()
            } else {
                sys::mrb_yield(interp.borrow().mrb, block.inner(), data)
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
        let args = unwrap_or_raise!(interp, args::Match::extract(&interp), interp.nil().inner());

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return TypeError::raise(&interp, "No implicit conversion into String"),
            _ => return interp.nil().inner(),
        };

        let pos = args.pos.unwrap_or_default();
        let num_captures = regexp
            .borrow()
            .regex()
            .and_then(|regexp| regexp.captures(string.as_str()))
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
            return interp.bool(false).inner();
        }
        let is_match = regexp.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                string.as_str(),
                pos,
                string.len(),
                SearchOptions::SEARCH_OPTION_NONE,
                None,
            )
        });
        if let Some(pos) = is_match {
            let pos = unwrap_or_raise!(interp, i64::try_from(pos), interp.nil().inner());
            interp.fixnum(pos).inner()
        } else {
            interp.nil().inner()
        }
    }

    unsafe extern "C" fn equal_equal(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let other = if let Ok(other) = Self::try_from_ruby(&interp, &args.rest[0]) {
            other
        } else {
            return interp.bool(false).inner();
        };
        let regborrow = regexp.borrow();
        let othborrow = other.borrow();
        interp.bool(regborrow.pattern == othborrow.pattern).inner()
    }

    unsafe extern "C" fn equal_equal_equal(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(
            interp,
            args::Match::extract(&interp),
            interp.bool(false).inner()
        );

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.bool(false).inner()
        );
        let string = match args.string {
            Ok(Some(ref string)) => string.to_owned(),
            Err(_) => return interp.bool(false).inner(),
            _ => {
                let global_last_match_captures = "$~";
                let global_match_captures_name = sys::mrb_intern(
                    interp.borrow().mrb,
                    global_last_match_captures.as_ptr() as *const i8,
                    global_last_match_captures.len(),
                );
                sys::mrb_gv_set(
                    interp.borrow().mrb,
                    global_match_captures_name,
                    interp.nil().inner(),
                );
                return interp.bool(false).inner();
            }
        };

        let pos = args.pos.unwrap_or_default();
        let pos = if pos < 0 {
            let strlen = i64::try_from(string.len()).unwrap_or_default();
            let pos = strlen + pos;
            if pos < 0 {
                return interp.nil().inner();
            }
            usize::try_from(pos).expect("positive i64 must be usize")
        } else {
            usize::try_from(pos).expect("positive i64 must be usize")
        };
        // onig will panic if pos is beyond the end of string
        if pos > string.len() {
            return interp.nil().inner();
        }
        let mut region = Region::new();
        let is_match = regexp.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                string.as_str(),
                pos,
                string.len(),
                SearchOptions::SEARCH_OPTION_NONE,
                Some(&mut region),
            )
        });
        let last_matched_string = if let Some((start, end)) = region.pos(0) {
            Value::from_mrb(&interp, string[start..end].to_owned()).inner()
        } else {
            interp.nil().inner()
        };
        let global_last_match_string = "$&";
        let global_match_string_name = sys::mrb_intern(
            interp.borrow().mrb,
            global_last_match_string.as_ptr() as *const i8,
            global_last_match_string.len(),
        );
        sys::mrb_gv_set(
            interp.borrow().mrb,
            global_match_string_name,
            last_matched_string,
        );
        let data = if is_match.is_some() {
            if let Some(captures) = regexp
                .borrow()
                .regex()
                .and_then(|regexp| regexp.captures(&string[pos..]))
            {
                for group in 1..=99 {
                    let value = Value::from_mrb(&interp, captures.at(group));
                    let global_capture_group = format!("${}", group);
                    let global_capture_group_name = sys::mrb_intern(
                        interp.borrow().mrb,
                        global_capture_group.as_ptr() as *const i8,
                        global_capture_group.len(),
                    );
                    sys::mrb_gv_set(
                        interp.borrow().mrb,
                        global_capture_group_name,
                        value.inner(),
                    );
                }
            }

            let data = MatchData {
                string,
                regexp: regexp.borrow().clone(),
                start_pos: pos,
            };
            unwrap_or_raise!(
                interp,
                data.try_into_ruby(&interp, None),
                interp.bool(false).inner()
            )
            .inner()
        } else {
            interp.nil().inner()
        };
        let global_last_match_captures = "$~";
        let global_match_captures_name = sys::mrb_intern(
            interp.borrow().mrb,
            global_last_match_captures.as_ptr() as *const i8,
            global_last_match_captures.len(),
        );
        sys::mrb_gv_set(interp.borrow().mrb, global_match_captures_name, data);
        interp.bool(!sys::mrb_sys_value_is_nil(data)).inner()
    }

    unsafe extern "C" fn names(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );

        let borrow = regexp.borrow();
        if let Some(regexp) = borrow.regex() {
            let names = regexp
                .capture_names()
                .map(|(name, _)| name.to_owned())
                .collect::<Vec<_>>();
            Value::from_mrb(&interp, names).inner()
        } else {
            interp.nil().inner()
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
            interp.nil().inner()
        );

        let borrow = regexp.borrow();
        if let Some(regexp) = borrow.regex() {
            let mut map = HashMap::default();
            for (name, pos) in regexp.capture_names() {
                map.insert(
                    name.to_owned(),
                    pos.iter().map(|pos| i64::from(*pos)).collect::<Vec<_>>(),
                );
            }
            Value::from_mrb(&interp, map).inner()
        } else {
            interp.nil().inner()
        }
    }

    unsafe extern "C" fn options(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
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
            interp.nil().inner()
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
            interp.nil().inner()
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
            interp.nil().inner()
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

#[derive(Debug)]
pub struct MatchData {
    string: String,
    regexp: Regexp,
    start_pos: usize,
}

impl RustBackedValue for MatchData {}

impl MatchData {
    fn string_to_capture(&self) -> &str {
        &self.string[self.start_pos..]
    }

    unsafe extern "C" fn string(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        Value::from_mrb(&interp, borrow.string.as_str()).inner()
    }

    unsafe extern "C" fn regexp(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        unwrap_value_or_raise!(interp, borrow.regexp.clone().try_into_ruby(&interp, None))
    }

    unsafe extern "C" fn idx(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let length = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()))
            .map(|captures| captures.len())
            .unwrap_or_default();
        let args = unwrap_or_raise!(
            interp,
            args::MatchIndex::extract(&interp, length),
            interp.nil().inner()
        );
        match args {
            args::MatchIndex::Index(index) => {
                let index = if index < 0 {
                    length
                        .checked_sub(usize::try_from(-index).expect("positive i64 must be usize"))
                        .unwrap_or_default()
                } else {
                    usize::try_from(index).expect("positive i64 must be usize")
                };
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
                match captures {
                    Some(captures) => Value::from_mrb(&interp, captures.at(index)).inner(),
                    None => interp.nil().inner(),
                }
            }
            args::MatchIndex::Name(name) => {
                let match_ = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| {
                        regexp
                            .capture_names()
                            .find(|capture| capture.0 == name)
                            .and_then(|capture| usize::try_from(capture.1[0]).ok())
                    })
                    .and_then(|index| {
                        borrow.regexp.regex().and_then(|regexp| {
                            regexp
                                .captures(borrow.string_to_capture())
                                .and_then(|captures| captures.at(index))
                        })
                    });
                Value::from_mrb(&interp, match_).inner()
            }
            args::MatchIndex::StartLen(start, len) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
                match captures {
                    Some(captures) => {
                        let start = if start < 0 {
                            captures.len().checked_sub(
                                usize::try_from(-start).expect("positive i64 must be usize"),
                            )
                        } else {
                            Some(usize::try_from(start).expect("positive i64 must be usize"))
                        };
                        match start {
                            Some(start) => {
                                let mut matches = vec![];
                                for index in start..(start + len) {
                                    matches.push(captures.at(index));
                                }
                                Value::from_mrb(&interp, matches).inner()
                            }
                            None => interp.nil().inner(),
                        }
                    }
                    None => interp.nil().inner(),
                }
            }
        }
    }

    unsafe extern "C" fn begin(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let length = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()))
            .map(|captures| captures.len());
        let args = unwrap_or_raise!(
            interp,
            args::MatchIndex::extract(&interp, length.unwrap_or_default()),
            interp.nil().inner()
        );
        match args {
            args::MatchIndex::Index(index) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
                match captures {
                    Some(captures) => {
                        let index = if index < 0 {
                            captures.len().checked_sub(
                                usize::try_from(-index).expect("positive i64 must be usize"),
                            )
                        } else {
                            Some(usize::try_from(index).expect("positive i64 must be usize"))
                        };
                        let index = index
                            .and_then(|index| captures.pos(index))
                            .map(|pos| borrow.string_to_capture()[0..pos.0].chars().count())
                            .and_then(|pos| i64::try_from(pos).ok());
                        Value::from_mrb(&interp, index).inner()
                    }
                    None => interp.nil().inner(),
                }
            }
            args::MatchIndex::Name(name) => {
                let pos = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| {
                        regexp
                            .capture_names()
                            .find(|capture| capture.0 == name)
                            .and_then(|capture| usize::try_from(capture.1[0]).ok())
                    })
                    .and_then(|index| {
                        borrow.regexp.regex().and_then(|regexp| {
                            regexp
                                .captures(borrow.string_to_capture())
                                .and_then(|captures| captures.pos(index))
                                .map(|pos| borrow.string_to_capture()[0..pos.0].chars().count())
                                .and_then(|pos| i64::try_from(pos).ok())
                        })
                    });
                Value::from_mrb(&interp, pos).inner()
            }
            args::MatchIndex::StartLen(_, _) => {
                ArgumentError::raise(&interp, "must pass index or symbol")
            }
        }
    }

    unsafe extern "C" fn end(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let length = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()))
            .map(|captures| captures.len());
        let args = unwrap_or_raise!(
            interp,
            args::MatchIndex::extract(&interp, length.unwrap_or_default()),
            interp.nil().inner()
        );
        match args {
            args::MatchIndex::Index(index) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
                match captures {
                    Some(captures) => {
                        let index = if index < 0 {
                            captures.len().checked_sub(
                                usize::try_from(-index).expect("positive i64 must be usize"),
                            )
                        } else {
                            Some(usize::try_from(index).expect("positive i64 must be usize"))
                        };
                        let index = index
                            .and_then(|index| captures.pos(index))
                            .map(|pos| borrow.string_to_capture()[0..pos.1].chars().count())
                            .and_then(|pos| i64::try_from(pos).ok());
                        Value::from_mrb(&interp, index).inner()
                    }
                    None => interp.nil().inner(),
                }
            }
            args::MatchIndex::Name(name) => {
                let pos = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| {
                        regexp
                            .capture_names()
                            .find(|capture| capture.0 == name)
                            .and_then(|capture| usize::try_from(capture.1[0]).ok())
                    })
                    .and_then(|index| {
                        borrow.regexp.regex().and_then(|regexp| {
                            regexp
                                .captures(borrow.string_to_capture())
                                .and_then(|captures| captures.pos(index))
                                .map(|pos| borrow.string_to_capture()[0..pos.1].chars().count())
                                .and_then(|pos| i64::try_from(pos).ok())
                        })
                    });
                Value::from_mrb(&interp, pos).inner()
            }
            args::MatchIndex::StartLen(_, _) => {
                ArgumentError::raise(&interp, "must pass index or symbol")
            }
        }
    }

    unsafe extern "C" fn length(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let Some(captures) = captures {
            unwrap_value_or_raise!(
                interp,
                i64::try_from(captures.len()).map(|len| interp.fixnum(len))
            )
        } else {
            interp.fixnum(0).inner()
        }
    }

    unsafe extern "C" fn captures(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let Some(captures) = captures {
            let mut vec = vec![];
            for (group, subcapture) in captures.iter().enumerate() {
                if group > 0 {
                    vec.push(subcapture.map(String::from));
                }
            }
            Value::from_mrb(&interp, vec).inner()
        } else {
            interp.nil().inner()
        }
    }

    unsafe extern "C" fn named_captures(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let (Some(captures), Some(regex)) = (captures, borrow.regexp.regex()) {
            let mut map = HashMap::default();
            for (name, index) in regex.capture_names() {
                if let Some(group) = captures.at(usize::try_from(index[0]).unwrap_or_default()) {
                    map.insert(name.to_owned(), group.to_owned());
                }
            }
            Value::from_mrb(&interp, map).inner()
        } else {
            interp.nil().inner()
        }
    }

    unsafe extern "C" fn pre_match(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let Some((start, _)) = captures.and_then(|captures| captures.pos(0)) {
            Value::from_mrb(&interp, borrow.string_to_capture()[..start].to_owned()).inner()
        } else {
            interp.nil().inner()
        }
    }

    unsafe extern "C" fn post_match(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let Some((_, end)) = captures.and_then(|captures| captures.pos(0)) {
            Value::from_mrb(&interp, borrow.string_to_capture()[end..].to_owned()).inner()
        } else {
            interp.nil().inner()
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_a(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let Some(captures) = captures {
            let mut vec = vec![];
            for subcapture in captures.iter() {
                vec.push(subcapture.map(String::from));
            }
            Value::from_mrb(&interp, vec).inner()
        } else {
            interp.nil().inner()
        }
    }

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        let captures = borrow
            .regexp
            .regex()
            .and_then(|regexp| regexp.captures(borrow.string_to_capture()));
        if let Some(captures) = captures {
            Value::from_mrb(&interp, captures.at(0)).inner()
        } else {
            Value::from_mrb(&interp, "").inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::FromMrb;
    use crate::eval::MrbEval;
    use crate::extn::core::regexp;
    use crate::interpreter::Interpreter;
    use crate::value::types::Ruby;
    use crate::value::{Value, ValueLike};
    use crate::MrbError;

    #[test]
    fn regexp_new_from_string() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar')").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
        let class = regexp
            .funcall::<Value, _, _>("class", &[])
            .expect("funcall");
        let name = class.funcall::<String, _, _>("name", &[]).expect("funcall");
        assert_eq!(&name, "Regexp");
    }

    #[test]
    fn regexp_new_from_pattern() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("/foo.*bar/").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
        let class = regexp
            .funcall::<Value, _, _>("class", &[])
            .expect("funcall");
        let name = class.funcall::<String, _, _>("name", &[]).expect("funcall");
        assert_eq!(&name, "Regexp");
        let regexp = interp.eval("/foo.*bar/i").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
    }

    #[test]
    fn regexp_new_from_string_with_options() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar', true)").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
        let regexp = interp.eval("Regexp.new('foo.*bar', false)").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
        let regexp = interp.eval("Regexp.new('foo.*bar', nil)").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
        let regexp = interp
            .eval("Regexp.new('foo.*bar', 1 | 2 | 4)")
            .expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
        let regexp = interp.eval("Regexp.new('foo.*bar', 'ixm')").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
    }

    #[test]
    fn regexp_new_from_string_with_encoding() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar', 'u')").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Data);
    }

    #[test]
    fn regexp_new_from_string_with_invalid_encoding() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar', 'un')").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: Invalid Regexp encoding (ArgumentError)\n(eval):1".to_owned()
            ))
        );
        let regexp = interp.eval("Regexp.new('foo.*bar', 16 | 32)").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: Invalid Regexp encoding (ArgumentError)\n(eval):1".to_owned()
            ))
        );
        let regexp = interp.eval("Regexp.new('foo.*bar', 0, 'x')").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: Invalid Regexp encoding (ArgumentError)\n(eval):1".to_owned()
            ))
        );
    }

    #[test]
    fn regexp_new_from_string_with_invalid_pattern() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new(2)").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: conversion error: failed to convert from ruby Fixnum to rust String (RuntimeError)\n(eval):1".to_owned()
            ))
        );
        let regexp = interp.eval("Regexp.new(nil)").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: conversion error: failed to convert from ruby NilClass to rust String (RuntimeError)\n(eval):1".to_owned()
            ))
        );
        let regexp = interp.eval("Regexp.new(2, 1)").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: conversion error: failed to convert from ruby Fixnum to rust String (RuntimeError)\n(eval):1".to_owned()
            ))
        );
    }

    #[test]
    fn regexp_new_from_string_with_invalid_options() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar', 1024)").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec(
                "(eval):1: Invalid Regexp flags (ArgumentError)\n(eval):1".to_owned()
            ))
        );
        let regexp = interp.eval("/foo.*bar/o").map(|_| ());
        assert_eq!(
            regexp,
            Err(MrbError::Exec("SyntaxError: syntax error".to_owned()))
        );
    }

    #[test]
    fn regexp_is_match() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        // Reuse regexp to ensure that Rc reference count is maintained
        // correctly so no segfaults.
        let regexp = interp.eval("/R.../").expect("eval");
        let result = regexp.funcall::<bool, _, _>("match?", &[Value::from_mrb(&interp, "Ruby")]);
        assert_eq!(result, Ok(true));
        let result = regexp.funcall::<bool, _, _>(
            "match?",
            &[
                Value::from_mrb(&interp, "Ruby"),
                Value::from_mrb(&interp, 1),
            ],
        );
        assert_eq!(result, Ok(false));
        let result = regexp.funcall::<bool, _, _>(
            "match?",
            &[
                Value::from_mrb(&interp, "Ruby"),
                // Pos beyond end of string
                Value::from_mrb(&interp, 5),
            ],
        );
        assert_eq!(result, Ok(false));
        let result = regexp.funcall::<bool, _, _>(
            "match?",
            &[
                Value::from_mrb(&interp, "Ruby"),
                // Pos = len of string
                Value::from_mrb(&interp, 4),
            ],
        );
        assert_eq!(result, Ok(false));
        let regexp = interp.eval("/P.../").expect("eval");
        let result = regexp.funcall::<bool, _, _>("match?", &[Value::from_mrb(&interp, "Ruby")]);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn regexp_matchdata_match() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        // Reuse regexp to ensure that Rc reference count is maintained
        // correctly so no segfaults.
        let regexp = interp.eval(r"/(.)(.)(\d+)(\d)/").expect("eval");
        let match_data = regexp
            .funcall::<Value, _, _>("match", &[Value::from_mrb(&interp, "THX1138.")])
            .expect("match");
        let result = match_data.funcall::<String, _, _>("[]", &[Value::from_mrb(&interp, 0)]);
        assert_eq!(result, Ok("HX1138".to_owned()));
        let result = match_data.funcall::<Vec<String>, _, _>(
            "[]",
            &[Value::from_mrb(&interp, 1), Value::from_mrb(&interp, 2)],
        );
        assert_eq!(result, Ok(vec!["H".to_owned(), "X".to_owned()]));
        let result =
            match_data.funcall::<Vec<String>, _, _>("[]", &[interp.eval("1..3").expect("range")]);
        assert_eq!(
            result,
            Ok(vec!["H".to_owned(), "X".to_owned(), "113".to_owned()])
        );
        let result = match_data.funcall::<Vec<String>, _, _>(
            "[]",
            &[Value::from_mrb(&interp, -3), Value::from_mrb(&interp, 2)],
        );
        assert_eq!(result, Ok(vec!["X".to_owned(), "113".to_owned()]));

        let regexp = interp.eval(r"/(?<foo>a+)b/").expect("eval");
        let match_data = regexp
            .funcall::<Value, _, _>("match", &[Value::from_mrb(&interp, "ccaaab")])
            .expect("match");
        let result = match_data.funcall::<String, _, _>("[]", &[Value::from_mrb(&interp, "foo")]);
        assert_eq!(result, Ok("aaa".to_owned()));
        let result =
            match_data.funcall::<String, _, _>("[]", &[interp.eval(":foo").expect("symbol")]);
        assert_eq!(result, Ok("aaa".to_owned()));
    }

    #[test]
    fn regexp_matchdata_begin() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let m = interp
            .eval(r#"m = /(.)(.)(\d+)(\d)/.match("THX1138.")"#)
            .expect("eval");
        let result = m.funcall::<i64, _, _>("begin", &[Value::from_mrb(&interp, 0)]);
        assert_eq!(result, Ok(1));
        let result = m.funcall::<i64, _, _>("begin", &[Value::from_mrb(&interp, 2)]);
        assert_eq!(result, Ok(2));
        let m = interp
            .eval(r#"m = /(?<foo>.)(.)(?<bar>.)/.match("hoge")"#)
            .expect("eval");
        let result = m.funcall::<i64, _, _>("begin", &[Value::from_mrb(&interp, "foo")]);
        assert_eq!(result, Ok(0));
        let result = m.funcall::<i64, _, _>(
            "begin",
            &[Value::from_mrb(&interp, "bar")
                .funcall::<Value, _, _>("to_sym", &[])
                .unwrap()],
        );
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn regexp_matchdata_end() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let m = interp
            .eval(r#"m = /(.)(.)(\d+)(\d)/.match("THX1138.")"#)
            .expect("eval");
        let result = m.funcall::<i64, _, _>("end", &[Value::from_mrb(&interp, 0)]);
        assert_eq!(result, Ok(7));
        let result = m.funcall::<i64, _, _>("end", &[Value::from_mrb(&interp, 2)]);
        assert_eq!(result, Ok(3));
        let m = interp
            .eval(r#"m = /(?<foo>.)(.)(?<bar>.)/.match("hoge")"#)
            .expect("eval");
        let result = m.funcall::<i64, _, _>("end", &[Value::from_mrb(&interp, "foo")]);
        assert_eq!(result, Ok(1));
        let result = m.funcall::<i64, _, _>(
            "end",
            &[Value::from_mrb(&interp, "bar")
                .funcall::<Value, _, _>("to_sym", &[])
                .unwrap()],
        );
        assert_eq!(result, Ok(3));
    }
}
