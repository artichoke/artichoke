use crate::convert::{Convert, Error, TryConvert};
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl Convert<String> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: String) -> Self {
        // mruby `String` is just bytes, so get a pointer to the underlying
        // `&[u8]` infallibly and convert that to a `Value`.
        Self::convert(interp, value.as_bytes())
    }
}

impl Convert<&str> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: &str) -> Self {
        // mruby `String` is just bytes, so get a pointer to the underlying
        // `&[u8]` infallibly and convert that to a `Value`.
        Self::convert(interp, value.as_bytes())
    }
}

impl TryConvert<Value> for String {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        interp: &Artichoke,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        if value.ruby_type() == Ruby::Symbol {
            return Ok(value.to_s());
        }
        // `Vec<u8>` converter operates on `Ruby::String`
        let bytes = <Vec<u8>>::try_convert(interp, value).map_err(|err| Error {
            from: err.from,
            to: Rust::String,
        })?;
        // This converter requires that the bytes be valid UTF-8 data. If the
        // `mrb_value` is binary data, use the `Vec<u8>` converter.
        Self::from_utf8(bytes).map_err(|_| Error {
            from: Ruby::String,
            to: Rust::String,
        })
    }
}

#[cfg(test)]
// Convert<String> is implemented in terms of Convert<&str> so only implement
// the tests for String to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;
    use std::convert::TryInto;

    use crate::convert::{Convert, Error, TryConvert};
    use crate::eval::Eval;
    use crate::sys;
    use crate::types::{Ruby, Rust};
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let expected = Error {
            from: Ruby::Object,
            to: Rust::String,
        };
        let result = unsafe { String::try_convert(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_string(s: String) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, s.clone());
        let ptr = unsafe { sys::mrb_string_value_ptr(interp.borrow().mrb, value.inner()) };
        let len = unsafe { sys::mrb_string_value_len(interp.borrow().mrb, value.inner()) };
        let string =
            unsafe { std::slice::from_raw_parts(ptr as *const u8, len.try_into().unwrap()) };
        s.as_bytes() == string
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn string_with_value(s: String) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, s.clone());
        value.to_s() == s
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(s: String) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, s.clone());
        let value = unsafe { String::try_convert(&interp, value) }.expect("convert");
        value == s
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, b);
        let value = unsafe { String::try_convert(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::String,
        });
        value == expected
    }

    #[test]
    fn symbol_to_string() {
        let interp = crate::interpreter().expect("init");
        let value = interp.eval(":sym").expect("eval");
        let value = unsafe { String::try_convert(&interp, value) }.expect("convert");
        assert_eq!(&value, "sym");
    }
}
