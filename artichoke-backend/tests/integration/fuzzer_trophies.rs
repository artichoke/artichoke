use artichoke_backend::prelude::*;

// https://github.com/mruby/mruby/issues/5676
//
// https://github.com/rust-fuzz/trophy-case/issues/115
// https://github.com/rust-fuzz/trophy-case/pull/116
#[test]
fn terminates_when_parsing_unclosed_heredocs_with_empty_delimiter() {
    const CODE: &[u8] = b"+<<''&";

    let mut interp = artichoke_backend::interpreter().unwrap();
    // Should terminate with error
    let result = interp.eval(CODE).unwrap_err();
    assert_eq!(result.name(), "SyntaxError");
    interp.close();
}

// https://github.com/mruby/mruby/issues/5676
#[test]
fn terminates_when_parsing_unclosed_heredocs() {
    const CODE: &[u8] = b"+<<'DOC'&";

    let mut interp = artichoke_backend::interpreter().unwrap();
    // Should terminate with error
    let result = interp.eval(CODE).unwrap_err();
    assert_eq!(result.name(), "SyntaxError");
    interp.close();
}
