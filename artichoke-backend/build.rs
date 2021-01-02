#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::restriction)]

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use target_lexicon::Triple;

fn enumerate_sources<T>(path: T, paths: &mut Vec<PathBuf>) -> io::Result<()>
where
    T: AsRef<Path>,
{
    let mut stack = vec![PathBuf::from(path.as_ref())];
    paths.push(PathBuf::from(path.as_ref()));
    while let Some(from) = stack.pop() {
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            paths.push(path);
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
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("artichoke-mruby")
    }

    pub mod source {
        use std::path::PathBuf;

        pub fn rerun_if_changed(paths: &mut Vec<PathBuf>) {
            crate::enumerate_sources(mruby_vendored_include_dir(), paths).unwrap();
            crate::enumerate_sources(mruby_vendored_source_dir(), paths).unwrap();
            crate::enumerate_sources(mruby_sys_ext_include_dir(), paths).unwrap();
            crate::enumerate_sources(mruby_sys_ext_source_dir(), paths).unwrap();
        }

        pub fn mruby_vendored_include_dir() -> PathBuf {
            super::crate_root().join("vendor").join("mruby").join("include")
        }

        pub fn mruby_vendored_source_dir() -> PathBuf {
            super::crate_root().join("vendor").join("mruby").join("src")
        }

        pub fn mruby_sys_ext_source_dir() -> PathBuf {
            super::crate_root().join("mruby-sys")
        }

        pub fn mruby_sys_ext_include_dir() -> PathBuf {
            mruby_sys_ext_source_dir().join("include")
        }

        pub fn mruby_sys_ext_source_file() -> PathBuf {
            mruby_sys_ext_source_dir().join("src").join("mruby-sys").join("ext.c")
        }
    }
}

mod libmruby {
    use std::collections::HashMap;
    use std::env;
    use std::ffi::OsStr;
    use std::fs;
    use std::path::PathBuf;
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

    fn mruby_include_dir() -> PathBuf {
        mruby_source_dir().join("include")
    }

    pub fn mruby_build_dir() -> PathBuf {
        buildpath::build_root().join("mruby-build")
    }

    fn bindgen_source_header() -> PathBuf {
        buildpath::source::mruby_sys_ext_include_dir().join("mruby-sys.h")
    }

    /// Build the mruby static library with its built in minirake build system.
    fn staticlib(target: &Triple, mrb_int: &str) {
        let mut sources = HashMap::new();
        sources.insert(
            buildpath::source::mruby_sys_ext_source_file(),
            buildpath::source::mruby_sys_ext_source_file(),
        );
        let mut mruby_sources = vec![];
        crate::enumerate_sources(mruby_source_dir(), &mut mruby_sources).unwrap();
        for source in mruby_sources {
            let relative_source = source.strip_prefix(mruby_source_dir()).unwrap();
            let is_core_source = source.strip_prefix(mruby_source_dir().join("src")).is_ok();
            let is_required_gem_source = gems().iter().any(|gem| {
                source
                    .components()
                    .any(|component| component.as_os_str() == OsStr::new(gem))
            });
            if is_core_source || is_required_gem_source {
                let is_build_source = relative_source
                    .components()
                    .any(|component| component.as_os_str() == OsStr::new("build"));
                let is_test_source = relative_source
                    .components()
                    .any(|component| component.as_os_str() == OsStr::new("test"));
                if is_build_source || is_test_source {
                    continue;
                }
                if source.extension().and_then(OsStr::to_str) == Some("c") {
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
            .include(buildpath::source::mruby_sys_ext_include_dir())
            .define("MRB_DISABLE_STDIO", None)
            .define("MRB_UTF8_STRING", None)
            .define("MRB_ARY_NO_EMBED", None)
            .define("MRB_NO_BOXING", None)
            .define(mrb_int, None)
            .define("DISABLE_GEMS", None)
            .define("ARTICHOKE", None);

        for gem in gems() {
            let dir = if gem == "mruby-compiler" { "core" } else { "include" };
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
            .clang_arg(format!(
                "-I{}",
                buildpath::source::mruby_sys_ext_include_dir()
                    .to_str()
                    .unwrap()
            ))
            .clang_arg("-DMRB_DISABLE_STDIO")
            .clang_arg("-DMRB_UTF8_STRING")
            .clang_arg("-DMRB_ARY_NO_EMBED")
            .clang_arg("-DMRB_NO_BOXING")
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

        bindgen.generate().unwrap().write_to_file(bindings_out_path).unwrap();
    }

    pub fn build(target: &Triple) {
        fs::create_dir_all(mruby_build_dir()).unwrap();
        let mrb_int = "MRB_INT64";
        staticlib(target, mrb_int);
        bindgen(target, mrb_int);
    }
}

mod build {
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    use super::{buildpath, libmruby};

    pub fn clean() {
        let _ = fs::remove_dir_all(buildpath::build_root());
    }

    pub fn setup_build_root() {
        fs::create_dir_all(buildpath::build_root()).unwrap();

        copy_dir_recursive(
            buildpath::crate_root().join("vendor").join("mruby"),
            libmruby::mruby_source_dir(),
        )
        .unwrap();

        fs::create_dir_all(libmruby::mruby_build_dir()).unwrap();
    }

    pub fn rerun_if_changed() {
        let mut paths = vec![];
        buildpath::source::rerun_if_changed(&mut paths);

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
        println!("copying {} -> {}", from.as_ref().display(), to.as_ref().display());

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
                    fs::copy(&path, &dest)?;
                } else {
                    eprintln!("failed to copy: {}", path.display());
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let target = env::var_os("TARGET").unwrap();
    let target = Triple::from_str(target.to_str().unwrap()).unwrap();
    build::clean();
    build::rerun_if_changed();
    build::setup_build_root();
    libmruby::build(&target);
}
