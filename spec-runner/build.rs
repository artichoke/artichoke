#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir::{self, CopyOptions};
use std::env;

/// Path helpers
struct Build;

impl Build {
    fn root() -> String {
        env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn mspec_vendored_dir() -> String {
        format!("{}/vendor/mspec", &Build::root(),)
    }

    fn mspec_source_dir() -> String {
        format!("{}/mspec", env::var("OUT_DIR").unwrap())
    }
}

fn main() {
    let opts = CopyOptions::new();
    let _ = dir::remove(Build::mspec_source_dir());
    dir::copy(
        Build::mspec_vendored_dir(),
        env::var("OUT_DIR").unwrap(),
        &opts,
    )
    .unwrap();
}
