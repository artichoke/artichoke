#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(intra_doc_link_resolution_failure)]

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

use artichoke_frontend::ruby::{self, Error};

fn main() {
    match ruby::entrypoint() {
        Ok(_) => {}
        Err(Error::Ruby(err)) => eprintln!("{}", err),
        Err(Error::Fail(err)) => eprintln!("{}", err),
    }
}
