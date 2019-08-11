#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Path helpers
struct Build;

impl Build {
    fn root() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
    }

    fn generated_dir() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("src")
            .join("generated")
    }

    fn ruby_source_dir() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("ruby_2_6_3")
    }

    fn ruby_vendored_dir() -> PathBuf {
        Build::root().join("vendor").join("ruby_2_6_3")
    }

    fn ruby_vendored_lib_dir() -> PathBuf {
        Build::ruby_vendored_dir().join("lib")
    }

    fn patch(patch: &str) -> PathBuf {
        Build::root().join("vendor").join(patch)
    }

    fn get_package_files(package: &str) -> String {
        let script = Build::root()
            .join("scripts")
            .join("auto_import")
            .join("get_package_files.rb");
        let output = Command::new("ruby")
            .arg("--disable-did_you_mean")
            .arg("--disable-gems")
            .arg(script)
            .arg(Build::ruby_vendored_lib_dir())
            .arg(package)
            .output()
            .unwrap();

        if !output.status.success() {
            panic!(
                "Command executed with failing error: {}",
                String::from_utf8(output.stderr).unwrap()
            );
        }
        return String::from_utf8(output.stdout).unwrap();
    }

    fn generated_package_out(package: &str) -> PathBuf {
        Build::generated_dir().join(format!("{}.rs", package))
    }

    // The invoked Ruby script handles writing the output to disk
    fn generate_rust_glue(package: &str, sources: Vec<String>) {
        let script = Build::root()
            .join("scripts")
            .join("auto_import")
            .join("auto_import.rb");
        let output = Command::new("ruby")
            .arg("--disable-did_you_mean")
            .arg("--disable-gems")
            .arg(script)
            .arg(Build::ruby_vendored_lib_dir())
            .arg(&package)
            .arg(Build::generated_package_out(&package))
            .arg(sources.join(","))
            .output()
            .unwrap();

        if !output.status.success() {
            panic!(
                "Command executed with failing error: {}",
                String::from_utf8(output.stderr).unwrap()
            );
        }
    }
}

fn main() {
    let _ = dir::remove(Build::ruby_source_dir());
    fs::create_dir_all(Build::generated_dir()).unwrap();
    println!("{:?}", Build::generated_dir());

    for package in vec!["benchmark", "uri"] {
        fs::create_dir_all(Build::generated_dir().join(package)).unwrap();
        let sources = Build::get_package_files(package)
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<Vec<_>>();
        for source in &sources {
            let package_source = PathBuf::from(source.to_owned());
            let package_source = package_source.strip_prefix(Build::ruby_vendored_lib_dir());
            let out = Build::generated_dir().join(package_source.unwrap());
            fs::copy(source, &out).unwrap();
        }
        Build::generate_rust_glue(package, sources);
    }

    for patch in vec![
        "00001-uri-brackets-for-ivar-interpolation.patch",
        "00002-uri-defined-keyword.patch",
    ] {
        if !Command::new("bash")
            .arg("-c")
            .arg(format!(
                "patch -p1 < '{}'",
                Build::patch(patch).to_str().unwrap()
            ))
            .current_dir(env::var("OUT_DIR").unwrap())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to patch Ruby lib sources with {}", patch);
        }
    }
}
