#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

#[macro_use]
extern crate rust_embed;

use std::env;
use std::fs;
use std::process;

mod mspec;

pub fn main() {
    let interp = match mruby::interpreter() {
        Ok(interp) => interp,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
    if let Err(err) = mspec::init(&interp) {
        eprintln!("{}", err);
        process::exit(1);
    };
    let mut mspec_runner = mspec::Runner::new(interp);

    let mut args = env::args();
    let mut specs = vec![];
    // ignore binary name
    args.next();
    for spec in args {
        if spec == "mutex/owned_spec.rb" {
            continue;
        }
        let contents = fs::read(&spec).unwrap();
        mspec_runner.add_spec(spec.as_str(), contents).unwrap();
        specs.push(spec);
    }
    match mspec_runner.run() {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
