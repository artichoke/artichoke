use std::env;
use std::fs;

use mruby::extn::test;
use mruby::extn::test::mspec::MSpec;
use mruby::interpreter::Interpreter;

pub fn main() {
    let interp = Interpreter::create().expect("mrb init");
    test::init(&interp).expect("mspec init");
    let mut mspec_runner = MSpec::runner(interp);

    let mut args = env::args();
    let mut specs = vec![];
    // ignore binary name
    args.next();
    for spec in args {
        let contents = fs::read_to_string(&spec).unwrap();
        mspec_runner.add_spec(spec.as_str(), contents).unwrap();
        specs.push(spec);
    }
    // this will panic if a spec fails.
    mspec_runner.run();
    for spec in specs {
        println!("OK {}", spec);
    }
}
