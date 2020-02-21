#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(intra_doc_link_resolution_failure)]

//! `airb` is the Artichoke implementation of `irb` and is an interactive Ruby shell
//! and [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).
//!
//! `airb` is a readline enabled shell, although it does not persist history.
//!
//! To invoke `airb`, run:
//!
//! ```shell
//! cargo run --bin airb
//! ```

use artichoke_frontend::repl;
use std::io;

fn main() -> Result<(), repl::Error> {
    repl::run(io::stdout(), io::stderr(), None)
}
