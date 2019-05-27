use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::mem;
use std::rc::Rc;

use crate::convert::TryFromMrb;
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
    regexp
        .borrow_mut()
        .add_method("initialize", initialize, sys::mrb_args_req_and_opt(1, 2));
    regexp
        .borrow_mut()
        .add_self_method("compile", compile, sys::mrb_args_rest());
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

#[derive(Debug, Clone)]
pub struct Regexp {
    source: String,
    options: Options,
    encoding: Encoding,
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
}

extern "C" fn initialize(mrb: *mut sys::mrb_state, mut slf: sys::mrb_value) -> sys::mrb_value {
    let interp = unsafe { interpreter_or_raise!(mrb) };
    let mrb = { interp.borrow().mrb };
    let spec = unsafe { class_spec_or_raise!(interp, Regexp) };
    let regexp_class = unsafe {
        unwrap_or_raise!(
            interp,
            spec.borrow()
                .rclass(&interp)
                .ok_or(MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        )
    };

    let (source, options, encoding) = unsafe {
        let source = mem::uninitialized::<sys::mrb_value>();
        let options = mem::uninitialized::<sys::mrb_value>();
        let has_options = mem::uninitialized::<sys::mrb_bool>();
        let encoding = mem::uninitialized::<sys::mrb_value>();
        let has_encoding = mem::uninitialized::<sys::mrb_bool>();
        let argspec = unwrap_or_raise!(
            interp,
            CString::new(format!(
                "{}{}{}{}{}{}",
                sys::specifiers::OBJECT,
                sys::specifiers::FOLLOWING_ARGS_OPTIONAL,
                sys::specifiers::OBJECT,
                sys::specifiers::PREVIOUS_OPTIONAL_ARG_GIVEN,
                sys::specifiers::OBJECT,
                sys::specifiers::PREVIOUS_OPTIONAL_ARG_GIVEN
            )),
            interp.nil().inner()
        );
        sys::mrb_get_args(
            mrb,
            argspec.as_ptr(),
            &source,
            &options,
            &has_options,
            &encoding,
            &has_encoding,
        );
        let opts = if has_options == 0 {
            Options::default()
        } else {
            unwrap_or_raise!(
                interp,
                Options::from_value(&interp, options),
                interp.nil().inner()
            )
        };
        // the C boolean as u8 comparisons are easier if we keep the comparison
        // inverted.
        #[allow(clippy::if_not_else)]
        let encoding = if has_encoding != 0 {
            Encoding::from_value(&interp, encoding, false)
        } else if has_options != 0 {
            Encoding::from_value(&interp, options, true)
        } else {
            Ok(Encoding::default())
        };
        let encoding = unwrap_or_raise!(interp, encoding, interp.nil().inner());

        let source = if sys::mrb_obj_is_kind_of(mrb, source, regexp_class) == 0 {
            String::try_from_mrb(&interp, Value::new(&interp, source))
                .map_err(MrbError::ConvertToRust)
        } else {
            // TODO: this doesn't work because we have not implemented the
            // `__regexp_source` accessor.
            Value::new(&interp, source).funcall::<String, _, _>("__regexp_source", &[])
        };
        let source = unwrap_or_raise!(interp, source, interp.nil().inner());
        (source, opts, encoding)
    };
    let data = Regexp {
        source,
        options,
        encoding,
    };
    let data = Rc::new(RefCell::new(data));

    unsafe {
        let ptr = mem::transmute::<Rc<RefCell<Regexp>>, *mut c_void>(data);
        let spec = class_spec_or_raise!(interp, Regexp);
        sys::mrb_sys_data_init(&mut slf, ptr, spec.borrow().data_type());
    };
    slf
}

extern "C" fn compile(mrb: *mut sys::mrb_state, mut _slf: sys::mrb_value) -> sys::mrb_value {
    let interp = unsafe { interpreter_or_raise!(mrb) };
    let mrb = { interp.borrow().mrb };
    let spec = unsafe { class_spec_or_raise!(interp, Regexp) };
    let regexp_class = unsafe {
        unwrap_or_raise!(
            interp,
            spec.borrow()
                .value(&interp)
                .ok_or(MrbError::NotDefined("Regexp".to_owned())),
            interp.nil().inner()
        )
    };

    let args = unsafe {
        let args = mem::uninitialized::<*const sys::mrb_value>();
        let count = mem::uninitialized::<usize>();
        let argspec = unwrap_or_raise!(
            interp,
            CString::new(sys::specifiers::REST),
            interp.nil().inner()
        );
        sys::mrb_get_args(mrb, argspec.as_ptr(), &args, &count);
        std::slice::from_raw_parts(args, count)
    };
    let args = args
        .iter()
        .map(|value| Value::new(&interp, *value))
        .collect::<Vec<_>>();
    unsafe { unwrap_value_or_raise!(interp, regexp_class.funcall::<Value, _, _>("new", args)) }
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
