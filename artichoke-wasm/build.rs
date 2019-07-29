#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use std::env;

fn main() {
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    let metadata = rustc_version::version_meta().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", metadata.semver);
    let mut commit = metadata.commit_hash.unwrap();
    commit.truncate(7);
    println!("cargo:rustc-env=RUSTC_COMMIT_HASH={}", commit);
    let date = metadata.commit_date.unwrap();
    println!("cargo:rustc-env=RUSTC_COMMIT_DATE={}", date);
}
