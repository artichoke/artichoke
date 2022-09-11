//! Implicit conversion routines based on `convert_type_with_id` in MRI.
//!
//! See: <https://github.com/ruby/ruby/blob/v3_1_2/object.c#L2908-L3018>.

use std::ffi::CStr;

use artichoke_core::debug::Debug as _;
use artichoke_core::value::Value as _;
use once_cell::sync::OnceCell;
use qed::const_cstr_from_str as cstr;
use spinoso_exception::TypeError;

use crate::types::Ruby;
use crate::value::Value;
use crate::{Artichoke, Error};

/// Strategy to use for handling errors in [`convert_type`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertOnError {
    /// Turn conversion errors into `TypeError`s.
    Raise,
    /// Turn conversion errors into a successful `nil` value.
    ReturnNil,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct ConvMethod {
    method: &'static str,
    cstr: &'static CStr,
    id: u32,
    is_implicit_conversion: bool,
}

fn conv_method_table(interp: &mut Artichoke) -> &'static [ConvMethod; 12] {
    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L2908-L2928
    #[rustfmt::skip]
    const METHODS: [(&str, &CStr, bool); 12] = [
        ("to_int",  cstr!("to_int\0"),   true),
        ("to_ary",  cstr!("to_ary\0"),   true),
        ("to_str",  cstr!("to_str\0"),   true),
        ("to_sym",  cstr!("to_sym\0"),   true),
        ("to_hash", cstr!("to_hash\0"),  true),
        ("to_proc", cstr!("to_proc\0"),  true),
        ("to_io",   cstr!("to_io\0"),    true),
        ("to_a",    cstr!("to_a\0"),    false),
        ("to_s",    cstr!("to_s\0"),    false),
        ("to_i",    cstr!("to_i\0"),    false),
        ("to_f",    cstr!("to_f\0"),    false),
        ("to_r",    cstr!("to_r\0"),    false),
    ];

    static CONV_METHOD_TABLE: OnceCell<[ConvMethod; 12]> = OnceCell::new();

    CONV_METHOD_TABLE.get_or_init(|| {
        METHODS.map(|(method, method_cstr, is_implicit_conversion)| {
            let bytes = method_cstr.to_bytes_with_nul();
            let sym = interp.intern_bytes_with_trailing_nul(bytes).unwrap();
            ConvMethod {
                method,
                cstr: method_cstr,
                id: sym,
                is_implicit_conversion,
            }
        })
    })
}

/// Attempt a fallible conversion of a Ruby value to a given type tag.
///
/// This function can convert Ruby values at the granularity of a [`Ruby`] type
/// tag. Conversion works as follows:
///
/// - If the given value has the same type tag as the given `convert_to`, return
///   the given value.
/// - Assert that the given conversion method is a valid conversion type.
/// - Call the conversion method on the given value. If this method raises,
///   return the error.
/// - If the converted value does not match the given type tag, raise a
///   [`TypeError`].
/// - The converted value matches the target type, return it.
///
/// # Conversion types
///
/// The method to be called to perform the implicit conversion must be one of a
/// permitted set. Valid method calls are:
///
/// - `to_int`
/// - `to_ary`
/// - `to_str`
/// - `to_sym`
/// - `to_hash`
/// - `to_proc`
/// - `to_io`
/// - `to_a`
/// - `to_s`
/// - `to_i`
/// - `to_f`
/// - `to_r`
///
/// # MRI Compatibility
///
/// This function is modeled after the [`rb_convert_type`] C API in MRI Ruby.
///
/// [`rb_convert_type`]: https://github.com/ruby/ruby/blob/v3_1_2/object.c#L2993-L3004
///
/// # Panics
///
/// If the given method is not a valid conversion method, this function will
/// panic.
///
/// # Errors
///
/// - If the call to the conversion method returns an error, that error is
///   returned.
/// - If the call to the conversion method returns a value that does not match
///   the target type tag, a [`TypeError`] is returned.
pub fn convert_type(
    interp: &mut Artichoke,
    value: Value,
    convert_to: Ruby,
    type_name: &str,
    method: &str,
    raise: ConvertOnError,
) -> Result<Value, Error> {
    if value.ruby_type() == convert_to {
        return Ok(value);
    }
    let converted = {
        let conversion = conv_method_table(interp)
            .iter()
            .find(|conversion| conversion.method == method)
            .unwrap_or_else(|| panic!("{method} is not a valid conversion method"));

        convert_type_inner(interp, value, type_name, conversion, raise)?
    };

    if converted.ruby_type() != convert_to {
        return Err(conversion_mismatch(interp, value, type_name, method, converted).into());
    }
    Ok(converted)
}

