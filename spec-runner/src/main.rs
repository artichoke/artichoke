#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs, intra_doc_link_resolution_failure)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

//! `spec-runner` is the ruby/spec runner for Artichoke.
//!
//! `spec-runner` is a wrapper around `MSpec` and ruby/spec that works with the
//! Artichoke virtual filesystem. `spec-runner` runs the specs declared in a
//! manifest file.
//!
//! # Spec Manifest
//!
//! `spec-runner` is invoked with a YAML manifest that specifies which specs to
//! run. The manifest can run whole suites, like all of the `StringScanner`
//! specs, or specific specs, like the `Array#drop` spec. The manifest supports
//! marking specs as skipped.
//!
//! ```yaml
//! core:
//!   - suite: array
//!     specs:
//!       - any
//!       - append
//!       - array
//!   - suite: comparable
//!   - suite: string
//!     specs:
//!       - scan
//! library:
//!   - suite: stringscanner
//!   - suite: uri
//!     skip:
//!       - parse
//! ```
//!
//! # Usage
//!
//! ```console
//! $ cargo run -q -p spec-runner -- --help
//! spec-runner 0.1.0
//! ruby/spec runner for Artichoke.
//!
//! USAGE:
//!     spec-runner <config>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! ARGS:
//!     <config>    Path to YAML config file
//! ```

#[macro_use]
extern crate rust_embed;

use artichoke_backend::LoadSources;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process;
use std::str;
use structopt::StructOpt;

mod model;
mod mspec;
mod rubyspec;

/// CLI specification for `spec-runner`.
#[derive(Debug, StructOpt)]
#[structopt(name = "spec-runner", about = "ruby/spec runner for Artichoke.")]
struct Opt {
    /// Path to YAML config file.
    #[structopt(parse(from_os_str))]
    config: PathBuf,
}

/// Main entrypoint.
pub fn main() {
    let opt = Opt::from_args();
    match try_main(opt.config.as_path()) {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(err) => {
            let _ = writeln!(io::stderr(), "{}", err);
            process::exit(1);
        }
    }
}

/// Result-returning entrypoint.
///
/// Initializes an interpreter, parses the spec manifest, and invokes the
/// Artichoke `MSpec` entrypoint.
///
/// # Errors
///
/// If the config at `path` cannot be read or parsed, an error is returned.
///
/// If an Artichoke interpreter cannot be initialized, an error is returned.
///
/// If the `MSpec` runner returns an error, an error is returned.
pub fn try_main(config: &Path) -> Result<bool, Box<dyn Error>> {
    let config = fs::read(config)?;
    let config = str::from_utf8(config.as_slice())?;
    let config = serde_yaml::from_str::<model::Config>(config)?;

    let mut interp = artichoke_backend::interpreter()?;

    rubyspec::init(&mut interp)?;
    let mut specs = vec![];
    for name in rubyspec::Specs::iter() {
        let path = Path::new(name.as_ref());
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
                interp.def_rb_source_file(name.as_bytes(), contents)?;
            }
            continue;
        }
        if is_require_path(&config, &name).is_some() {
            specs.push(name.into_owned())
        }
    }
    mspec::init(&mut interp)?;
    let result = mspec::run(&mut interp, specs.iter().map(String::as_str))?;
    Ok(result)
}

/// Determine if an embedded ruby/spec should be tested.
///
/// This function evaluates a ruby/spec source file against the parsed spec
/// manifest config to determine if the source should be tested.
pub fn is_require_path(config: &model::Config, name: &str) -> Option<()> {
    let path = Path::new(name);
    let mut components = path.components();
    let family = components.next()?.as_os_str();
    let suites = config.suites_for_family(family)?;
    let suite_name = components.next()?.as_os_str();
    let suite = suites
        .iter()
        .find(|suite| OsStr::new(suite.suite.as_str()) == suite_name)?;
    let spec_name = components.next()?.as_os_str().to_str()?;
    if let Some(ref skip) = suite.skip {
        if skip.iter().any(|name| spec_name.starts_with(name)) {
            return None;
        }
    }
    if let Some(ref specs) = suite.specs {
        specs
            .iter()
            .position(|name| spec_name.starts_with(name))
            .map(|_| ())
    } else {
        Some(())
    }
}
