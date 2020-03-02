#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(rust_2018_idioms)]

//! `spec-runner` is a wrapper around `MSpec` and ruby/spec that works with the
//! Artichoke virtual filesystem.
//!
//! `spec-runner` is invokable directly by passing paths to spec files as command
//! line arguments. `spec-runner` is sensitive to CWD relative to the specs it
//! wraps, so in practice it is easier to invoke `spec-runner` via the `spec.rb`
//! wrapper in `scripts`.
//!
//! ```console
//! $ ruby scripts/spec.rb --help
//! spec.rb runs ruby/specs against Artichoke and MRI.
//!
//! Usage: scripts/spec.rb artichoke [ --timed ITERATIONS | --profile ] [ passing | family [ component [ spec ] ] ]
//! Usage: scripts/spec.rb ruby [ --timed ITERATIONS ] family [ component [ spec ] ]
//!
//! Examples:
//!     $ scripts/spec.rb artichoke passing
//!     $ scripts/spec.rb artichoke core
//!     $ scripts/spec.rb artichoke core string
//!     $ scripts/spec.rb ruby core string scan
//!     $ scripts/spec.rb artichoke --timed 30 core string scan
//!     $ scripts/spec.rb artichoke --profile passing
//! ```

#[macro_use]
extern crate rust_embed;

use artichoke_backend::{Artichoke, LoadSources};
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use structopt::StructOpt;

mod model;
mod mspec;
mod rubyspec;

#[derive(Debug, StructOpt)]
#[structopt(name = "spec-runner", about = "ruby/spec runner for Artichoke.")]
struct Opt {
    /// Path to YAML config file.
    #[structopt(parse(from_os_str))]
    config: PathBuf,
}

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

pub fn try_main(config: &Path) -> Result<bool, Box<dyn Error>> {
    let config = fs::read(config)?;
    let config = str::from_utf8(config.as_slice())?;
    let config = serde_yaml::from_str::<model::Config>(config)?;

    let mut interp = artichoke_backend::interpreter()?;

    rubyspec::init(&mut interp)?;
    let specs = rubyspec::Specs::iter()
        .filter_map(|path| {
            is_require_path(&mut interp, &config, path.as_ref()).map(|_| path.into_owned())
        })
        .collect::<Vec<_>>();

    mspec::init(&mut interp)?;
    let result = mspec::run(&mut interp, specs.as_slice())?;
    Ok(result)
}

pub fn is_require_path(interp: &mut Artichoke, config: &model::Config, name: &str) -> Option<()> {
    let path = Path::new(name);
    let is_shared = path.components().any(|component| {
        component.as_os_str() == OsStr::new("fixture")
            || component.as_os_str() == OsStr::new("shared")
    });
    if is_shared {
        if let Some(contents) = mspec::Sources::get(name.as_ref()) {
            interp.def_rb_source_file(name.as_bytes(), contents).ok()?;
        }
        return None;
    }
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
