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

//! `airb` is the Artichoke implementation of `irb` and is an interactive Ruby shell
//! and [REPL][repl].
//!
//! `airb` is a readline enabled shell, although it does not persist history.
//!
//! To invoke `airb`, run:
//!
//! ```shell
//! cargo run --bin airb
//! ```
//!
//! [repl]: https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop

#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon-32x32.png")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

use std::io::{self, Write};
use std::process;

use artichoke::repl;
use termcolor::{ColorChoice, StandardStream, WriteColor};

fn main() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    if let Err(err) = repl::run(io::stdout(), &mut stderr, None) {
        // Reset colors and write the error message to stderr.
        //
        // Suppress all errors at this point (e.g. from a broken pipe) since
        // we're exiting with an error code anyway.
        let _ignored = stderr.reset();
        let _ignored = writeln!(stderr, "{err}");
        process::exit(1);
    }
}
