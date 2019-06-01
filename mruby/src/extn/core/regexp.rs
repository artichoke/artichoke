use onig::{Regex, RegexOptions, SearchOptions, Syntax};
use std::cell::RefCell;
use std::ffi::c_void;
use std::io::Write;
use std::mem;
use std::rc::Rc;

use crate::convert::{FromMrb, TryFromMrb};
use crate::def::{rust_data_free, ClassLike, Define};
use crate::extn::core::error::ArgumentError;
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::MrbError;

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
    regexp.borrow().define(&interp)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
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

#[derive(Debug, Clone)]
enum Encoding {
    Fixed,
    None,
}

impl Encoding {
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

#[derive(Debug)]
pub struct Regexp {
    pattern: String,
    options: Options,
    encoding: Encoding,
    regex: Regex,
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

    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        mut slf: sys::mrb_value,
    ) -> sys::mrb_value {
        struct Args {
            pattern: Value,
            options: Option<Options>,
            encoding: Option<Encoding>,
        }

        impl Args {
            unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
                let pattern = mem::uninitialized::<sys::mrb_value>();
                let options = mem::uninitialized::<sys::mrb_value>();
                let has_options = mem::uninitialized::<sys::mrb_bool>();
                let encoding = mem::uninitialized::<sys::mrb_value>();
                let has_encoding = mem::uninitialized::<sys::mrb_bool>();
                let mut argspec = vec![];
                argspec
                    .write_all(
                        format!(
                            "{}{}{}{}{}{}\0",
                            sys::specifiers::OBJECT,
                            sys::specifiers::FOLLOWING_ARGS_OPTIONAL,
                            sys::specifiers::OBJECT,
                            sys::specifiers::PREVIOUS_OPTIONAL_ARG_GIVEN,
                            sys::specifiers::OBJECT,
                            sys::specifiers::PREVIOUS_OPTIONAL_ARG_GIVEN
                        )
                        .as_bytes(),
                    )
                    .map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(
                    interp.borrow().mrb,
                    argspec.as_ptr() as *const i8,
                    &pattern,
                    &options,
                    &has_options,
                    &encoding,
                    &has_encoding,
                );
                let pattern = Value::new(&interp, pattern);
                // the C boolean as u8 comparisons are easier if we keep the
                // comparison inverted.
                #[allow(clippy::if_not_else)]
                let (options, encoding) = if has_encoding != 0 {
                    let encoding = Some(Encoding::from_value(&interp, encoding, false)?);
                    let options = if has_options == 0 {
                        None
                    } else {
                        Some(Options::from_value(&interp, options)?)
                    };
                    (options, encoding)
                } else if has_options != 0 {
                    (
                        Some(Options::from_value(&interp, options)?),
                        Some(Encoding::from_value(&interp, options, true)?),
                    )
                } else {
                    (None, None)
                };
                Ok(Self {
                    pattern,
                    options,
                    encoding,
                })
            }
        }

        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, Args::extract(&interp), interp.nil().inner());
        let spec = class_spec_or_raise!(interp, Self);
        let regexp_class = unwrap_or_raise!(
            interp,
            spec.borrow()
                .rclass(&interp)
                .ok_or(MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        );
        let pattern_is_regexp =
            sys::mrb_obj_is_kind_of(interp.borrow().mrb, args.pattern.inner(), regexp_class) != 0;

        let pattern = if pattern_is_regexp {
            // TODO: this doesn't work because we have not implemented the
            // `__regexp_source` accessor.
            args.pattern.funcall::<String, _, _>("__regexp_source", &[])
        } else {
            args.pattern.funcall::<String, _, _>("itself", &[])
        };
        let options = args.options.unwrap_or_default();
        let pattern = unwrap_or_raise!(interp, pattern, interp.nil().inner());
        let regex = unwrap_or_raise!(
            interp,
            Regex::with_options(&pattern, options.flags(), Syntax::default()),
            interp.nil().inner()
        );
        let data = Self {
            pattern,
            options,
            encoding: args.encoding.unwrap_or_default(),
            regex,
        };
        let data = Rc::new(RefCell::new(data));

        let ptr = mem::transmute::<Rc<RefCell<Self>>, *mut c_void>(data);
        let spec = class_spec_or_raise!(interp, Self);
        sys::mrb_sys_data_init(&mut slf, ptr, spec.borrow().data_type());
        slf
    }

    unsafe extern "C" fn compile(
        mrb: *mut sys::mrb_state,
        mut _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        struct Args {
            rest: Vec<Value>,
        }

        impl Args {
            unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
                let args = mem::uninitialized::<*const sys::mrb_value>();
                let count = mem::uninitialized::<usize>();
                let mut argspec = vec![];
                argspec
                    .write_all(sys::specifiers::REST.as_bytes())
                    .map_err(|_| MrbError::ArgSpec)?;
                argspec.write_all(b"\0").map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(
                    interp.borrow().mrb,
                    argspec.as_ptr() as *const i8,
                    &args,
                    &count,
                );
                let args = std::slice::from_raw_parts(args, count);
                let args = args
                    .iter()
                    .map(|value| Value::new(&interp, *value))
                    .collect::<Vec<_>>();
                Ok(Self { rest: args })
            }
        }

        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, Args::extract(&interp), interp.nil().inner());
        let spec = class_spec_or_raise!(interp, Self);
        let regexp_class = unwrap_or_raise!(
            interp,
            spec.borrow()
                .value(&interp)
                .ok_or(MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        );

        unwrap_value_or_raise!(
            interp,
            regexp_class.funcall::<Value, _, _>("new", args.rest)
        )
    }

    unsafe extern "C" fn is_match(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        struct Args {
            string: String,
            pos: Option<usize>,
        }

        impl Args {
            unsafe fn extract(interp: &Mrb) -> Result<Self, MrbError> {
                let string = mem::uninitialized::<sys::mrb_value>();
                let pos = mem::uninitialized::<sys::mrb_value>();
                let has_pos = mem::uninitialized::<sys::mrb_bool>();
                let mut argspec = vec![];
                argspec
                    .write_all(
                        format!(
                            "{}{}{}{}\0",
                            sys::specifiers::OBJECT,
                            sys::specifiers::FOLLOWING_ARGS_OPTIONAL,
                            sys::specifiers::OBJECT,
                            sys::specifiers::PREVIOUS_OPTIONAL_ARG_GIVEN
                        )
                        .as_bytes(),
                    )
                    .map_err(|_| MrbError::ArgSpec)?;
                sys::mrb_get_args(
                    interp.borrow().mrb,
                    argspec.as_ptr() as *const i8,
                    &string,
                    &pos,
                    &has_pos,
                );
                let string = String::try_from_mrb(&interp, Value::new(&interp, string))
                    .map_err(MrbError::ConvertToRust)?;
                let pos = if has_pos == 0 {
                    None
                } else {
                    let pos = usize::try_from_mrb(&interp, Value::new(&interp, pos))
                        .map_err(MrbError::ConvertToRust)?;
                    Some(pos)
                };
                Ok(Self { string, pos })
            }
        }
        let interp = interpreter_or_raise!(mrb);
        let args = unwrap_or_raise!(interp, Args::extract(&interp), interp.nil().inner());

        let ptr = {
            let spec = class_spec_or_raise!(interp, Self);
            let borrow = spec.borrow();
            sys::mrb_data_get_ptr(mrb, slf, borrow.data_type())
        };
        let data = mem::transmute::<*mut c_void, Rc<RefCell<Self>>>(ptr);
        let regex = Rc::clone(&data);
        mem::forget(data);

        // onig will panic if pos is beyond the end of string
        if args.pos.unwrap_or_default() > args.string.len() {
            return Value::from_mrb(&interp, false).inner();
        }
        let is_match = regex.borrow().regex.search_with_options(
            &args.string,
            args.pos.unwrap_or_default(),
            args.string.len(),
            SearchOptions::SEARCH_OPTION_NONE,
            None,
        );
        Value::from_mrb(&interp, is_match.is_some()).inner()
    }
}

