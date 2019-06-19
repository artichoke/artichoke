use std::env;
use std::fs;
use std::panic;
use std::process;

use mruby::extn::test;
use mruby::extn::test::mspec::MSpec;
use mruby::interpreter::Interpreter;

pub fn main() {
    // this will panic if a spec fails.
    let result = panic::catch_unwind(|| {
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
        mspec_runner.run()
    });
    process::exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
