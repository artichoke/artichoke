#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir;
// use fs_extra::file::{self, CopyOptions};
use std::env;
use std::fs;
use std::path::Path;
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

    fn patch(patch: &str) -> String {
        format!("{}/vendor/{}", Build::root(), patch)
    }

    fn get_package_files(package: &str) -> String {
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "ruby -I {}/lib {}/scripts/auto_import/get_package_files.rb '{}' '{}'",
                &Build::ruby_vendored_dir(),
                &Build::root(),
                &Build::ruby_vendored_dir(),
                package
            ))
            .output()
            .unwrap();

        if !output.status.success() {
            panic!("Command executed with failing error code");
        }
        return String::from_utf8(output.stdout).unwrap();
    }

    fn generated_package_out(package: &str) -> String {
        format!("{}/{}.rs", Build::generated_dir(), package)
    }

    fn generate_rust_glue(package: &str) {
        println!(
            "{}",
            format!(
                "ruby {}/scripts/auto_import/auto_import.rb '{}' '{}'",
                &Build::root(),
                package,
                Build::generated_package_out(&package),
            )
        );
        Command::new("bash")
            .arg("-c")
            .arg(format!(
                "ruby {}/scripts/auto_import/auto_import.rb '{}' '{}'",
                &Build::ruby_vendored_dir(),
                package,
                Build::generated_package_out(&package),
            ))
            .output()
            .unwrap();
    }
}

fn main() {
    // let opts = file::CopyOptions::new();
    let _ = dir::remove(Build::ruby_source_dir());

    fs::create_dir_all(Build::generated_dir()).unwrap();

    for package in vec!["benchmark"] {
        for package_file in Build::get_package_files(package).split("\n") {
            if package_file != "" {
                let output_file =
                    package_file.replace(&Build::ruby_vendored_dir(), &Build::ruby_source_dir());
                let dir = Path::new(&output_file).parent().unwrap();

                fs::create_dir_all(&dir).unwrap();

                fs::copy(package_file, &output_file).unwrap();
            }
        }

        Build::generate_rust_glue(package)
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
            panic!("Failed to patch mspec sources with {}", patch);
        }
    }
}
