#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::option_if_let_else)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]

//! `spec-runner` is the ruby/spec runner for Artichoke.
//!
//! `spec-runner` is a wrapper around `MSpec` and ruby/spec that works with the
//! Artichoke virtual file system. `spec-runner` runs the specs declared in a
//! manifest file.
//!
//! # Spec Manifest
//!
//! `spec-runner` is invoked with a YAML manifest that specifies which specs to
//! run. The manifest can run whole suites, like all of the `StringScanner`
//! specs, or specific specs, like the `Array#drop` spec. The manifest supports
//! marking specs as skipped.
//!
//! ```toml
//! [specs.core.array]
//! include = "set"
//! specs = [
//!   "any",
//!   "append",
//!   "drop",
//! ]
//!
//! [specs.library.stringscanner]
//! include = "all"
//!
//! [specs.library.time]
//! include = "none"
//!
//! [specs.library.uri]
//! include = "all"
//! skip = ["parse"]
//! ```
//!
//! # Usage
//!
//! ```console
//! $ cargo run -q -p spec-runner -- --help
//! spec-runner 0.6.0
//! ruby/spec runner for Artichoke.
//!
//! USAGE:
//!     spec-runner [FLAGS] [OPTIONS] <config>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -q, --quiet      Suppress spec failures when exiting
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -f, --format <formatter>    Output spec results in YAML [default: artichoke]  [possible values: artichoke, summary,
//!                                 tagger, yaml]
//!
//! ARGS:
//!     <config>    Path to TOML config file
//! ```

#![doc(html_favicon_url = "https://www.artichokeruby.org/favicon-32x32.png")]
#![doc(html_logo_url = "https://www.artichokeruby.org/artichoke-logo.svg")]

#[macro_use]
extern crate rust_embed;

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process;
use std::str;

use artichoke::backtrace;
use artichoke::prelude::*;
use clap::{App, Arg};
use termcolor::{ColorChoice, StandardStream, WriteColor};

mod model;
mod mspec;
mod rubyspec;

use model::{Config, Suite};
use mspec::Formatter;

/// CLI specification for `spec-runner`.
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Args {
    /// Path to TOML config file.
    config: PathBuf,
    /// `MSpec` formatter.
    formatter: Formatter,
}

/// Main entry point.
pub fn main() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);

    let app = App::new("spec-runner");
    let app = app
        .about("CLI specification for `spec-runner`")
        .about("ruby/spec runner for Artichoke.");
    let app = app.arg(
        Arg::with_name("formatter")
            .long("format")
            .short("f")
            .default_value("artichoke")
            .possible_values(&["artichoke", "summary", "tagger", "yaml"])
            .required(false)
            .help("Output spec results in YAML"),
    );
    let app = app.arg(
        Arg::with_name("quiet")
            .long("quiet")
            .short("q")
            .required(false)
            .help("Suppress spec failures when exiting"),
    );
    let app = app.arg(
        Arg::with_name("config")
            .takes_value(true)
            .multiple(false)
            .required(true)
            .help("Path to TOML config file"),
    );
    let app = app.version(env!("CARGO_PKG_VERSION"));

    let matches = app.get_matches();

    let formatter = matches
        .value_of_os("formatter")
        .expect("formatter has a default value, clap should ensure");
    let formatter = formatter
        .to_str()
        .expect("formatter has possible values, clap should ensure");
    let formatter = formatter.parse::<Formatter>();
    let formatter = match formatter {
        Ok(f) => f,
        Err(err) => {
            // Suppress all errors at this point (e.g. from a broken pipe) since
            // we're exiting with an error code anyway.
            let _ignored = writeln!(&mut stderr, "{}", err);
            process::exit(1);
        }
    };
    let quiet = matches.is_present("quiet");

    let args = if let Some(config) = matches.value_of_os("config") {
        Args {
            config: config.into(),
            formatter,
        }
    } else {
        // Suppress all errors at this point (e.g. from a broken pipe) since
        // we're exiting with an error code anyway.
        let _ignored = writeln!(&mut stderr, "Missing required spec configuration");
        process::exit(1);
    };

    match try_main(&mut stderr, &args) {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(if quiet { 0 } else { 1 }),
        Err(err) => {
            // Suppress all errors at this point (e.g. from a broken pipe) since
            // we're exiting with an error code anyway.
            let _ignored = writeln!(&mut stderr, "{}", err);
            process::exit(1);
        }
    }
}

/// Result-returning entry point.
///
/// Initializes an interpreter, parses the spec manifest, and invokes the
/// Artichoke `MSpec` entry point.
///
/// # Errors
///
/// If the config at `path` cannot be read or parsed, an error is returned.
///
/// If an Artichoke interpreter cannot be initialized, an error is returned.
///
/// If the `MSpec` runner returns an error, an error is returned.
fn try_main<W>(stderr: W, args: &Args) -> Result<bool, Box<dyn Error>>
where
    W: Write + WriteColor,
{
    let config = fs::read(&args.config)?;
    let config = str::from_utf8(config.as_slice())?;
    let config = toml::from_str::<Config>(config)?;

    let mut interp = artichoke::interpreter()?;

    rubyspec::init(&mut interp)?;
    let mut specs = vec![];
    for name in rubyspec::Specs::iter() {
        let path = Path::new(&*name);
        let is_fixture = path
            .components()
            .map(Component::as_os_str)
            .any(|component| component == OsStr::new("fixture"));
        let is_shared = path
            .components()
            .map(Component::as_os_str)
            .any(|component| component == OsStr::new("shared"));
        if is_fixture || is_shared {
            if let Some(contents) = mspec::Sources::get(&name) {
                interp.def_rb_source_file(path, contents.data)?;
            }
            continue;
        }
        if is_require_path(&config, &name) {
            specs.push(name.into_owned());
        }
    }
    mspec::init(&mut interp)?;
    let result = match mspec::run(&mut interp, args.formatter, specs.iter().map(String::as_str)) {
        Ok(result) => Ok(result),
        Err(exc) => {
            backtrace::format_cli_trace_into(stderr, &mut interp, &exc)?;
            Err(exc.into())
        }
    };
    interp.close();
    result
}

/// Determine if an embedded ruby/spec should be tested.
///
/// This function evaluates a ruby/spec source file against the parsed spec
/// manifest config to determine if the source should be tested.
#[must_use]
pub fn is_require_path(config: &Config, name: &str) -> bool {
    // Use an inner function to allow short-circuiting `None` with the `?`
    // operator.
    fn inner(config: &Config, name: &str) -> Option<bool> {
        let path = Path::new(name);
        let mut components = path.components();
        let family = components.next()?.as_os_str();

        let suites = config.suites_for_family(family)?;
        let suite_name = components.next()?.as_os_str();
        let (_, suite) = suites.iter().find(|(name, _)| OsStr::new(name) == suite_name)?;
        let spec_name = components.next()?.as_os_str().to_str()?;

        match suite {
            Suite::All(ref all) if all.skip.iter().flatten().any(|name| spec_name.starts_with(name)) => Some(false),
            Suite::All(..) => Some(true),
            Suite::None => Some(false),
            Suite::Set(ref set) if set.specs.iter().any(|name| spec_name.starts_with(name)) => Some(true),
            Suite::Set(..) => Some(false),
        }
    }
    // And the convert to the expected `bool`.
    matches!(inner(config, name), Some(true))
}
