#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

use mruby_bin::repl;
use std::io;

fn main() -> Result<(), repl::Error> {
    repl::run(io::stdout(), io::stderr(), None)
}
