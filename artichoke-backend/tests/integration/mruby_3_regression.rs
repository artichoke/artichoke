use artichoke_backend::prelude::*;

#[test]
fn bignum_parses() {
    const CODE: &[u8] = b"def foo; 0x8000_0000_0000_0000; end";

    let mut interp = artichoke_backend::interpreter().unwrap();
    interp.eval(CODE).unwrap();
    interp.close();
}

#[test]
fn presym_cmp_method_fallthrough() {
    const CLASS_DEF: &[u8] = b"class A; include Comparable; def <=>(other); 1; end; end";
    const RANGE: &[u8] = b"A.new..A.new";

    let mut interp = artichoke_backend::interpreter().unwrap();
    interp.eval(CLASS_DEF).unwrap();
    interp.eval(RANGE).unwrap();
    interp.close();
}

#[test]
fn division_by_signed_zero_gives_negative_infinity() {
    const CODE: &[u8] = b"(1.0/-0.0).infinite? == -1";

    let mut interp = artichoke_backend::interpreter().unwrap();
    assert!(interp.eval(CODE).unwrap().try_convert_into::<bool>(&interp).unwrap());
    interp.close();
}