/// Attempt a fallible conversion of a Ruby value to a given type tag or `nil`.
///
/// This function can convert Ruby values at the granularity of a [`Ruby`] type
/// tag. Conversion works as follows:
///
/// - If the given value has the same type tag as the given `convert_to`, return
///   the given value.
/// - Assert that the given conversion method is a valid conversion type.
/// - Call the conversion method on the given value. If this method raises,
///   return the error.
/// - If the converted value is `nil`, return `nil`.
/// - If the converted value does not match the given type tag, raise a
///   [`TypeError`].
/// - The converted value matches the target type, return it.
///
/// # Conversion types
///
/// The method to be called to perform the implicit conversion must be one of a
/// permitted set. Valid method calls are:
///
/// - `to_int`
/// - `to_ary`
/// - `to_str`
/// - `to_sym`
/// - `to_hash`
/// - `to_proc`
/// - `to_io`
/// - `to_a`
/// - `to_s`
/// - `to_i`
/// - `to_f`
/// - `to_r`
///
/// # MRI Compatibility
///
/// This function is modeled after the [`rb_check_convert_type_with_id`] C API
/// in MRI Ruby.
///
/// [`rb_check_convert_type_with_id`]: https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3035-L3049
///
/// # Panics
///
/// If the given method is not a valid conversion method, this function will
/// panic.
///
/// # Errors
///
/// - If the call to the conversion method returns an error, that error is
///   returned.
/// - If the call to the conversion method returns a value that does is non-`nil`
///   and not match the target type tag, a [`TypeError`] is returned.
pub fn check_convert_type(
    interp: &mut Artichoke,
    value: Value,
    convert_to: Ruby,
    type_name: &str,
    method: &str,
) -> Result<Value, Error> {
    // always convert T_DATA
    if value.ruby_type() == convert_to && convert_to != Ruby::Data {
        return Ok(value);
    }
    let converted = {
        let conversion = conv_method_table(interp)
            .iter()
            .find(|conversion| conversion.method == method)
            .unwrap_or_else(|| panic!("{method} is not a valid conversion method"));

        convert_type_inner(interp, value, type_name, conversion, ConvertOnError::ReturnNil)?
    };

    match converted.ruby_type() {
        Ruby::Nil => Ok(Value::nil()),
        tt if tt == convert_to => Ok(converted),
        _ => Err(conversion_mismatch(interp, value, type_name, method, converted).into()),
    }
}

// https://github.com/ruby/ruby/blob/v3_1_2/object.c#L2948-L2971
fn convert_type_inner(
    interp: &mut Artichoke,
    value: Value,
    type_name: &str,
    conversion: &'static ConvMethod,
    raise: ConvertOnError,
) -> Result<Value, Error> {
    if value.respond_to(interp, conversion.method)? {
        return value.funcall(interp, conversion.method, &[], None);
    }
    let mut message = match raise {
        ConvertOnError::ReturnNil => return Ok(Value::nil()),
        ConvertOnError::Raise if conversion.is_implicit_conversion => String::from("no implicit conversion of "),
        ConvertOnError::Raise => String::from("can't convert "),
    };
    match value.try_convert_into::<Option<bool>>(interp) {
        Ok(None) => message.push_str("nil"),
        Ok(Some(true)) => message.push_str("true"),
        Ok(Some(false)) => message.push_str("false"),
        Err(_) => message.push_str(interp.class_name_for_value(value)),
    }
    message.push_str(" into ");
    message.push_str(type_name);
    Err(TypeError::from(message).into())
}

