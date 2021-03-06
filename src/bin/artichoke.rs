#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(renamed_and_removed_lints)]
#![allow(unknown_lints)]
#![warn(broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! `artichoke` is the `ruby` binary frontend to Artichoke.
//!
//! `artichoke` supports executing programs via files, stdin, or inline with one
//! or more `-e` flags.
//!
//! Artichoke does not yet support reading from the local filesystem. A
//! temporary workaround is to inject data into the interpreter with the
//! `--with-fixture` flag, which reads file contents into a `$fixture` global.
//!
//! ```console
//! $ cargo run -q --bin artichoke -- --help
//! artichoke 0.1.0-pre.0
//! Artichoke is a Ruby made with Rust.
//!
//! USAGE:
//!     artichoke [FLAGS] [OPTIONS] [--] [programfile]...
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
//!     <programfile>...
//! ```

use artichoke::ruby::{self, Args};
use clap::{App, AppSettings, Arg};
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use termcolor::{ColorChoice, StandardStream, WriteColor};

fn main() {
    let app = App::new("artichoke");
    let app = app.about("Artichoke is a Ruby made with Rust.");
    let app = app.arg(
        Arg::with_name("copyright")
            .takes_value(false)
            .multiple(false)
            .help("print the copyright")
            .long("copyright"),
    );
    let app = app.arg(
        Arg::with_name("commands")
            .takes_value(true)
            .multiple(true)
            .help(r"one line of script. Several -e's allowed. Omit [programfile]")
            .short("e"),
    );
    let app = app.arg(
        Arg::with_name("fixture")
            .takes_value(true)
            .multiple(false)
            .help("file whose contents will be read into the `$fixture` global")
            .long("with-fixture"),
    );
    let app = app.arg(Arg::with_name("programfile").takes_value(true).multiple(true));
    let app = app.version(env!("CARGO_PKG_VERSION"));
    let app = app.setting(AppSettings::TrailingVarArg);

    let matches = app.get_matches();
    let mut args = Args::empty()
        .with_copyright(matches.is_present("copyright"))
        .with_commands(
            matches
                .values_of_os("commands")
                .into_iter()
                .flat_map(|v| v.map(OsString::from))
                .collect(),
        )
        .with_fixture(matches.value_of_os("fixture").map(PathBuf::from));
    if let Some(mut positional) = matches.values_of_os("programfile") {
        if let Some(programfile) = positional.next() {
            args = args.with_programfile(Some(programfile.into()));
        }
        let ruby_program_argv = positional.map(OsString::from).collect::<Vec<_>>();
        args = args.with_argv(ruby_program_argv);
    }

    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    match ruby::run(args, io::stdin(), &mut stderr) {
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
