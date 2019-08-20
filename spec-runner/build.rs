#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::path::PathBuf;

/// Path helpers
struct Build;

impl Build {
    fn root() -> PathBuf {
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
    }

    fn mspec_vendored_dir() -> PathBuf {
        Build::root().join("vendor").join("mspec")
    }

    fn mspec_source_dir() -> PathBuf {
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("mspec")
    }
}

fn main() {
    let opts = CopyOptions::new();
    let _ = dir::remove(Build::mspec_source_dir());
    dir::copy(
        Build::mspec_vendored_dir(),
        env::var_os("OUT_DIR").unwrap(),
        &opts,
    )
    .unwrap();
}
