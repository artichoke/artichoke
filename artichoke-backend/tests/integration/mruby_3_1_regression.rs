use artichoke_backend::prelude::*;

// https://github.com/mruby/mruby/issues/5678
#[test]
fn negative_integer_division_rounds_to_negative_infinity_negative_numerator() {
    const CODE: &[u8] = b"-1 / 2";

    let mut interp = artichoke_backend::interpreter().unwrap();
    let result = interp.eval(CODE).unwrap();
    let result = result.try_convert_into::<i64>(&interp).unwrap();
    assert_eq!(result, -1);
    interp.close();
}

// https://github.com/mruby/mruby/issues/5678
#[test]
fn negative_integer_division_rounds_to_negative_infinity_negative_denominator() {
    const CODE: &[u8] = b"1 / -2";

    let mut interp = artichoke_backend::interpreter().unwrap();
    let result = interp.eval(CODE).unwrap();
    let result = result.try_convert_into::<i64>(&interp).unwrap();
    assert_eq!(result, -1);
    interp.close();
}

// https://github.com/mruby/mruby/issues/5676
#[test]
fn terminates_when_parsing_heredocs_with_empty_delimiter() {
    const CODE: &[u8] = b"+<<''&";

    let mut interp = artichoke_backend::interpreter().unwrap();
    // Should terminate with error
    let result = interp.eval(CODE).unwrap_err();
    assert_eq!(result.name(), "SyntaxError");
    interp.close();
}
