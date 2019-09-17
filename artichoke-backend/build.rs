#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use fs_extra::dir;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use target_lexicon::Triple;

/// Path helpers
struct Build;

impl Build {
    fn root() -> PathBuf {
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
    }

    fn generated_dir() -> PathBuf {
        PathBuf::from(env::var_os("OUT_DIR").unwrap())
            .join("src")
            .join("generated")
    }

    fn ruby_source_dir() -> PathBuf {
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("ruby")
    }

    fn ruby_vendored_dir() -> PathBuf {
        Build::root().join("vendor").join("ruby")
    }

    fn ruby_vendored_lib_dir() -> PathBuf {
        Build::ruby_vendored_dir().join("lib")
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
        String::from_utf8(output.stdout).unwrap()
    }

    fn generated_package_out(package: &str) -> PathBuf {
        Build::generated_dir().join(format!("{}.rs", package))
    }

    // The invoked Ruby script handles writing the output to disk
    fn generate_rust_glue(package: &str, sources: Vec<String>) {
        if let Some(parent) = Build::generated_package_out(&package).parent() {
            fs::create_dir_all(parent).unwrap();
        }
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

    let packages = vec![
        "abbrev",
        "base64",
        "benchmark",
        // "bigdecimal", implemented with native code in MRI
        "cgi",
        "cmath",
        // "coverage", implemented with native code in MRI
        "csv",
        // "date", implemented with native code in MRI
        // "dbm",
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
        "gdbm",
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
        "zlib",
    ];

    packages.par_iter().for_each(|package| {
        let sources = Build::get_package_files(package)
            .trim()
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<Vec<_>>();
        sources.par_iter().for_each(|source| {
            println!(
                "source = {:?}, prefix = {:?}",
                source,
                Build::ruby_vendored_lib_dir()
            );
            let package_source = PathBuf::from(source.to_owned());
            let package_source = package_source.strip_prefix(Build::ruby_vendored_lib_dir());
            let out = Build::generated_dir().join(package_source.unwrap());
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            println!("source = {:?}, dest = {:?}", source, out);
            fs::copy(source, &out).unwrap();
        });
        Build::generate_rust_glue(package, sources);
    });

    // Release information

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
    let birth_date =
        <DateTime<Utc>>::from(DateTime::parse_from_rfc2822(birth_date.trim()).expect("birth"));
    let build_date: DateTime<Utc> = Utc::now();
    println!(
        "cargo:rustc-env=RUBY_RELEASE_DATE={}",
        NaiveDateTime::from_timestamp(build_date.timestamp(), 0).date()
    );
    println!("cargo:rustc-env=RUBY_RELEASE_YEAR={}", build_date.year());
    println!("cargo:rustc-env=RUBY_RELEASE_MONTH={}", build_date.month());
    println!("cargo:rustc-env=RUBY_RELEASE_DAY={}", build_date.day());

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
    let revision_count = String::from_utf8(revision_count.stdout).unwrap();
    println!("cargo:rustc-env=RUBY_REVISION={}", revision_count);

    let target_platform = Triple::from_str(env::var("TARGET").unwrap().as_str()).unwrap();
    let ruby_platform = format!(
        "{}-{}",
        target_platform.architecture, target_platform.operating_system
    );
    println!("cargo:rustc-env=RUBY_PLATFORM={}", ruby_platform);

    println!(
        "cargo:rustc-env=RUBY_COPYRIGHT=Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
        if birth_date.year() == build_date.year() {
            format!("{}", birth_date.year())
        } else {
            format!("{}-{}", birth_date.year(), build_date.year())
        }
    );

    println!(
        "cargo:rustc-env=RUBY_DESCRIPTION=artichoke {} ({} revision {}) [{}]",
        env::var("CARGO_PKG_VERSION").unwrap(),
        NaiveDateTime::from_timestamp(build_date.timestamp(), 0).date(),
        revision_count.trim(),
        ruby_platform
    );

    // compiler info
    let metadata = rustc_version::version_meta().unwrap();
    let mut commit = metadata.commit_hash.unwrap();
    commit.truncate(7);
    println!(
        "cargo:rustc-env=ARTICHOKE_COMPILER_VERSION=Rust {} (rev {}) on {}",
        metadata.semver, commit, metadata.host
    );
}
