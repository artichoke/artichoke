#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

//! `spec-runner` is a wrapper around MSpec and ruby/spec that works with the
//! Artichoke [virtual filesystem](/artichoke-vfs).
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

use std::env;
use std::fs;
use std::process;

mod mspec;

pub fn main() {
    let interp = match artichoke_backend::interpreter() {
        Ok(interp) => interp,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
    if let Err(err) = mspec::init(&interp) {
        eprintln!("{}", err);
        process::exit(1);
    };
    let mut mspec_runner = mspec::Runner::new(interp);

    let mut args = env::args();
    let mut specs = vec![];
    // ignore binary name
    args.next();
    for spec in args {
        // multi-threading is not supported
        if spec.contains("thread/") || spec.contains("mutex/owned_spec.rb") {
            continue;
        }
        // access modifiers are a mess which means the fixtures in
        // `core/module/fixtures/classes.rb` are unloadable
        if spec.contains("module/") {
            continue;
        }
        let contents = fs::read(&spec).unwrap();
        mspec_runner.add_spec(spec.as_str(), contents).unwrap();
        specs.push(spec);
    }
    match mspec_runner.run() {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
