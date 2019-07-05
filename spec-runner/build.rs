use fs_extra::dir::{self, CopyOptions};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// vendored mruby version
const MRUBY_REVISION: &str = "1685c45";

/// Path helpers
struct Build;

impl Build {
    fn root() -> String {
        env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn mspec_vendored_dir() -> String {
        format!("{}/spec/mspec", &Build::root(),)
    }

    fn mspec_source_dir() -> String {
        format!("{}/mspec", env::var("OUT_DIR").unwrap())
    }

    fn patch(patch: &str) -> String {
        format!("{}/spec/{}", Build::root(), patch)
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
    for patch in vec![
        "0001-Add-mruby-engine-detection-to-mspec-platform-guard.patch",
        "0002-Add-fallback-for-Fixnum-size.patch",
        "0003-Short-circuit-in-mspec-helpers-tmp.patch",
        "0004-Call-Kernel-require-instead-of-Kernel-load.patch",
        "0005-Remove-reference-to-SystemExit.patch",
        "0006-Use-explicit-call-to-to_int.patch",
    ] {
        if !Command::new("bash")
            .arg("-c")
            .arg(format!("patch -p1 < '{}'", Build::patch(patch)))
            .current_dir(Build::mspec_source_dir())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to patch mspec sources with {}", patch);
        }
    }
}
