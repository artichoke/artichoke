#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)] disable for bitflags v2 churn
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! `artichoke` is the `ruby` binary frontend to Artichoke.
//!
//! `artichoke` supports executing programs via files, stdin, or inline with one
//! or more `-e` flags.
//!
//! Artichoke does not yet support reading from the local file system. A
//! temporary workaround is to inject data into the interpreter with the
//! `--with-fixture` flag, which reads file contents into a `$fixture` global.

use std::io::{self, Write};
use std::process;

use artichoke::ruby::cli;
use artichoke::ruby::{self, ExecutionResult};
use termcolor::{ColorChoice, StandardStream, WriteColor};

fn main() {
    let args = cli::parse_args();

    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    match ruby::run(args, io::stdin(), &mut stderr) {
        Ok(ExecutionResult::Success) => {}
        Ok(ExecutionResult::Error(..)) => process::exit(1),
        Err(err) => {
            // Reset colors and write the error message to stderr.
            //
            // Suppress all errors at this point (e.g. from a broken pipe) since
            // we're exiting with an error code anyway.
            let _ignored = stderr.reset();
            let _ignored = writeln!(stderr, "{err}");
            process::exit(1);
        }
    }
}
