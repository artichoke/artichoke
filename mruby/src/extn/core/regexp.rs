use onig::{Regex, RegexOptions, SearchOptions, Syntax};
use std::convert::TryFrom;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::extn::core::error::{ArgumentError, RubyException};
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::MrbError;

mod args;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
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
        .add_method("match?", Regexp::is_match, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("match", Regexp::match_, sys::mrb_args_req_and_opt(1, 1));
    regexp
        .borrow_mut()
        .add_method("=~", Regexp::equal_squiggle, sys::mrb_args_req(1));
    regexp
        .borrow_mut()
        .add_method("to_s", Regexp::to_s, sys::mrb_args_none());
    regexp
        .borrow_mut()
        .add_method("inspect", Regexp::to_s, sys::mrb_args_none());
    regexp.borrow().define(&interp)?;
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
    match_data.borrow().define(&interp)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Options {
    ignore_case: bool,
    extended: bool,
    multiline: bool,
}

impl Options {
    fn flags(self) -> RegexOptions {
        let mut bits = RegexOptions::REGEX_OPTION_NONE;
        if self.ignore_case {
            bits |= RegexOptions::REGEX_OPTION_IGNORECASE;
        }
        if self.extended {
            bits |= RegexOptions::REGEX_OPTION_EXTEND;
        }
        if self.multiline {
            bits |= RegexOptions::REGEX_OPTION_MULTILINE;
        }
        bits
    }

    fn as_string_opts(self) -> String {
        let mut buf = String::new();
        if self.ignore_case {
            buf.push('i');
        }
        if self.multiline {
            buf.push('m');
        }
        if self.extended {
            buf.push('e');
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

    fn ignore_case() -> Self {
        let mut opts = Self::default();
        opts.ignore_case = true;
        opts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Encoding {
    Fixed,
    None,
}

impl Encoding {
    fn flags(self) -> i64 {
        match self {
            Encoding::Fixed => Regexp::FIXEDENCODING,
            Encoding::None => Regexp::NOENCODING,
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
                Ok(Encoding::None)
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
                if flag == 'u' {
                    enc.push(Encoding::Fixed);
                } else if flag == 'n' {
                    enc.push(Encoding::None);
                } else if from_options && (flag == 'i' || flag == 'm' || flag == 'x') {
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
    pattern: String,
    options: Options,
    encoding: Encoding,
}

impl RustBackedValue for Regexp {
    fn new_obj_args(&self, interp: &Mrb) -> Vec<sys::mrb_value> {
        vec![
            Value::from_mrb(interp, self.pattern.as_str()).inner(),
            Value::from_mrb(interp, self.options.flags().bits()).inner(),
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
        Regex::with_options(&self.pattern, self.options.flags(), Syntax::default()).ok()
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
            // TODO: this doesn't work because we have not implemented the
            // `__regexp_source` accessor.
            args.pattern.funcall::<String, _, _>("__regexp_source", &[])
        } else {
            args.pattern.try_into()
        };
        let options = args.options.unwrap_or_default();
        let pattern = unwrap_or_raise!(interp, pattern, interp.nil().inner());
        let data = Self {
            pattern,
            options,
            encoding: args.encoding.unwrap_or_default(),
        };
        // Make sure we the regexp is valid.
        if data.regex().is_none() {
            return ArgumentError::raise(&interp, "Cannot parse Regexp");
        }
        unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, Some(slf)))
    }

    unsafe extern "C" fn compile(
        mrb: *mut sys::mrb_state,
        mut _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Rest::extract(&interp), interp.nil().inner());
        let spec = class_spec_or_raise!(interp, Self);
        let regexp_class = unwrap_or_raise!(
            interp,
            spec.borrow()
                .value(&interp)
                .ok_or_else(|| MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        );

        unwrap_value_or_raise!(
            interp,
            regexp_class.funcall::<Value, _, _>("new", args.rest)
        )
    }

    unsafe extern "C" fn is_match(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, args::Match::extract(&interp), interp.nil().inner());

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );

        // onig will panic if pos is beyond the end of string
        if args.pos.unwrap_or_default() > args.string.len() {
            return Value::from_mrb(&interp, false).inner();
        }
        let is_match = data.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                &args.string,
                args.pos.unwrap_or_default(),
                args.string.len(),
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

        let is_match = regexp.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                &args.string,
                args.pos.unwrap_or_default(),
                args.string.len(),
                SearchOptions::SEARCH_OPTION_NONE,
                None,
            )
        });
        let data = if is_match.is_some() {
            MatchData {
                string: args.string,
                regexp: regexp.borrow().clone(),
                start_pos: args.pos.unwrap_or_default(),
            }
        } else {
            return interp.nil().inner();
        };
        unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, None))
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

        let is_match = regexp.borrow().regex().and_then(|regexp| {
            regexp.search_with_options(
                &args.string,
                args.pos.unwrap_or_default(),
                args.string.len(),
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

    #[allow(clippy::wrong_self_convention)]
    unsafe extern "C" fn to_s(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let regexp = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let s = format!(
            "/{}/{}",
            regexp.borrow().pattern,
            regexp.borrow().options.as_string_opts()
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
        let args = unwrap_or_raise!(
            interp,
            args::MatchIndex::extract(&interp),
            interp.nil().inner()
        );
        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        match args {
            args::MatchIndex::Index(index) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string.as_str()));
                match captures {
                    Some(captures) => {
                        let index = if index < 0 {
                            captures.len().checked_sub(
                                usize::try_from(-index).expect("positive i64 must be usize"),
                            )
                        } else {
                            Some(usize::try_from(index).expect("positive i64 must be usize"))
                        };
                        Value::from_mrb(&interp, index.and_then(|index| captures.at(index))).inner()
                    }
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
                                .captures(borrow.string.as_str())
                                .and_then(|captures| captures.at(index))
                        })
                    });
                Value::from_mrb(&interp, match_).inner()
            }
            args::MatchIndex::StartLen(start, len) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string.as_str()));
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
                                    if let Some(matched) = captures.at(index) {
                                        matches.push(matched);
                                    }
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

        let args = unwrap_or_raise!(
            interp,
            args::MatchIndex::extract(&interp),
            interp.nil().inner()
        );
        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        match args {
            args::MatchIndex::Index(index) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string.as_str()));
                match captures {
                    Some(captures) => {
                        let index = if index < 0 {
                            captures.len().checked_sub(
                                usize::try_from(-index).expect("positive i64 must be usize"),
                            )
                        } else {
                            Some(usize::try_from(index).expect("positive i64 must be usize"))
                        };
                        Value::from_mrb(
                            &interp,
                            index
                                .and_then(|index| captures.pos(index))
                                .map(|pos| pos.0)
                                .and_then(|pos| i64::try_from(pos).ok()),
                        )
                        .inner()
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
                                .captures(borrow.string.as_str())
                                .and_then(|captures| captures.pos(index))
                                .map(|pos| pos.0)
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

        let args = unwrap_or_raise!(
            interp,
            args::MatchIndex::extract(&interp),
            interp.nil().inner()
        );
        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let borrow = data.borrow();
        match args {
            args::MatchIndex::Index(index) => {
                let captures = borrow
                    .regexp
                    .regex()
                    .and_then(|regexp| regexp.captures(borrow.string.as_str()));
                match captures {
                    Some(captures) => {
                        let index = if index < 0 {
                            captures.len().checked_sub(
                                usize::try_from(-index).expect("positive i64 must be usize"),
                            )
                        } else {
                            Some(usize::try_from(index).expect("positive i64 must be usize"))
                        };
                        Value::from_mrb(
                            &interp,
                            index
                                .and_then(|index| captures.pos(index))
                                .map(|pos| pos.1)
                                .and_then(|pos| i64::try_from(pos).ok()),
                        )
                        .inner()
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
                                .captures(borrow.string.as_str())
                                .and_then(|captures| captures.pos(index))
                                .map(|pos| pos.1)
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
