#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![doc(deny(warnings))]

mod buildpath {
    use std::env;
    use std::path::PathBuf;

    pub fn crate_root() -> PathBuf {
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
    }

    pub fn build_root() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("artichoke-mruby")
    }

    pub mod source {
        use std::path::PathBuf;

        pub fn ruby_vendored_lib_dir() -> PathBuf {
            super::crate_root().join("vendor").join("ruby").join("lib")
        }

        pub fn mruby_build_config() -> PathBuf {
            super::crate_root().join("build_config.rb")
        }

        pub fn mruby_bootstrap_gembox() -> PathBuf {
            super::crate_root().join("bootstrap.gembox")
        }
    }
}

mod libmruby {
    use std::collections::HashMap;
    use std::env;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::{Component, PathBuf};
    use std::process::Command;
    use std::str::FromStr;
    use target_lexicon::{Architecture, OperatingSystem, Triple};
    use walkdir::WalkDir;

    use super::buildpath;

    fn gems() -> Vec<&'static str> {
        vec![
            "mruby-compiler",     // Ruby parser and bytecode generation
            "mruby-error",        // `mrb_raise`, `mrb_protect`
            "mruby-eval",         // eval, instance_eval, and friends
            "mruby-metaprog",     // APIs on Kernel and Module for accessing classes and variables
            "mruby-method",       // `Method`, `UnboundMethod`, and method APIs on Kernel and Module
            "mruby-toplevel-ext", // expose API for top self
            "mruby-enumerator",   // Enumerator class from core
            "mruby-enum-lazy",    // Enumerable#lazy
            "mruby-fiber",        // Fiber class from core, required by mruby-enumerator
            "mruby-pack",         // Array#pack and String#unpack
            "mruby-sprintf",      // Kernel#sprintf, Kernel#format, String#%
            "mruby-class-ext",    // Pending removal, see GH-32
            "mruby-proc-ext",     // required by mruby-method, see GH-32
        ]
    }

    pub fn build_config() -> PathBuf {
        mruby_source_dir().join("build_config.rb")
    }

    pub fn bootstrap_gembox() -> PathBuf {
        mruby_source_dir().join("bootstrap.gembox")
    }

    fn ext_source_dir() -> PathBuf {
        buildpath::crate_root().join("mruby-sys")
    }

    fn ext_include_dir() -> PathBuf {
        ext_source_dir().join("include")
    }

    fn ext_source_file() -> PathBuf {
        ext_source_dir().join("src").join("mruby-sys").join("ext.c")
    }

    fn wasm_include_dir() -> PathBuf {
        buildpath::crate_root()
            .join("vendor")
            .join("emscripten")
            .join("system")
            .join("include")
            .join("libc")
    }

    fn mruby_source_dir() -> PathBuf {
        buildpath::build_root().join("mruby")
    }

    fn mruby_minirake() -> PathBuf {
        mruby_source_dir().join("minirake")
    }

    fn mruby_include_dir() -> PathBuf {
        mruby_source_dir().join("include")
    }

    fn mruby_build_dir() -> PathBuf {
        buildpath::build_root().join("mruby-build")
    }

    fn mruby_generated_source_dir() -> PathBuf {
        mruby_build_dir().join("sys")
    }

    fn bindgen_source_header() -> PathBuf {
        ext_include_dir().join("mruby-sys.h")
    }

    fn staticlib(target: &Triple, mrb_int: &str) {
        let mut gembox = String::from("MRuby::GemBox.new { |conf| ");
        for gem in gems() {
            gembox.push_str("conf.gem core: '");
            gembox.push_str(gem);
            gembox.push_str("';");
        }
        gembox.push('}');
        fs::write(mruby_source_dir().join("sys.gembox"), gembox).unwrap();

        // Build the mruby static library with its built in minirake build system.
        // minirake dynamically generates some c source files so we can't build
        // directly with the `cc` crate.
        println!(
            "cargo:rerun-if-changed={}",
            build_config().to_str().unwrap()
        );
        if !Command::new(mruby_minirake())
            .arg("--jobs")
            .arg("4")
            .env("MRUBY_BUILD_DIR", mruby_build_dir())
            .env("MRUBY_CONFIG", build_config())
            .current_dir(mruby_source_dir())
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to build generate mruby C sources");
        }

        let mut sources = HashMap::new();
        sources.insert(ext_source_file(), ext_source_file());
        let walker = WalkDir::new(mruby_source_dir()).into_iter();
        for entry in walker {
            if let Ok(entry) = entry {
                let source = entry.path();
                let relative_source = source.strip_prefix(mruby_source_dir()).unwrap();
                let mut is_buildable = source.strip_prefix(mruby_source_dir().join("src")).is_ok();
                for gem in gems() {
                    is_buildable |= source
                        .components()
                        .any(|component| component == Component::Normal(OsStr::new(gem)));
                }
                if relative_source
                    .components()
                    .any(|component| component == Component::Normal(OsStr::new("build")))
                {
                    // Skip build artifacts generated by minirake invocation that we
                    // do not intend to build.
                    is_buildable = false;
                }
                if relative_source
                    .components()
                    .any(|component| component == Component::Normal(OsStr::new("test")))
                {
                    // Skip build artifacts generated by minirake invocation that we
                    // do not intend to build.
                    is_buildable = false;
                }
                if is_buildable && source.extension().and_then(OsStr::to_str) == Some("c") {
                    sources.insert(relative_source.to_owned(), source.to_owned());
                }
            }
        }
        let walker = WalkDir::new(mruby_generated_source_dir()).into_iter();
        for entry in walker {
            if let Ok(entry) = entry {
                let source = entry.path();
                let mut is_buildable = true;
                let relative_source = source.strip_prefix(mruby_generated_source_dir()).unwrap();
                if relative_source
                    .components()
                    .any(|component| component == Component::Normal(OsStr::new("test")))
                {
                    // Skip build artifacts generated by minirake invocation that we
                    // do not intend to build.
                    is_buildable = false;
                }
                if is_buildable && source.extension().and_then(OsStr::to_str) == Some("c") {
                    sources.insert(relative_source.to_owned(), source.to_owned());
                }
            }
        }
        // Build the extension library
        let mut build = cc::Build::new();
        build
            .warnings(false)
            .files(sources.values())
            .include(mruby_include_dir())
            .include(ext_include_dir())
            .define("MRB_DISABLE_STDIO", None)
            .define("MRB_UTF8_STRING", None)
            .define(mrb_int, None)
            .define("DISABLE_GEMS", None);

        if env::var("CARGO_FEATURE_ARTICHOKE_ARRAY").is_ok() {
            build.define("ARTICHOKE", None);
        }

        for gem in gems() {
            let mut dir = "include";
            if gem == "mruby-compiler" {
                dir = "core";
            }
            let gem_include_dir = mruby_source_dir().join("mrbgems").join(gem).join(dir);
            build.include(gem_include_dir);
        }

        if let Architecture::Wasm32 = target.architecture {
            build.include(wasm_include_dir());
            if let OperatingSystem::Emscripten = target.operating_system {
                build.define("MRB_API", Some(r#"__attribute__((used))"#));
            } else if let OperatingSystem::Unknown = target.operating_system {
                build.define("MRB_API", Some(r#"__attribute__((visibility("default")))"#));
                build.define("MRB_DISABLE_DIRECT_THREADING", None);
            }
        }

        build.compile("libartichokemruby.a");
    }

    fn bindgen(target: &Triple, mrb_int: &str) {
        println!(
            "cargo:rerun-if-changed={}",
            bindgen_source_header().to_str().unwrap()
        );
        let bindings_out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ffi.rs");
        let mut bindgen = bindgen::Builder::default()
            .header(bindgen_source_header().to_str().unwrap())
            .clang_arg(format!("-I{}", mruby_include_dir().to_str().unwrap()))
            .clang_arg(format!("-I{}", ext_include_dir().to_str().unwrap()))
            .clang_arg("-DMRB_DISABLE_STDIO")
            .clang_arg("-DMRB_UTF8_STRING")
            .clang_arg(format!("-D{}", mrb_int))
            .whitelist_function("^mrb.*")
            .whitelist_type("^mrb.*")
            .whitelist_var("^mrb.*")
            .whitelist_var("^MRB.*")
            .whitelist_var("^MRUBY.*")
            .whitelist_var("REGEXP_CLASS")
            .rustified_enum("mrb_vtype")
            .rustified_enum("mrb_lex_state_enum")
            .rustified_enum("mrb_range_beg_len")
            .rustfmt_bindings(true)
            // work around warnings caused by cargo doc interpreting Ruby doc blocks
            // as Rust code.
            // See: https://github.com/rust-lang/rust-bindgen/issues/426
            .generate_comments(false);
        if let Architecture::Wasm32 = target.architecture {
            bindgen = bindgen
                .clang_arg(format!("-I{}", wasm_include_dir().to_str().unwrap()))
                .clang_arg(r#"-DMRB_API=__attribute__((visibility("default")))"#);
        }
        if env::var("CARGO_FEATURE_ARTICHOKE_ARRAY").is_ok() {
            bindgen = bindgen.clang_arg("-DARTICHOKE");
        }

        bindgen
            .generate()
            .unwrap()
            .write_to_file(bindings_out_path)
            .unwrap();
    }

    pub fn build() {
        fs::create_dir_all(mruby_build_dir()).unwrap();
        let target = Triple::from_str(env::var("TARGET").unwrap().as_str()).unwrap();
        let mrb_int = if let Architecture::Wasm32 = target.architecture {
            "MRB_INT32"
        } else {
            "MRB_INT64"
        };
        staticlib(&target, mrb_int);
        bindgen(&target, mrb_int);
    }
}

mod rubylib {
    use rayon::prelude::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    use super::buildpath;

    pub fn generated_package_dir() -> PathBuf {
        PathBuf::from(env::var_os("OUT_DIR").unwrap())
            .join("src")
            .join("generated")
    }

    fn generated_package(package: &str) -> PathBuf {
        generated_package_dir().join(format!("{}.rs", package))
    }

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
                let out = generated_package_dir().join(package_source.unwrap());
                if let Some(parent) = out.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::copy(source, &out).unwrap();
            });
            generate_rust_glue(package, sources);
        });
    }

    fn package_files(package: &str) -> String {
        let script = buildpath::crate_root()
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
        let pkg_dest = generated_package(&package);
        if let Some(parent) = pkg_dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let script = buildpath::crate_root()
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

mod build {
    use fs_extra::{dir, file};
    use std::fs;

    use super::{buildpath, libmruby, rubylib};

    pub fn clean() {
        let _ = dir::remove(buildpath::build_root());
    }

    pub fn setup_build_root() {
        fs::create_dir_all(buildpath::build_root()).unwrap();

        let opts = dir::CopyOptions::new();
        dir::copy(
            buildpath::crate_root().join("vendor").join("mruby"),
            buildpath::build_root(),
            &opts,
        )
        .unwrap();
        dir::copy(
            buildpath::crate_root().join("vendor").join("ruby"),
            buildpath::build_root(),
            &opts,
        )
        .unwrap();

        fs::create_dir_all(rubylib::generated_package_dir()).unwrap();

        let opts = file::CopyOptions::new();
        let _ = file::remove(libmruby::build_config());
        file::copy(
            buildpath::source::mruby_build_config(),
            libmruby::build_config(),
            &opts,
        )
        .unwrap();
        file::copy(
            buildpath::source::mruby_bootstrap_gembox(),
            libmruby::bootstrap_gembox(),
            &opts,
        )
        .unwrap();
    }
}

fn main() {
    build::clean();
    build::setup_build_root();
    libmruby::build();
    rubylib::build();
    release::build();
}
