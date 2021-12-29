use crate::run;

const BINARY: &str = "artichoke";
const FIXTURES_ROOT: &str = "./tests/integration/fixtures/";

#[test]
fn test_hello_world() {
    let app_name = "hello_world.rb";
    let path = format!("{}{}", FIXTURES_ROOT, app_name);
    insta::assert_toml_snapshot!(run(BINARY, &[&path]).unwrap());
}

#[test]
fn test_fizz_buzz() {
    let app_name = "fizz_buzz.rb";
    let path = format!("{}{}", FIXTURES_ROOT, app_name);
    insta::assert_toml_snapshot!(run(BINARY, &[&path]).unwrap());
}
