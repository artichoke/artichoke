#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use artichoke_frontend::repl;
use std::io;

fn main() -> Result<(), artichoke_frontend::repl::Error> {
    repl::run(io::stdout(), io::stderr(), None)
}
