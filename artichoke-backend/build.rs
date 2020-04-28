#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::restriction)]

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use target_lexicon::Triple;

fn enumerate_sources<T>(path: T, into: &mut Vec<PathBuf>) -> io::Result<()>
where
    T: AsRef<Path>,
{
    let mut stack = vec![PathBuf::from(path.as_ref())];
    into.push(PathBuf::from(path.as_ref()));
    while let Some(from) = stack.pop() {
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            into.push(path);
        }
    }
    Ok(())
}

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
        use target_lexicon::Triple;

        pub fn rerun_if_changed(target: &Triple, into: &mut Vec<PathBuf>) {
            into.push(mruby_build_config(target));
            into.push(mruby_bootstrap_gembox());
            into.push(mruby_bootstrap_gembox());
            into.push(mruby_noop());

            crate::enumerate_sources(mruby_vendored_include_dir(), into).unwrap();
            crate::enumerate_sources(mruby_vendored_source_dir(), into).unwrap();
        }

        pub fn mruby_vendored_include_dir() -> PathBuf {
            super::crate_root()
                .join("vendor")
                .join("mruby")
                .join("include")
        }

        pub fn mruby_vendored_source_dir() -> PathBuf {
            super::crate_root().join("vendor").join("mruby").join("src")
        }

        pub fn mruby_build_config(target: &Triple) -> PathBuf {
            let _ = target;
            super::crate_root().join("mruby_build_config_null.rb")
        }

        pub fn mruby_bootstrap_gembox() -> PathBuf {
            super::crate_root().join("bootstrap.gembox")
        }

        pub fn mruby_noop() -> PathBuf {
            super::crate_root().join("scripts").join("noop.rb")
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
    use std::str;
    use target_lexicon::{Architecture, OperatingSystem, Triple};

    use super::buildpath;

    fn gems() -> Vec<&'static str> {
        vec![
            "mruby-compiler",     // Ruby parser and bytecode generation
            "mruby-error",        // `mrb_raise`, `mrb_protect`
            "mruby-eval",         // eval, instance_eval, and friends
            "mruby-metaprog",     // APIs on Kernel and Module for accessing classes and variables
            "mruby-method",       // `Method`, `UnboundMethod`, and method APIs on Kernel and Module
            "mruby-toplevel-ext", // expose API for top self
            "mruby-fiber",        // Fiber class from core, required by mruby-enumerator
            "mruby-pack",         // Array#pack and String#unpack
            "mruby-sprintf",      // Kernel#sprintf, Kernel#format, String#%
            "mruby-class-ext",    // NOTE(GH-32): Pending removal.
            "mruby-proc-ext",     // NOTE(GH-32): This gem is required by `mruby-method`.
        ]
    }

    pub fn mruby_build_config() -> PathBuf {
        mruby_source_dir().join("build_config.rb")
    }

    pub fn bootstrap_gembox() -> PathBuf {
        mruby_source_dir().join("bootstrap.gembox")
    }

    pub fn builder_noop() -> PathBuf {
        mruby_source_dir().join("noop.rb")
    }

    pub fn ext_source_dir() -> PathBuf {
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

    pub fn mruby_source_dir() -> PathBuf {
        buildpath::build_root().join("mruby")
    }

    fn mruby_minirake() -> PathBuf {
        mruby_source_dir().join("minirake")
    }

    fn mruby_include_dir() -> PathBuf {
        mruby_source_dir().join("include")
    }

    pub fn mruby_build_dir() -> PathBuf {
        buildpath::build_root().join("mruby-build")
    }

    fn mruby_generated_source_dir() -> PathBuf {
        mruby_build_dir().join("sys")
    }

    fn mruby_generated_gembox() -> PathBuf {
        mruby_source_dir().join("sys.gembox")
    }

    fn bindgen_source_header() -> PathBuf {
        ext_include_dir().join("mruby-sys.h")
    }

    fn generate_mrbgem_config() {
        let mut gembox = String::from("MRuby::GemBox.new { |conf| ");
        for gem in gems() {
            gembox.push_str("conf.gem core: '");
            gembox.push_str(gem);
            gembox.push_str("';");
        }
        gembox.push('}');
        fs::write(mruby_generated_gembox(), gembox).unwrap();
    }

    /// Build the mruby static library with its built in minirake build system.
    fn staticlib(target: &Triple, mrb_int: &str) {
        // minirake dynamically generates some c source files so we can't build
        // directly with the `cc` crate. We must first hijack the mruby build
        // system to do the codegen for us.
        generate_mrbgem_config();
        let status = Command::new("ruby")
            .arg(mruby_minirake())
            .arg("--verbose")
            .arg("--jobs")
            .arg(num_cpus::get().to_string())
            .env("MRUBY_BUILD_DIR", mruby_build_dir())
            .env("MRUBY_CONFIG", mruby_build_config())
            .current_dir(mruby_source_dir())
            .status()
            .unwrap();
        if !status.success() {
            panic!("minirake failed");
        }

        let mut sources = HashMap::new();
        sources.insert(ext_source_file(), ext_source_file());
        let mut mruby_sources = vec![];
        crate::enumerate_sources(mruby_source_dir(), &mut mruby_sources).unwrap();
        for source in mruby_sources {
            let relative_source = source.strip_prefix(mruby_source_dir()).unwrap();
            let is_core_source = source.strip_prefix(mruby_source_dir().join("src")).is_ok();
            let is_required_gem_source = gems().iter().any(|gem| {
                source
                    .components()
                    .any(|component| component == Component::Normal(OsStr::new(gem)))
            });
            if is_core_source || is_required_gem_source {
                let is_build_source = relative_source
                    .components()
                    .any(|component| component == Component::Normal(OsStr::new("build")));
                let is_test_source = relative_source
                    .components()
                    .any(|component| component == Component::Normal(OsStr::new("test")));
                if is_build_source || is_test_source {
                    continue;
                }
                if source.extension().and_then(OsStr::to_str) == Some("c") {
                    sources.insert(relative_source.to_owned(), source.to_owned());
                }
            }
        }
        let mut mruby_codegen_sources = vec![];
        crate::enumerate_sources(mruby_generated_source_dir(), &mut mruby_codegen_sources).unwrap();
        for source in mruby_codegen_sources {
            let relative_source = source.strip_prefix(mruby_generated_source_dir()).unwrap();
            let is_test_source = relative_source
                .components()
                .any(|component| component == Component::Normal(OsStr::new("test")));
            if is_test_source {
                continue;
            }
            if source.extension().and_then(OsStr::to_str) == Some("c") {
                sources.insert(relative_source.to_owned(), source.to_owned());
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
            .define("DISABLE_GEMS", None)
            .define("ARTICHOKE", None);

        for gem in gems() {
            let dir = if gem == "mruby-compiler" {
                "core"
            } else {
                "include"
            };
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
            .generate_comments(false)
            .size_t_is_usize(true);
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

    pub fn build(target: &Triple) {
        fs::create_dir_all(mruby_build_dir()).unwrap();
        let mrb_int = "MRB_INT64";
        staticlib(target, mrb_int);
        bindgen(target, mrb_int);
    }
}

mod release {
    use chrono::prelude::*;
    use git2::Repository;
    use std::env;
    use std::fmt;
    use std::str;
    use target_lexicon::Triple;

    pub fn build(target: &Triple) {
        let version = env::var("CARGO_PKG_VERSION").unwrap();
        let repo = Repository::open(crate::buildpath::crate_root().parent().unwrap()).ok();
        let birth_date = birthdate(repo.as_ref());
        let build_date = Utc::now();
        let release_date = Utc.timestamp(build_date.timestamp(), 0).date();
        let revision_count = revision_count(repo.as_ref());
        let platform = platform(target);
        let copyright = copyright(birth_date, build_date);
        let description = description(
            version.as_str(),
            release_date,
            revision_count,
            platform.as_str(),
        );

        emit("RUBY_RELEASE_DATE", release_date.naive_utc());
        emit("RUBY_RELEASE_YEAR", build_date.year());
        emit("RUBY_RELEASE_MONTH", build_date.month());
        emit("RUBY_RELEASE_DAY", build_date.day());
        if let Some(revision_count) = revision_count {
            emit("RUBY_REVISION", revision_count);
        }
        emit("RUBY_PLATFORM", platform);
        emit("RUBY_COPYRIGHT", copyright);
        emit("RUBY_DESCRIPTION", description);
        if let Some(compiler_version) = compiler_version() {
            emit("ARTICHOKE_COMPILER_VERSION", compiler_version);
        }
    }

    fn emit<T>(env: &str, value: T)
    where
        T: fmt::Display,
    {
        println!("cargo:rustc-env={}={}", env, value);
    }

    fn birthdate(repo: Option<&Repository>) -> Option<DateTime<Utc>> {
        let repo = repo?;
        let mut revwalk = repo.revwalk().ok()?;
        revwalk.push_head().ok()?;
        revwalk
            .set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)
            .ok()?;
        let rev = revwalk.next()?.ok()?;
        let commit = repo.find_commit(rev).ok()?;
        let time = commit.time().seconds();
        Some(Utc.timestamp(time, 0))
    }

    fn revision_count(repo: Option<&Repository>) -> Option<usize> {
        let repo = repo?;
        let mut revwalk = repo.revwalk().ok()?;
        revwalk.push_head().ok();
        Some(revwalk.count())
    }

    fn platform(target: &Triple) -> String {
        format!("{}-{}", target.architecture, target.operating_system)
    }

    fn copyright(birth_date: Option<DateTime<Utc>>, build_date: DateTime<Utc>) -> String {
        match birth_date {
            Some(birth) if birth.year() == build_date.year() => format!(
                "Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
                birth.year()
            ),
            Some(birth) => format!(
                "Copyright (c) {}-{} Ryan Lopopolo <rjl@hyperbo.la>",
                birth.year(),
                build_date.year()
            ),
            None => format!(
                "Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
                build_date.year()
            ),
        }
    }

    fn description(
        version: &str,
        release_date: Date<Utc>,
        revision_count: Option<usize>,
        platform: &str,
    ) -> String {
        if let Some(revision_count) = revision_count {
            format!(
                "artichoke {} ({} revision {}) [{}]",
                version,
                release_date.naive_utc(),
                revision_count,
                platform
            )
        } else {
            format!(
                "artichoke {} ({}) [{}]",
                version,
                release_date.naive_utc(),
                platform
            )
        }
    }

    fn compiler_version() -> Option<String> {
        let metadata = rustc_version::version_meta().ok()?;
        let compiler_version = if let Some(mut commit) = metadata.commit_hash {
            commit.truncate(7);
            format!(
                "Rust {} (rev {}) on {}",
                metadata.semver, commit, metadata.host
            )
        } else {
            format!("Rust {} on {}", metadata.semver, metadata.host)
        };
        Some(compiler_version)
    }
}

mod build {
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};
    use target_lexicon::Triple;

    use super::{buildpath, libmruby};

    pub fn clean() {
        let _ = fs::remove_dir_all(buildpath::build_root());
    }

    pub fn setup_build_root(target: &Triple) {
        fs::create_dir_all(buildpath::build_root()).unwrap();

        copy_dir_recursive(
            buildpath::crate_root().join("vendor").join("mruby"),
            libmruby::mruby_source_dir(),
        )
        .unwrap();

        let _ = fs::remove_file(libmruby::mruby_build_config());
        fs::create_dir_all(libmruby::mruby_build_dir()).unwrap();
        fs::copy(
            buildpath::source::mruby_build_config(target),
            libmruby::mruby_build_config(),
        )
        .unwrap();
        fs::copy(
            buildpath::source::mruby_bootstrap_gembox(),
            libmruby::bootstrap_gembox(),
        )
        .unwrap();
        fs::copy(buildpath::source::mruby_noop(), libmruby::builder_noop()).unwrap();
    }

    pub fn rerun_if_changed(target: &Triple) {
        let mut paths = vec![];
        buildpath::source::rerun_if_changed(target, &mut paths);
        crate::enumerate_sources(libmruby::ext_source_dir(), &mut paths).unwrap();

        for path in paths {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
        }
    }

    fn copy_dir_recursive<T, U>(from: T, to: U) -> io::Result<()>
    where
        T: AsRef<Path>,
        U: AsRef<Path>,
    {
        let mut stack = vec![PathBuf::from(from.as_ref())];
        let dest_root = PathBuf::from(to.as_ref());
        let input_root_depth = from.as_ref().components().count();
        println!("copying {:?} -> {:?}", from.as_ref(), to.as_ref());

        while let Some(from) = stack.pop() {
            let dest = from.components().skip(input_root_depth);
            let dest = dest_root.join(dest.collect::<PathBuf>());
            let _ = fs::create_dir_all(&dest);

            for entry in fs::read_dir(from)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if let Some(filename) = path.file_name() {
                    let dest = dest.as_path().join(filename);
                    println!("  copy: {:?} -> {:?}", path, dest);
                    fs::copy(&path, &dest)?;
                } else {
                    eprintln!("failed: {:?}", path);
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let target = Triple::from_str(env::var("TARGET").unwrap().as_str()).unwrap();
    build::clean();
    build::rerun_if_changed(&target);
    build::setup_build_root(&target);
    libmruby::build(&target);
    release::build(&target);
}
