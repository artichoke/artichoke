use artichoke_core::value::Value as _;

use crate::error::Error;
use crate::value::Value;
use crate::Artichoke;

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
pub fn maybe_to_int(interp: &mut Artichoke, value: Value) -> Result<Option<i64>, Error> {
    if let Ok(int) = value.try_convert_into::<i64>(interp) {
        return Ok(Some(int));
    }
    if !value.respond_to(interp, "to_int")? {
        return Ok(None);
    }
    let value = value.funcall(interp, "to_int", &[], None)?;
    if let Ok(int) = value.try_convert_into::<i64>(interp) {
        return Ok(Some(int));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::maybe_to_int;
    use crate::test::prelude::*;

    #[test]
    fn integer_is_returned() {
        let mut interp = interpreter();
        let int = interp.eval(b"5").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), Some(5));
        let int = interp.eval(b"-5").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), Some(-5));
    }

    #[test]
    fn object_is_is_not_applicable() {
        let mut interp = interpreter();
        let int = interp.eval(b"BasicObject.new").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), None);
        let int = interp.eval(b"Object.new").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), None);
        let int = interp.eval(b"[1, 2, 3]").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), None);
    }

    #[test]
    fn conversion_with_to_int_is_returned() {
        let mut interp = interpreter();
        let int = interp.eval(b"class A; def to_int; 99; end; end; A.new").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), Some(99));
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
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), None);
        let int = interp.eval(b"class B; def to_int; 'rip'; end; end; B.new").unwrap();
        assert_eq!(maybe_to_int(&mut interp, int).unwrap(), None);
    }
}
