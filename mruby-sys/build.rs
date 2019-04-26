use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// vendored mruby version
const MRUBY_VERSION: &str = "c078758";
/// env var to force a clean build
const MRUBY_SYS_CLEAN: &str = "MRUBY_SYS_CLEAN";

/// Path helpers
struct Build;

impl Build {
    fn root() -> String {
        env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn ext_source_dir() -> String {
        format!("{}/mruby-sys", &Build::root())
    }

    fn ext_include_dir() -> String {
        format!("{}/include", Build::ext_source_dir())
    }

    fn mruby_source_dir() -> String {
        format!("{}/vendor/mruby-{}", &Build::root(), MRUBY_VERSION)
    }

    fn mruby_include_dir() -> String {
        format!("{}/include", Build::mruby_source_dir())
    }

    fn mruby_build_dir() -> String {
        format!("{}/{}", &env::var("OUT_DIR").unwrap(), "mruby-build")
    }

    fn mruby_out_dir() -> String {
        format!("{}/host/lib", &Build::mruby_build_dir())
    }

    fn mruby_out_lib() -> String {
        format!("{}/{}", &Build::mruby_out_dir(), "libmruby.a")
    }

    fn bindgen_source_header() -> String {
        format!("{}/mruby-sys.h", &Build::ext_include_dir())
    }
}

/// Set env variable `MRUBY_SYS_CLEAN` to force a clean build and bindgen.
fn is_clean_build() -> bool {
    env::var(MRUBY_SYS_CLEAN).is_ok()
}

/// Build libruby.a static library if a clean build or the lib doesn't exist.
fn should_build_lib() -> bool {
    let out = Build::mruby_out_lib();
    let lib = Path::new(&out);
    is_clean_build() || !lib.exists()
}

/// Run bindgen to generate FFI bindings if a clean build or we are building
/// libruby.a static library.
fn should_run_bindgen() -> bool {
    is_clean_build() || should_build_lib()
}

fn main() {
    // Only build mruby if the static lib is not already built or if this is a
    // clean build. The source doesn't change so we can "cache" the lib.
    if should_build_lib() {
        // Build the mruby static library with its built in rake build system
        // rake dynamically generates some c source files so we can't build
        // directly with the `cc` crate.
        if !Command::new("rake")
            .env("MRUBY_BUILD_DIR", Build::mruby_build_dir())
            .current_dir(Build::mruby_source_dir())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to build libmruby.a");
        }
    }

    // Set static lib and search path flags so rustc will link libmruby.a
    // into our binary.
    println!("cargo:rustc-link-lib=static=mruby");
    println!("cargo:rustc-link-search=native={}", Build::mruby_out_dir(),);

    // Build the extension library
    cc::Build::new()
        .file(format!("{}/src/mruby-sys/ext.c", Build::ext_source_dir()))
        .include(Build::mruby_include_dir())
        .include(Build::ext_include_dir())
        .compile("libmrubysys.a");

    // Only run bindgen if this is a clean build. The source doesn't change so
    // we can "cache" the headers.
    if should_run_bindgen() {
        let bindings_path: PathBuf = [&Build::root(), "src", "ffi.rs"].iter().collect();
        let bindings = bindgen::Builder::default()
            .header(Build::bindgen_source_header())
            .clang_arg(format!("-I{}", Build::mruby_include_dir()))
            .clang_arg(format!("-I{}", Build::ext_include_dir()))
            .generate()
            .expect("Unable to generate mruby bindings");
        bindings
            .write_to_file(bindings_path)
            .expect("Unable to write mruby bindings");
    }

    println!("cargo:rerun-if-env-changed={}", MRUBY_SYS_CLEAN);
}