// https://github.com/ruby/ruby/blob/v3_1_2/object.c#L2982-L2991
fn conversion_mismatch(
    interp: &mut Artichoke,
    value: Value,
    type_name: &str,
    method: &str,
    result: Value,
) -> TypeError {
    let cname = interp.inspect_type_name_for_value(value);

    let mut message = String::from("can't convert ");
    message.push_str(cname);
    message.push_str(" to ");
    message.push_str(type_name);
    message.push_str(" (");
    message.push_str(cname);
    message.push('#');
    message.push_str(method);
    message.push_str(" gives ");
    message.push_str(interp.class_name_for_value(result));
    message.push(')');

    TypeError::from(message)
}

#[inline]
fn try_to_int(interp: &mut Artichoke, val: Value, method: &str, raise: ConvertOnError) -> Result<Value, Error> {
    let conversion = conv_method_table(interp)
        .iter()
        .find(|conversion| conversion.method == method)
        .unwrap_or_else(|| panic!("{method} is not a valid conversion method"));

    convert_type_inner(interp, val, "Integer", conversion, raise)
}

/// Fallible conversion of the given value to a Ruby `Integer` via `#to_int`.
///
/// If the given value is an integer, it is returned. If the give value responds
/// to a `#to_int` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// a non-`nil` Ruby `Integer`.
///
/// # Errors
///
/// - If the call to `#to_int` returns an error, that error is returned.
/// - If the call to `#to_int` returns anything other than a `Integer`, a
///   [`TypeError`] is returned.
#[inline]
pub fn to_int(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    // Fast path (no additional funcalls) for values that are already integers.
    if let Ruby::Fixnum = value.ruby_type() {
        return Ok(value);
    }
    convert_type(interp, value, Ruby::Fixnum, "Integer", "to_int", ConvertOnError::Raise)
}

/// Fallible conversion of the given value to a Ruby `Integer` or `nil` via
/// `#to_int`.
///
/// If the given value is an integer, it is returned. If the give value responds
/// to a `#to_int` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// either a Ruby `Integer` or `nil`.
///
/// # Errors
///
/// - If the call to `#to_int` returns an error, that error is returned.
/// - If the call to `#to_int` returns anything other than an `Integer` or `nil`,
///   a [`TypeError`] is returned.
#[inline]
pub fn check_to_int(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    // Fast path (no additional funcalls) for values that are already integers.
    if let Ruby::Fixnum = value.ruby_type() {
        return Ok(value);
    }
    let value = try_to_int(interp, value, "to_int", ConvertOnError::ReturnNil)?;
    if let Ruby::Fixnum = value.ruby_type() {
        Ok(value)
    } else {
        Ok(Value::nil())
    }
}

/// Fallible coercion of the given value to a Ruby `Integer` via `#to_i`.
///
/// If the given value is an integer, it is returned. If the give value responds
/// to a `#to_i` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// a non-`nil` Ruby `Integer`.
///
/// # Errors
///
/// - If the call to `#to_i` returns an error, that error is returned.
/// - If the call to `#to_i` returns anything other than a `Integer`, a
///   [`TypeError`] is returned.
#[inline]
pub fn to_i(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    if let Ruby::Fixnum = value.ruby_type() {
        return Ok(value);
    }
    convert_type(interp, value, Ruby::Fixnum, "Integer", "to_i", ConvertOnError::Raise)
}

// NOTE: A `check_to_i` variant is only used in `Kernel#Integer`.
//
// This API is not necessary in Artichoke since exceptions are passed by value
// instead of via unwinding.
//
// See: https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3149
/*
#[inline(always)]
pub(crate) fn check_to_i(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    // Fast path (no additional funcalls) for values that are already integers.
    if let Ruby::Fixnum = value.ruby_type() {
        return Ok(value);
    }
    let val = try_to_int(interp, val, "to_i", ConvertOnError::ReturnNil)?;
    if let Ruby::Fixnum = val.ruby_type() {
        Ok(val)
    } else {
        Ok(Value::nil())
    }
}
*/

