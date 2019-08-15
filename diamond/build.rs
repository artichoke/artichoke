#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir::{self, CopyOptions};
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

/// vendored mruby version
const MRUBY_REVISION: &str = "0c5ebd75";

/// Path helpers
struct Build;

impl Build {
    fn root() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("diamond")
    }

    fn gems() -> Vec<&'static str> {
        vec!["mruby-compiler", "mruby-eval", "mruby-error"]
    }

    fn build_config() -> PathBuf {
        Build::root().join("build_config.rb")
    }

    fn ext_source_dir() -> PathBuf {
        Build::root().join("mruby-sys")
    }

    fn ext_include_dir() -> PathBuf {
        Build::ext_source_dir().join("include")
    }

    fn ext_source_file() -> PathBuf {
        Build::ext_source_dir()
            .join("src")
            .join("mruby-sys")
            .join("ext.c")
    }

    fn mruby_source_dir() -> PathBuf {
        Build::root()
            .join("vendor")
            .join(format!("mruby-{}", MRUBY_REVISION))
    }

    fn mruby_minirake() -> PathBuf {
        Build::mruby_source_dir().join("minirake")
    }

    fn mruby_include_dir() -> PathBuf {
        Build::mruby_source_dir().join("include")
    }

    fn mruby_build_dir() -> PathBuf {
        Build::root().join("mruby-build")
    }

    fn patch(patch: &str) -> PathBuf {
        Build::root().join("vendor").join(patch)
    }
}

fn main() {
    let opts = CopyOptions::new();
    let _ = dir::remove(Build::root());
    dir::copy(
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        env::var("OUT_DIR").unwrap(),
        &opts,
    )
    .unwrap();
    for patch in vec![
        "0001-Support-parsing-a-Regexp-literal-with-CRuby-options.patch",
        "0002-Allow-externally-specifying-MRB_API.patch",
    ] {
        println!(
            "cargo:rerun-if-changed={}",
            Build::patch(patch).to_string_lossy()
        );
        if !Command::new("bash")
            .arg("-c")
            .arg(format!(
                "patch -p1 < '{}'",
                Build::patch(patch).to_string_lossy()
            ))
            .current_dir(Build::mruby_source_dir())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to patch mruby sources with {}", patch);
        }
    }

    let mut gembox = String::from("MRuby::GemBox.new { |conf| ");
    for gem in Build::gems() {
        gembox.push_str("conf.gem core: '");
        gembox.push_str(gem);
        gembox.push_str("';");
    }
    gembox.push('}');
    fs::write(Build::root().join("sys.gembox"), gembox).unwrap();

    // Build the mruby static library with its built in minirake build system.
    // minirake dynamically generates some c source files so we can't build
    // directly with the `cc` crate.
    env::set_var("MRUBY_REVISION", MRUBY_REVISION);
    println!("cargo:rustc-env=MRUBY_REVISION={}", MRUBY_REVISION);
    println!("cargo:rerun-if-env-changed=MRUBY_REVISION");
    println!("cargo:rerun-if-env-changed=PROFILE");
    println!(
        "cargo:rerun-if-changed={}",
        Build::build_config().to_string_lossy()
    );
    if !Command::new(Build::mruby_minirake())
        .arg("--jobs")
        .arg("4")
        .env("MRUBY_BUILD_DIR", Build::mruby_build_dir())
        .env("MRUBY_CONFIG", Build::build_config())
        .current_dir(Build::mruby_source_dir())
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to build generate mruby C sources");
    }

    let mut sources = HashSet::new();
    sources.insert(Build::ext_source_file());
    let walker = WalkDir::new(Build::mruby_source_dir()).into_iter();
    for entry in walker {
        if let Ok(entry) = entry {
            let is_gem = Build::gems()
                .iter()
                .any(|gem| entry.path().to_str().map(|path| path.contains(gem)) == Some(true));
            let is_core = entry.path().to_str().map(|path| {
                let core_prefix = Build::mruby_source_dir().join("src");
                path.starts_with(core_prefix.to_str().unwrap())
            }) == Some(true);
            if is_gem || is_core {
                if entry.path().extension().and_then(OsStr::to_str) == Some("c") {
                    sources.insert(entry.path().to_owned());
                }
            }
        }
    }
    let walker = WalkDir::new(Build::mruby_build_dir().join("sys")).into_iter();
    let mut counter = 0;
    for entry in walker {
        if let Ok(entry) = entry {
            let is_gem = Build::gems()
                .iter()
                .any(|gem| entry.path().to_str().map(|path| path.contains(gem)) == Some(true));
            let is_mrbgem_infra = entry
                .path()
                .to_str()
                .map(|path| path.ends_with("gem_init.c") || path.ends_with("mrblib.c"))
                == Some(true);
            if is_gem || is_mrbgem_infra {
                let mut path = entry.path().to_path_buf();
                if path.to_string_lossy().contains("test") {
                    continue;
                }
                if path.extension().and_then(OsStr::to_str) == Some("c") {
                    if path.file_name().and_then(OsStr::to_str) == Some("gem_init.c") {
                        let dest = path
                            .parent()
                            .unwrap()
                            .join(format!("gem_init_{}.c", counter));
                        counter += 1;
                        fs_extra::file::move_file(
                            path,
                            dest.as_path(),
                            &fs_extra::file::CopyOptions::new(),
                        )
                        .unwrap();
                        path = dest;
                    }
                    sources.insert(path.to_owned());
                }
            }
        }
    }

    // Build the extension library
    let mut build = cc::Build::new();
    build
        .warnings(false)
        .files(sources)
        .include(Build::mruby_include_dir())
        .include(Build::ext_include_dir())
        .define("MRB_DISABLE_STDIO", None)
        .define("MRB_UTF8_STRING", None)
        .define("MRB_DISABLE_DIRECT_THREADING", None);

    for gem in Build::gems() {
        let mut dir = "include";
        if gem == "mruby-compiler" {
            dir = "core";
        }
        let gem_include_dir = Build::mruby_source_dir()
            .join("mrbgems")
            .join(gem)
            .join(dir);
        build.include(gem_include_dir);
    }

    build.compile("libmrubysys.a");
}
