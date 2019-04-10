use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const MRUBY: &str = "mruby-2.0.1";
const MRUBY_INCLUDE_DIR: &str = "mruby-2.0.1/include";

fn main() {
    // Only build mruby if the static lib is not already built
    // The source doesn't change so we can "cache" the lib
    if !Path::new(&format!(
        "{}/{}/build/host/lib/libmruby.a",
        &env::var("CARGO_MANIFEST_DIR").unwrap(),
        MRUBY
    ))
    .exists()
    {
        let mruby_dir: PathBuf = [&env::var("CARGO_MANIFEST_DIR").unwrap(), MRUBY]
            .iter()
            .collect();
        let mruby_dir = mruby_dir.to_str().unwrap();

        // Build the mruby static library with its built in rake build system
        // rake dynamically generates some c source files so we can't build
        // directly with the `cc` crate.
        if !Command::new("rake")
            .current_dir(mruby_dir)
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to install react deps");
        }
    }

    println!("cargo:rustc-link-lib=static=mruby");
    println!(
        "cargo:rustc-link-search=native={}/{}/build/host/lib",
        &env::var("CARGO_MANIFEST_DIR").unwrap(),
        MRUBY
    );

    let bindings_path = PathBuf::from(&format!(
        "{}/src/ffi.rs",
        &env::var("CARGO_MANIFEST_DIR").unwrap()
    ));
    // Only run bindgen if the bindings don't already exist
    // The source doesn't change so we can "cache" the headers
    if !bindings_path.exists() {
        let header_path: PathBuf = [&env::var("CARGO_MANIFEST_DIR").unwrap(), "bindgen.h"]
            .iter()
            .collect();
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header(header_path.to_str().expect("bindgen.h path"))
            .clang_arg(format!("-I{}", MRUBY_INCLUDE_DIR))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to src so we can check them in.
        bindings
            .write_to_file(bindings_path)
            .expect("Couldn't write bindings!");
    }
}