/// Fallible conversion of the given value to a Ruby `String` via `#to_str`.
///
/// If the given value is a string, it is returned. If the give value responds
/// to a `#to_str` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// a non-`nil` Ruby `String`.
///
/// # Errors
///
/// - If the call to `#to_str` returns an error, that error is returned.
/// - If the call to `#to_str` returns anything other than a `String`, a
///   [`TypeError`] is returned.
pub fn to_str(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    convert_type(interp, value, Ruby::String, "String", "to_str", ConvertOnError::Raise)
}

/// Fallible conversion of the given value to a Ruby `String` or `nil` via
/// `#to_str`.
///
/// If the given value is a string, it is returned. If the give value responds
/// to a `#to_str` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// either a Ruby `String` or `nil`.
///
/// # Errors
///
/// - If the call to `#to_str` returns an error, that error is returned.
/// - If the call to `#to_str` returns anything other than a `String` or `nil`,
///   a [`TypeError`] is returned.
pub fn check_to_str(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    check_convert_type(interp, value, Ruby::String, "String", "to_str")
}

pub fn check_string_type(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    check_convert_type(interp, value, Ruby::String, "String", "to_str")
}

/// Fallible conversion of the given value to a Ruby `Array` via `#to_ary`.
///
/// If the given value is a array, it is returned. If the give value responds
/// to a `#to_ary` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// a non-`nil` Ruby `Array`.
///
/// # Errors
///
/// - If the call to `#to_ary` returns an error, that error is returned.
/// - If the call to `#to_ary` returns anything other than an `Array`, a
///   [`TypeError`] is returned.
pub fn to_ary(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    convert_type(interp, value, Ruby::Array, "Array", "to_ary", ConvertOnError::Raise)
}

/// Fallible conversion of the given value to a Ruby `Array` or `nil` via
/// `#to_ary`.
///
/// If the given value is a array, it is returned. If the give value responds
/// to a `#to_ary` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// either a Ruby `Array` or `nil`.
///
/// # Errors
///
/// - If the call to `#to_ary` returns an error, that error is returned.
/// - If the call to `#to_ary` returns anything other than an `Array` or `nil`,
///   a [`TypeError`] is returned.
pub fn check_to_ary(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    check_convert_type(interp, value, Ruby::Array, "Array", "to_ary")
}

/// Fallible coercion of the given value to a Ruby `Array` via `#to_a`.
///
/// If the given value is a array, it is returned. If the give value responds
/// to a `#to_a` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// a non-`nil` Ruby `Array`.
///
/// # Errors
///
/// - If the call to `#to_a` returns an error, that error is returned.
/// - If the call to `#to_a` returns anything other than an `Array`, a
///   [`TypeError`] is returned.
pub fn to_a(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    convert_type(interp, value, Ruby::Array, "Array", "to_a", ConvertOnError::Raise)
}

