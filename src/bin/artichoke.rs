#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs, intra_doc_link_resolution_failure)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

//! `artichoke` is the `ruby` binary frontend to Artichoke.
//!
//! `artichoke` supports executing programs via files, stdin, or inline with one or
//! more `-e` flags.
//!
//! Artichoke does not yet support reading from the local filesystem. A temporary
//! workaround is to inject data into the interpreter with the `--with-fixture`
//! flag, which reads file contents into a `$fixture` global.
//!
//! ```console
//! $ cargo run --bin artichoke -- --help
//! artichoke 0.1.0
//! Artichoke is a Ruby made with Rust.
//!
//! USAGE:
//!     artichoke [FLAGS] [OPTIONS] [--] [programfile]
//!
//! FLAGS:
//!         --copyright    print the copyright
//!     -h, --help         Prints help information
//!     -V, --version      Prints version information
//!
//! OPTIONS:
//!     -e <commands>...                one line of script. Several -e's allowed. Omit [programfile]
//!         --with-fixture <fixture>    file whose contents will be read into the `$fixture` global
//!
//! ARGS:
//!     <programfile>
//! ```

use artichoke::ruby;
use std::io::{self, Write};
use std::process;
use termcolor::{ColorChoice, StandardStream, WriteColor};

fn main() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    match ruby::entrypoint(io::stdin(), &mut stderr) {
        Ok(Ok(())) => {}
        Ok(Err(())) => process::exit(1),
        Err(err) => {
            // reset colors
            let _ = stderr.reset();
            let _ = writeln!(stderr, "{}", err);
            process::exit(1);
        }
    }
}
