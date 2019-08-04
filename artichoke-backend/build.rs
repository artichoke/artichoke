#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::process::Command;

/// Path helpers
struct Build;

impl Build {
    fn root() -> String {
        env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn ruby_source_dir() -> String {
        format!("{}/vendor/ruby_2_6_3", env::var("OUT_DIR").unwrap())
    }

    fn ruby_vendored_dir() -> String {
        format!("{}/vendor/ruby_2_6_3", &Build::root())
    }

    fn patch(patch: &str) -> String {
        format!("{}/vendor/{}", Build::root(), patch)
    }
}

fn main() {
    let opts = CopyOptions::new();
    let _ = dir::remove(Build::ruby_source_dir());
    dir::copy(
        Build::ruby_vendored_dir(),
        env::var("OUT_DIR").unwrap(),
        &opts,
    )
    .unwrap();
    for patch in vec![] {
        if !Command::new("bash")
            .arg("-c")
            .arg(format!("patch -p1 < '{}'", Build::patch(patch)))
            .current_dir(env::var("OUT_DIR").unwrap())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to patch mspec sources with {}", patch);
        }
    }
}
