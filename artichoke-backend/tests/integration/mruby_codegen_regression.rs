use artichoke_backend::prelude::*;

// https://github.com/mruby/mruby/issues/5752
#[test]
fn assign_class_to_fully_qualified_constant() {
    const CODE: &[u8] = b"::MyClass = Class.new";

    let mut interp = artichoke_backend::interpreter().unwrap();
    let _result = interp.eval(CODE).unwrap();
    let result = interp.eval(b"::MyClass.name").unwrap();
    let result = result.try_convert_into_mut::<&str>(&mut interp).unwrap();
    assert_eq!(result, "MyClass");
    interp.close();
}

// https://github.com/mruby/mruby/issues/5752
#[test]
fn assign_immediate_to_fully_qualified_constant() {
    const CODE: &[u8] = b"::MyConst = 7";

    let mut interp = artichoke_backend::interpreter().unwrap();
    let _result = interp.eval(CODE).unwrap();
    let result = interp.eval(b"::MyConst").unwrap();
    let result = result.try_convert_into::<i64>(&interp).unwrap();
    assert_eq!(result, 7);
    interp.close();
}

// https://github.com/mruby/mruby/issues/5785
#[test]
fn numparam_as_hash_key() {
    const CODE: &[u8] = br#"\
def it(name, &blk); end

it "adds the entries from other, overwriting duplicate keys. Returns self" do
  h = { _1: 'a', _2: '3' }
end
"#;

    let mut interp = artichoke_backend::interpreter().unwrap();
    let _result = interp.eval(CODE).unwrap();
    interp.close();
}
