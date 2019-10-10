#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use fs_extra::dir;
use std::fs;

mod buildpath {
    use std::env;
    use std::path::PathBuf;

    pub fn root() -> PathBuf {
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
    }

    pub mod out {
        use std::env;
        use std::path::PathBuf;

        pub fn ruby_source_dir() -> PathBuf {
            PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("ruby")
        }

        pub fn generated_dir() -> PathBuf {
            PathBuf::from(env::var_os("OUT_DIR").unwrap())
                .join("src")
                .join("generated")
        }

        pub fn generated_package(package: &str) -> PathBuf {
            generated_dir().join(format!("{}.rs", package))
        }
    }

    pub mod source {
        use std::path::PathBuf;

        fn ruby_vendored_dir() -> PathBuf {
            super::root().join("vendor").join("ruby")
        }

        pub fn ruby_vendored_lib_dir() -> PathBuf {
            ruby_vendored_dir().join("lib")
        }
    }

}

mod rubylib {
    use rayon::prelude::*;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    use super::buildpath;

    pub fn build() {
        packages().par_iter().for_each(|package| {
            let sources = package_files(package)
                .trim()
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect::<Vec<_>>();
            sources.par_iter().for_each(|source| {
                let package_source = PathBuf::from(source.to_owned());
                let package_source =
                    package_source.strip_prefix(buildpath::source::ruby_vendored_lib_dir());
                let out = buildpath::out::generated_dir().join(package_source.unwrap());
                if let Some(parent) = out.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::copy(source, &out).unwrap();
            });
            generate_rust_glue(package, sources);
        });
    }

    fn package_files(package: &str) -> String {
        let script = buildpath::root()
            .join("scripts")
            .join("auto_import")
            .join("get_package_files.rb");
        let output = Command::new("ruby")
            .arg("--disable-did_you_mean")
            .arg("--disable-gems")
            .arg(script)
            .arg(buildpath::source::ruby_vendored_lib_dir())
            .arg(package)
            .output()
            .unwrap();

        if !output.status.success() {
            panic!(
                "Command executed with failing error: {}",
                String::from_utf8(output.stderr).unwrap()
            );
        }
        String::from_utf8(output.stdout).unwrap()
    }

    // The invoked Ruby script handles writing the output to disk
    fn generate_rust_glue(package: &str, sources: Vec<String>) {
        let pkg_dest = buildpath::out::generated_package(&package);
        if let Some(parent) = pkg_dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let script = buildpath::root()
            .join("scripts")
            .join("auto_import")
            .join("auto_import.rb");
        let output = Command::new("ruby")
            .arg("--disable-did_you_mean")
            .arg("--disable-gems")
            .arg(script)
            .arg(buildpath::source::ruby_vendored_lib_dir())
            .arg(package)
            .arg(pkg_dest)
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

    fn packages() -> Vec<&'static str> {
        vec![
            "abbrev",
            "base64",
            "benchmark",
            // "bigdecimal", implemented with native code in MRI
            "cgi",
            "cmath",
            // "coverage", implemented with native code in MRI
            "csv",
            // "date", implemented with native code in MRI
            // "dbm", implemented with native code in MRI and not present in some build configurations. See GH-206.
            // "debug", this package outputs on require which breaks the autogen script
            "delegate",
            "digest",
            "drb",
            "e2mmap",
            "English",
            "erb",
            "etc",
            "expect",
            // "extmk", this is part of ext for building native extensions
            "fcntl",
            "fiddle",
            "fileutils",
            "find",
            "forwardable",
            // "gdbm", implemented with native code in MRI and not present in some build configurations. See GH-206.
            "getoptlong",
            "io/console",
            "io/nonblock",
            "io/wait",
            "ipaddr",
            "irb",
            "json",
            "logger",
            "matrix",
            "mkmf",
            "monitor",
            "mutex_m",
            "net/ftp",
            "net/http",
            "net/imap",
            "net/pop",
            "net/smtp",
            // "net/telnet", as of Ruby 2.3.0, net/telnet is gemified
            "nkf",
            "objspace",
            "observer",
            "open-uri",
            "open3",
            "openssl",
            "optparse",
            "ostruct",
            "pathname",
            "prettyprint",
            "prime",
            "profile",
            "profiler",
            "pstore",
            "psych",
            "pty",
            // "racc", racc is a gem
            "racc/parser",
            // "rake", rake is a gem
            "rdoc",
            "readline",
            "resolv",
            "resolv-replace",
            // "rexml", this gem is not requirable with its package name. e.g. require 'rexml/rexml'
            // "rinda", ???
            "ripper",
            "rss",
            "rubygems",
            "scanf",
            "sdbm",
            "securerandom",
            "set",
            "shell",
            "shellwords",
            "singleton",
            "socket",
            "stringio",
            "strscan",
            "sync",
            "syslog",
            "tempfile",
            "thwait",
            "time",
            "timeout",
            "tmpdir",
            "tracer",
            "tsort",
            "un",
            // "unicode_normalize", this gem is not requirable with its package name. e.g. require 'unicode_normalize/normalize'
            "uri",
            "weakref",
            "webrick",
            // "win32ole", native code, not requirable on all platforms
            "yaml",
            // "zlib", implemented with native code in MRI
        ]
    }
}

