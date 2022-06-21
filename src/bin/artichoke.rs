#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
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
//!
//! ```console
//! $ cargo run -q --bin artichoke -- --help
//! artichoke 0.1.0-pre.0
//! Artichoke is a Ruby made with Rust.
//!
//! USAGE:
//!     artichoke [OPTIONS] [ARGS]
//!
//! ARGS:
//!     <programfile>
//!     <arguments>...
//!
//! OPTIONS:
//!         --copyright                 print the copyright
//!     -e <commands>                   one line of script. Several -e's allowed. Omit [programfile]
//!     -h, --help                      Print help information
//!     -V, --version                   Print version information
//!         --with-fixture <fixture>    file whose contents will be read into the `$fixture` global
//! ```

use std::env;
use std::error;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use artichoke::ruby::{self, Args};
use clap::builder::ArgAction;
use clap::{Arg, ArgMatches, Command};
use termcolor::{ColorChoice, StandardStream, WriteColor};

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            // Suppress all errors at this point (e.g. from a broken pipe) since
            // we're exiting with an error code anyway.
            let _ignored = writeln!(io::stderr(), "{}", err);
            process::exit(2);
        }
    };

    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    match ruby::run(args, io::stdin(), &mut stderr) {
        Ok(Ok(())) => {}
        Ok(Err(())) => process::exit(1),
        Err(err) => {
            // Reset colors and write the error message to stderr.
            //
            // Suppress all errors at this point (e.g. from a broken pipe) since
            // we're exiting with an error code anyway.
            let _ignored = stderr.reset();
            let _ignored = writeln!(stderr, "{}", err);
            process::exit(1);
        }
    }
}

fn parse_args() -> Result<Args> {
    let matches = clap_matches(env::args_os())?;

    let commands = matches
        .get_many::<OsString>("commands")
        .into_iter()
        .flat_map(|s| s.map(Clone::clone))
        .collect::<Vec<_>>();
    let mut args = Args::empty()
        .with_copyright(*matches.get_one::<bool>("copyright").expect("defaulted by clap"))
        .with_fixture(matches.get_one::<PathBuf>("fixture").cloned());

    // If no `-e` arguments are given, the first positional argument is the
    // `programfile`. All trailing arguments are ARGV to the script.
    //
    // If there are `-e` arguments given, there is no programfile and all
    // positional arguments are ARGV to the inline script.
    //
    // ```console
    // $ ruby -e 'puts ARGV.inspect' a b c
    // ["a", "b", "c"]
    // $ cat foo.rb
    // puts ARGV.inspect
    // $ ruby foo.rb a b c
    // ["a", "b", "c"]
    // $ ruby bar.rb a b c
    // ruby: No such file or directory -- bar.rb (LoadError)
    // ```
    if commands.is_empty() {
        if let Some(programfile) = matches.get_one::<PathBuf>("programfile").cloned() {
            args = args.with_programfile(Some(programfile));
            if let Some(argv) = matches.get_many::<OsString>("arguments") {
                let ruby_program_argv = argv.map(Clone::clone).collect::<Vec<_>>();
                args = args.with_argv(ruby_program_argv);
            }
        }
    } else {
        args = args.with_commands(commands);
        if let Some(first_arg) = matches.get_one::<PathBuf>("programfile").cloned() {
            if let Some(argv) = matches.get_many::<OsString>("arguments") {
                let ruby_program_argv = [OsString::from(first_arg)]
                    .into_iter()
                    .chain(argv.map(Clone::clone))
                    .collect::<Vec<_>>();
                args = args.with_argv(ruby_program_argv);
            } else {
                args = args.with_argv(vec![OsString::from(first_arg)]);
            }
        }
    }

    Ok(args)
}

fn command() -> Command<'static> {
    Command::new("artichoke")
        .about("Artichoke is a Ruby made with Rust.")
        .arg(
            Arg::new("copyright")
                .long("copyright")
                .action(ArgAction::SetTrue)
                .help("print the copyright"),
        )
        .arg(
            Arg::new("commands")
                .short('e')
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(OsString))
                .help(r"one line of script. Several -e's allowed. Omit [programfile]"),
        )
        .arg(
            Arg::new("fixture")
                .long("with-fixture")
                .takes_value(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("file whose contents will be read into the `$fixture` global"),
        )
        .arg(Arg::new("programfile").value_parser(clap::value_parser!(PathBuf)))
        .arg(
            Arg::new("arguments")
                .multiple_values(true)
                .value_parser(clap::value_parser!(OsString)),
        )
        .version(env!("CARGO_PKG_VERSION"))
        .trailing_var_arg(true)
}

// NOTE: This routine is plucked from `ripgrep` as of
// 9f924ee187d4c62aa6ebe4903d0cfc6507a5adb5.
//
// `ripgrep` is licensed with the MIT License Copyright (c) 2015 Andrew Gallant.
//
// https://github.com/BurntSushi/ripgrep/blob/9f924ee187d4c62aa6ebe4903d0cfc6507a5adb5/LICENSE-MIT
//
// See https://github.com/artichoke/artichoke/issues/1301.

/// Returns a clap matches object if the given arguments parse successfully.
///
/// Otherwise, if an error occurred, then it is returned unless the error
/// corresponds to a `--help` or `--version` request. In which case, the
/// corresponding output is printed and the current process is exited
/// successfully.
fn clap_matches<I, T>(args: I) -> Result<ArgMatches>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let err = match command().try_get_matches_from(args) {
        Ok(matches) => return Ok(matches),
        Err(err) => err,
    };
    if err.use_stderr() {
        return Err(err.into());
    }
    // Explicitly ignore any error returned by write!. The most likely error
    // at this point is a broken pipe error, in which case, we want to ignore
    // it and exit quietly.
    //
    // (This is the point of this helper function. clap's functionality for
    // doing this will panic on a broken pipe error.)
    let _ignored = write!(io::stdout(), "{}", err);
    process::exit(0);
}