pub struct MatchData;

#[cfg(test)]
mod tests {
    use crate::eval::MrbEval;
    use crate::extn::core::regexp;
    use crate::interpreter::Interpreter;
    use crate::sys;
    use crate::value::types::Ruby;
    use crate::value::{Value, ValueLike};
    use crate::MrbError;

    #[test]
    fn regexp_new_from_string() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar')").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
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
        assert_eq!(regexp.ruby_type(), Ruby::Object);
        let class = regexp
            .funcall::<Value, _, _>("class", &[])
            .expect("funcall");
        let name = class.funcall::<String, _, _>("name", &[]).expect("funcall");
        assert_eq!(&name, "Regexp");
        let regexp = interp.eval("/foo.*bar/i").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
    }

    #[test]
    fn regexp_new_from_string_with_options() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar', true)").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
        let regexp = interp.eval("Regexp.new('foo.*bar', false)").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
        let regexp = interp.eval("Regexp.new('foo.*bar', nil)").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
        let regexp = interp
            .eval("Regexp.new('foo.*bar', 1 | 2 | 4)")
            .expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
        let regexp = interp.eval("Regexp.new('foo.*bar', 'ixm')").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
    }

    #[test]
    fn regexp_new_from_string_with_encoding() {
        let interp = Interpreter::create().expect("mrb init");
        regexp::init(&interp).expect("regexp init");
        let regexp = interp.eval("Regexp.new('foo.*bar', 'u')").expect("eval");
        assert_eq!(regexp.ruby_type(), Ruby::Object);
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
            Err(MrbError::UnreachableValue(sys::mrb_vtype::MRB_TT_UNDEF))
        );
    }
}
