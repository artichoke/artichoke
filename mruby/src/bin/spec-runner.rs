use std::env;
use std::fs;
use std::process;

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
        let contents = fs::read(&spec).unwrap();
        mspec_runner.add_spec(spec.as_str(), contents).unwrap();
        specs.push(spec);
    }
    match mspec_runner.run() {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
