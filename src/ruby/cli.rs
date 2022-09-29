//! Command line interface parser for the `ruby` binary.

use std::env;
use std::ffi::OsString;
use std::iter;
use std::path::PathBuf;
use std::process;

use clap::builder::ArgAction;
use clap::{Arg, ArgMatches, Command};

use super::Args;

/// Parse CLI arguments into an [`Args`] struct.
///
/// # Errors
///
/// If an invalid argument is provided, an error is returned.
#[must_use]
pub fn parse_args() -> Args {
    let matches = clap_matches(env::args_os());

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
                let ruby_program_argv = iter::once(OsString::from(first_arg))
                    .chain(argv.map(Clone::clone))
                    .collect::<Vec<_>>();
                args = args.with_argv(ruby_program_argv);
            } else {
                args = args.with_argv(vec![OsString::from(first_arg)]);
            }
        }
    }

    args
}

// NOTE: This routine is plucked from `ripgrep` as of commit
// `9f924ee187d4c62aa6ebe4903d0cfc6507a5adb5`.
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
#[must_use]
fn clap_matches<I, T>(args: I) -> ArgMatches
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let err = match cli().try_get_matches_from(args) {
        Ok(matches) => return matches,
        Err(err) => err,
    };
    // Explicitly ignore any error returned by write!. The most likely error
    // at this point is a broken pipe error, in which case, we want to ignore
    // it and exit quietly.
    let _ignored = err.print();
    process::exit(0);
}

/// Build a [`clap`] CLI parser.
#[must_use]
pub fn cli() -> Command {
    Command::new("artichoke")
        .about("Artichoke is a Ruby made with Rust.")
        .version(env!("CARGO_PKG_VERSION"))
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
                .value_parser(clap::value_parser!(PathBuf))
                .help("file whose contents will be read into the `$fixture` global"),
        )
        .arg(Arg::new("programfile").value_parser(clap::value_parser!(PathBuf)))
        .arg(
            Arg::new("arguments")
                .num_args(..)
                .value_parser(clap::value_parser!(OsString))
                .trailing_var_arg(true),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        cli().debug_assert();
    }
}
