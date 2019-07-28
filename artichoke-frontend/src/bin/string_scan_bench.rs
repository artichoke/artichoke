#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

use artichoke_backend::convert::Convert;
use artichoke_backend::eval::{EvalContext, MrbEval};
use artichoke_backend::sys;
use artichoke_backend::value::Value;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() {
    let interp = match artichoke_backend::interpreter() {
        Ok(interp) => interp,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
    let mrb = interp.borrow().mrb;

    // program is either supplied as a file via command line argument or piped
    // in via stdin.
    let mut program = vec![];
    if let Some(file) = env::args().nth(1) {
        match File::open(file) {
            Ok(mut file) => {
                if let Err(err) = file.read_to_end(&mut program) {
                    eprintln!("Unable to read program: {}", err);
                    process::exit(1);
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    } else if let Err(err) = io::stdin().read_to_end(&mut program) {
        eprintln!("Unable to read program: {}", err);
        process::exit(1);
    }

    let data = Value::from_mrb(
        &interp,
        include_bytes!("../../ruby/fixtures/learnxinyminutes.txt").as_ref(),
    );
    data.protect();
    unsafe { sys::mrb_gv_set(mrb, interp.borrow_mut().sym_intern("$data"), data.inner()) }
    let ctx = EvalContext::new("(main)");
    if let Err(err) = interp.eval_with_context(program, ctx) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
