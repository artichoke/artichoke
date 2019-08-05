#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir;
use std::env;
use std::fs;
use std::process::Command;

/// Path helpers
struct Build;

impl Build {
    fn root() -> String {
        env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn generated_dir() -> String {
        format!("{}/src/generated", env::var("OUT_DIR").unwrap())
    }

    fn ruby_source_dir() -> String {
        format!("{}/ruby_2_6_3", env::var("OUT_DIR").unwrap())
    }

    fn ruby_vendored_dir() -> String {
        format!("{}/vendor/ruby_2_6_3", &Build::root())
    }

    fn ruby_vendored_lib_dir() -> String {
        format!("{}/lib", &Build::ruby_vendored_dir())
    }

    fn patch(patch: &str) -> String {
        format!("{}/vendor/{}", Build::root(), patch)
    }

    fn get_package_files(package: &str) -> String {
        let output = Command::new("ruby")
            .arg(format!(
                "{}/scripts/auto_import/get_package_files.rb",
                Build::root()
            ))
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

    fn generated_package_out(package: &str) -> String {
        format!("{}/{}.rs", Build::generated_dir(), package)
    }

    // The invoked Ruby script handles writing the output to disk
    fn generate_rust_glue(package: &str, sources: Vec<String>) {
        let output = Command::new("ruby")
            .arg(format!(
                "{}/scripts/auto_import/auto_import.rb",
                Build::root()
            ))
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

    for package in vec!["benchmark"] {
        let sources = Build::get_package_files(package)
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<Vec<_>>();
        for package_file in &sources {
            let output_file = format!("{}/{}.rb", Build::generated_dir(), package);
            fs::copy(package_file, &output_file).unwrap();
        }
        Build::generate_rust_glue(package, sources);
    }
    println!("{}", Build::ruby_source_dir());

    for patch in vec![] {
        if !Command::new("bash")
            .arg("-c")
            .arg(format!("patch -p1 < '{}'", Build::patch(patch)))
            .current_dir(env::var("OUT_DIR").unwrap())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to patch Ruby lib sources with {}", patch);
        }
    }
}
