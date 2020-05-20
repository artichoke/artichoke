#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

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

#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon.ico")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

use artichoke::repl;
use std::io::{self, Write};
use std::process;

fn main() {
    if let Err(err) = repl::run(io::stdout(), io::stderr(), None) {
        let _ = writeln!(io::stderr(), "{}", err);
        process::exit(1);
    }
}