mod release {
    use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
    use std::env;
    use std::fmt;
    use std::process::Command;
    use std::str::FromStr;
    use target_lexicon::Triple;

    pub fn build() {
        let version = env::var("CARGO_PKG_VERSION").unwrap();
        let birth_date = birthdate();
        let build_date: DateTime<Utc> = Utc::now();
        let release_date = NaiveDateTime::from_timestamp(build_date.timestamp(), 0).date();
        let revision_count = revision_count();
        let platform = platform();
        let copyright = copyright(birth_date, build_date);
        let description = description(
            version.as_str(),
            release_date,
            revision_count.as_str(),
            platform.as_str(),
        );

        emit("RUBY_RELEASE_DATE", release_date);
        emit("RUBY_RELEASE_YEAR", build_date.year());
        emit("RUBY_RELEASE_MONTH", build_date.month());
        emit("RUBY_RELEASE_DAY", build_date.day());
        emit("RUBY_REVISION", revision_count);
        emit("RUBY_PLATFORM", platform);
        emit("RUBY_COPYRIGHT", copyright);
        emit("RUBY_DESCRIPTION", description);
        emit("ARTICHOKE_COMPILER_VERSION", compiler_version());
    }

    fn emit<T: fmt::Display>(env: &str, value: T) {
        println!("cargo:rustc-env={}={}", env, value);
    }

    fn birthdate() -> DateTime<Utc> {
        // birth date taken from git log of first commit.
        let initial_commit = Command::new("git")
            .arg("rev-list")
            .arg("--max-parents=0")
            .arg("HEAD")
            .output()
            .unwrap();
        if !initial_commit.status.success() {
            panic!(
                "Command executed with failing error: {}",
                String::from_utf8(initial_commit.stderr).unwrap()
            );
        }
        let initial_commit = String::from_utf8(initial_commit.stdout).unwrap();
        let birth_date = Command::new("git")
            .arg("show")
            .arg("--no-patch")
            .arg("--format=%cD")
            .arg(initial_commit.trim())
            .output()
            .unwrap();
        if !birth_date.status.success() {
            panic!(
                "Command executed with failing error: {}",
                String::from_utf8(birth_date.stderr).unwrap()
            );
        }
        let birth_date = String::from_utf8(birth_date.stdout).unwrap();
        <DateTime<Utc>>::from(DateTime::parse_from_rfc2822(birth_date.trim()).expect("birth"))
    }

    fn revision_count() -> String {
        let revision_count = Command::new("git")
            .arg("rev-list")
            .arg("--count")
            .arg("HEAD")
            .output()
            .unwrap();
        if !revision_count.status.success() {
            panic!(
                "Command executed with failing error: {}",
                String::from_utf8(revision_count.stderr).unwrap()
            );
        }
        String::from_utf8(revision_count.stdout)
            .unwrap()
            .trim()
            .to_owned()
    }

    fn platform() -> String {
        let target_platform = Triple::from_str(env::var("TARGET").unwrap().as_str()).unwrap();
        format!(
            "{}-{}",
            target_platform.architecture, target_platform.operating_system
        )
    }

    fn copyright(birth_date: DateTime<Utc>, build_date: DateTime<Utc>) -> String {
        if birth_date.year() == build_date.year() {
            format!(
                "Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
                birth_date.year()
            )
        } else {
            format!(
                "Copyright (c) {}-{} Ryan Lopopolo <rjl@hyperbo.la>",
                birth_date.year(),
                build_date.year()
            )
        }
    }

    fn description(
        version: &str,
        release_date: NaiveDate,
        revision_count: &str,
        platform: &str,
    ) -> String {
        format!(
            "artichoke {} ({} revision {}) [{}]",
            version, release_date, revision_count, platform
        )
    }

    fn compiler_version() -> String {
        let metadata = rustc_version::version_meta().unwrap();
        let mut commit = metadata.commit_hash.unwrap();
        commit.truncate(7);
        format!(
            "Rust {} (rev {}) on {}",
            metadata.semver, commit, metadata.host
        )
    }
}

fn main() {
    let _ = dir::remove(buildpath::out::ruby_source_dir());
    fs::create_dir_all(buildpath::out::generated_dir()).unwrap();

    rubylib::build();
    release::build();
}
