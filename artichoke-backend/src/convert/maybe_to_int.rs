use artichoke_core::debug::Debug as _;
use artichoke_core::value::Value as _;
use spinoso_exception::TypeError;

use crate::error::Error;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug)]
pub enum MaybeToInt {
    Int(i64),
    NotApplicable,
    Err(TypeError),
    UncriticalReturn(Value),
}

/// Attempt a fallible conversion of the given value to `i64`.
///
/// Conversion steps:
///
/// - If the given value is an `Integer`, return its underlying [`i64`].
/// - If the value does not have a `to_int` method, the conversion is not
///   applicable so return [`None`].
/// - If `value.to_int` raises, return the exception in `Err(_)`.
/// - If the value returned by the conversion is an `Integer`, return its
///   underlying [`i64`].
/// - Else, the conversion is not applicable so return [`None`].
pub fn maybe_to_int(interp: &mut Artichoke, value: Value) -> Result<MaybeToInt, Error> {
    if let Ok(int) = value.try_convert_into::<i64>(interp) {
        return Ok(MaybeToInt::Int(int));
    }
    if value.respond_to(interp, "to_int")? {
        let to_int = value.funcall(interp, "to_int", &[], None)?;
        if let Ok(int) = to_int.try_convert_into::<i64>(interp) {
            return Ok(MaybeToInt::Int(int));
        }
    }
    if !value.respond_to(interp, "to_i")? {
        return Ok(MaybeToInt::NotApplicable);
    }
    let to_i = value.funcall(interp, "to_i", &[], None)?;
    if to_i.is_nil() {
        let message = format!(
            "can't convert {class} to Integer ({class}#to_i gives NilClass)",
            class = interp.class_name_for_value(value)
        );
        return Ok(MaybeToInt::Err(TypeError::from(message)));
    }
    // Uncritically return the result of `#to_i`.
    Ok(MaybeToInt::UncriticalReturn(to_i))
}

#[cfg(test)]
mod tests {
    use super::{maybe_to_int, MaybeToInt};
    use crate::test::prelude::*;

    #[test]
    fn integer_is_returned() {
        let mut interp = interpreter();
        let int = interp.eval(b"5").unwrap();
        assert!(matches!(maybe_to_int(&mut interp, int).unwrap(), MaybeToInt::Int(5)));
        let int = interp.eval(b"-5").unwrap();
        assert!(matches!(maybe_to_int(&mut interp, int).unwrap(), MaybeToInt::Int(-5)));
    }

    #[test]
    fn object_is_is_not_applicable() {
        let mut interp = interpreter();
        let int = interp.eval(b"BasicObject.new").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::NotApplicable
        ));
        let int = interp.eval(b"Object.new").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::NotApplicable
        ));
        let int = interp.eval(b"[1, 2, 3]").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::NotApplicable
        ));
    }

    #[test]
    fn conversion_with_to_int_not_applicable_to_i_nil_is_err() {
        let mut interp = interpreter();
        let int = interp
            .eval(b"class A; def to_int; Object.new; end; def to_i; nil; end; end; A.new")
            .unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::Err(err) if err.message() == b"can't convert A to Integer (A#to_i gives NilClass)"
        ));
    }

    #[test]
    fn conversion_without_to_to_i_is_nil_is_err() {
        let mut interp = interpreter();
        let int = interp.eval(b"class A; def to_i; nil; end; end; A.new").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::Err(err) if err.message() == b"can't convert A to Integer (A#to_i gives NilClass)"
        ));
    }

    #[test]
    fn conversion_without_to_to_i_non_nil_is_uncritically_returned() {
        let mut interp = interpreter();
        let int = interp.eval(b"class A; def to_i; Object.new; end; end; A.new").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::UncriticalReturn(..)
        ));
    }

    #[test]
    fn conversion_with_to_int_is_returned() {
        let mut interp = interpreter();
        let int = interp.eval(b"class A; def to_int; 99; end; end; A.new").unwrap();
        assert!(matches!(maybe_to_int(&mut interp, int).unwrap(), MaybeToInt::Int(99)));
    }

    #[test]
    fn conversion_with_raising_to_int_is_error() {
        let mut interp = interpreter();
        let int = interp
            .eval(b"class A; def to_int; raise 'bonk'; end; end; A.new")
            .unwrap();
        maybe_to_int(&mut interp, int).unwrap_err();
    }

    #[test]
    fn conversion_with_unapplicable_to_int_is_not_applicable() {
        let mut interp = interpreter();
        let int = interp.eval(b"class A; def to_int; [1, 2, 3]; end; end; A.new").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::NotApplicable
        ));
        let int = interp.eval(b"class B; def to_int; 'rip'; end; end; B.new").unwrap();
        assert!(matches!(
            maybe_to_int(&mut interp, int).unwrap(),
            MaybeToInt::NotApplicable
        ));
    }
}