/// Fallible coercion of the given value to a Ruby `Array` or `nil` via `#to_a`.
///
/// If the given value is a array, it is returned. If the give value responds
/// to a `#to_a` method, it is called. Otherwise, a [`TypeError`] is raised.
///
/// If this function returns [`Ok`], the returned [`Value`] is guaranteed to be
/// either a Ruby `Array` or `nil`.
///
/// # Errors
///
/// - If the call to `#to_a` returns an error, that error is returned.
/// - If the call to `#to_a` returns anything other than an `Array` or `nil`,
///   a [`TypeError`] is returned.
pub fn check_to_a(interp: &mut Artichoke, value: Value) -> Result<Value, Error> {
    check_convert_type(interp, value, Ruby::Array, "Array", "to_a")
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::{conv_method_table, convert_type, to_int, ConvertOnError};
    use crate::test::prelude::*;

    #[test]
    fn conv_method_table_is_built() {
        let mut interp = interpreter();
        assert_eq!(
            conv_method_table(&mut interp).as_ptr(),
            conv_method_table(&mut interp).as_ptr()
        );
    }

    #[test]
    fn seven_implicit_conversions() {
        let mut interp = interpreter();
        for (idx, conv) in conv_method_table(&mut interp).iter().enumerate() {
            if idx < 7 {
                assert!(
                    conv.is_implicit_conversion,
                    "{} should be implicit conversion",
                    conv.method
                );
            } else {
                assert!(
                    !conv.is_implicit_conversion,
                    "{} should NOT be implicit conversion",
                    conv.method
                );
            }
        }
    }

    #[test]
    fn to_int_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_int")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_ary_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_ary")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_str_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_str")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_sym_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_sym")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_hash_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_hash")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_proc_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_proc")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_io_is_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_io")
            .unwrap();
        assert!(conv.is_implicit_conversion);
    }

    #[test]
    fn to_a_is_not_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_a")
            .unwrap();
        assert!(!conv.is_implicit_conversion);
    }

    #[test]
    fn to_s_is_not_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_s")
            .unwrap();
        assert!(!conv.is_implicit_conversion);
    }

    #[test]
    fn to_i_is_not_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_i")
            .unwrap();
        assert!(!conv.is_implicit_conversion);
    }

    #[test]
    fn to_f_is_not_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_f")
            .unwrap();
        assert!(!conv.is_implicit_conversion);
    }

    #[test]
    fn to_r_is_not_implicit_conversion() {
        let mut interp = interpreter();
        let conv = conv_method_table(&mut interp)
            .iter()
            .find(|conv| conv.method == "to_r")
            .unwrap();
        assert!(!conv.is_implicit_conversion);
    }

    #[test]
    fn implicit_to_int_reflexive() {
        let mut interp = interpreter();
        let i = interp.convert(17);
        let converted =
            convert_type(&mut interp, i, Ruby::Fixnum, "Integer", "to_int", ConvertOnError::Raise).unwrap();
        let converted = converted.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(17, converted);
    }

    #[test]
    fn implicit_to_int_conv() {
        let mut interp = interpreter();
        interp.eval(b"class A; def to_int; 17; end; end").unwrap();
        let value = interp.eval(b"A.new").unwrap();
        let converted = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap();
        let converted = converted.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(17, converted);
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > a[true]
    // (irb):2:in `<main>': no implicit conversion of true into Integer (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn implicit_to_int_true_type_error() {
        let mut interp = interpreter();
        let value = interp.convert(true);
        let err = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"no implicit conversion of true into Integer".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > a[false]
    // (irb):3:in `<main>': no implicit conversion of false into Integer (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn implicit_to_int_false_type_error() {
        let mut interp = interpreter();
        let value = interp.convert(false);
        let err = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"no implicit conversion of false into Integer".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > a[Object.new]
    // (irb):3:in `<main>': no implicit conversion of Object into Integer (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn implicit_to_int_object_type_error() {
        let mut interp = interpreter();
        let value = interp.eval(b"Object.new").unwrap();
        let err = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"no implicit conversion of Object into Integer".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > class C; def to_int; nil; end; end
    // => :to_int
    // [3.1.2] > a[C.new]
    // (irb):5:in `<main>': can't convert C to Integer (C#to_int gives NilClass) (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn implicit_to_int_object_with_nil_to_int_returns_nil() {
        let mut interp = interpreter();
        // define class
        interp.eval(b"class C; def to_int; nil; end; end").unwrap();
        let value = interp.eval(b"C.new").unwrap();
        let err = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"can't convert C to Integer (C#to_int gives NilClass)".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > class D; def to_int; 'not an integer'; end; end
    // => :to_int
    // [3.1.2] > a[D.new]
    // (irb):7:in `<main>': can't convert D to Integer (D#to_int gives String) (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn implicit_to_int_object_with_string_to_int_returns_type_error() {
        let mut interp = interpreter();
        // define class
        interp.eval(b"class D; def to_int; 'not an integer'; end; end").unwrap();
        let value = interp.eval(b"D.new").unwrap();
        let err = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"can't convert D to Integer (D#to_int gives String)".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > class F; def to_int; raise ArgumentError, 'not an integer'; end; end
    // => :to_int
    // [3.1.2] > a[F.new]
    // (irb):8:in `to_int': not an integer (ArgumentError)
    //         from (irb):9:in `<main>'
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    #[test]
    fn implicit_to_int_object_with_raising_to_int_returns_raised_exception() {
        let mut interp = interpreter();
        // define class
        interp
            .eval(b"class F; def to_int; raise ArgumentError, 'not an integer'; end; end")
            .unwrap();
        let value = interp.eval(b"F.new").unwrap();
        let err = convert_type(
            &mut interp,
            value,
            Ruby::Fixnum,
            "Integer",
            "to_int",
            ConvertOnError::Raise,
        )
        .unwrap_err();
        assert_eq!(err.name(), "ArgumentError");
        assert_eq!(err.message().as_bstr(), b"not an integer".as_bstr());
    }

    #[test]
    fn to_int_reflexive() {
        let mut interp = interpreter();
        let i = interp.convert(17);
        let converted = to_int(&mut interp, i).unwrap();
        let converted = converted.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(17, converted);
    }

    #[test]
    fn to_int_conv() {
        let mut interp = interpreter();
        interp.eval(b"class A; def to_int; 17; end; end").unwrap();
        let value = interp.eval(b"A.new").unwrap();
        let converted = to_int(&mut interp, value).unwrap();
        let converted = converted.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(17, converted);
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > a[true]
    // (irb):2:in `<main>': no implicit conversion of true into Integer (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn to_int_true_type_error() {
        let mut interp = interpreter();
        let value = interp.convert(true);
        let err = to_int(&mut interp, value).unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"no implicit conversion of true into Integer".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > a[false]
    // (irb):3:in `<main>': no implicit conversion of false into Integer (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn to_int_false_type_error() {
        let mut interp = interpreter();
        let value = interp.convert(false);
        let err = to_int(&mut interp, value).unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"no implicit conversion of false into Integer".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > a[Object.new]
    // (irb):3:in `<main>': no implicit conversion of Object into Integer (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn to_int_object_type_error() {
        let mut interp = interpreter();
        let value = interp.eval(b"Object.new").unwrap();
        let err = to_int(&mut interp, value).unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"no implicit conversion of Object into Integer".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > class C; def to_int; nil; end; end
    // => :to_int
    // [3.1.2] > a[C.new]
    // (irb):5:in `<main>': can't convert C to Integer (C#to_int gives NilClass) (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn to_int_object_with_nil_to_int_returns_nil() {
        let mut interp = interpreter();
        // define class
        interp.eval(b"class C; def to_int; nil; end; end").unwrap();
        let value = interp.eval(b"C.new").unwrap();
        let err = to_int(&mut interp, value).unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"can't convert C to Integer (C#to_int gives NilClass)".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > class D; def to_int; 'not an integer'; end; end
    // => :to_int
    // [3.1.2] > a[D.new]
    // (irb):7:in `<main>': can't convert D to Integer (D#to_int gives String) (TypeError)
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // ```
    #[test]
    fn to_int_object_with_string_to_int_returns_type_error() {
        let mut interp = interpreter();
        // define class
        interp.eval(b"class D; def to_int; 'not an integer'; end; end").unwrap();
        let value = interp.eval(b"D.new").unwrap();
        let err = to_int(&mut interp, value).unwrap_err();
        assert_eq!(err.name(), "TypeError");
        assert_eq!(
            err.message().as_bstr(),
            b"can't convert D to Integer (D#to_int gives String)".as_bstr()
        );
    }

    // ```console
    // [3.1.2] > a = []
    // => []
    // [3.1.2] > class F; def to_int; raise ArgumentError, 'not an integer'; end; end
    // => :to_int
    // [3.1.2] > a[F.new]
    // (irb):8:in `to_int': not an integer (ArgumentError)
    //         from (irb):9:in `<main>'
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    #[test]
    fn to_int_object_with_raising_to_int_returns_raised_exception() {
        let mut interp = interpreter();
        // define class
        interp
            .eval(b"class F; def to_int; raise ArgumentError, 'not an integer'; end; end")
            .unwrap();
        let value = interp.eval(b"F.new").unwrap();
        let err = to_int(&mut interp, value).unwrap_err();
        assert_eq!(err.name(), "ArgumentError");
        assert_eq!(err.message().as_bstr(), b"not an integer".as_bstr());
    }
}
