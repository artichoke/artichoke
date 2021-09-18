#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::restriction)]

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use target_lexicon::Triple;

fn enumerate_sources(path: PathBuf, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut stack = vec![path.clone()];
    paths.push(path);
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
        let root = env::var_os("CARGO_MANIFEST_DIR").expect("cargo-provided CARGO_MANIFEST_DIR env variable not set");
        PathBuf::from(root)
    }

    pub fn build_root() -> PathBuf {
        let out = env::var_os("OUT_DIR").expect("cargo-provided OUT_DIR env variable not set");
        PathBuf::from(out).join("artichoke-mruby")
    }

    pub mod source {
        use std::path::PathBuf;

        pub fn rerun_if_changed(paths: &mut Vec<PathBuf>) {
            paths.push(mruby_build_config());
            paths.push(mruby_bootstrap_gembox());
            paths.push(mruby_bootstrap_gembox());
            paths.push(mruby_noop());

            crate::enumerate_sources(mruby_vendored_include_dir(), paths).unwrap();
            crate::enumerate_sources(mruby_vendored_source_dir(), paths).unwrap();
            crate::enumerate_sources(mruby_sys_ext_include_dir(), paths).unwrap();
            crate::enumerate_sources(mruby_sys_ext_source_dir(), paths).unwrap();
        }

        fn mruby_vendored_include_dir() -> PathBuf {
            super::crate_root().join("vendor").join("mruby").join("include")
        }

        fn mruby_vendored_source_dir() -> PathBuf {
            super::crate_root().join("vendor").join("mruby").join("src")
        }

        pub fn mruby_build_config() -> PathBuf {
            super::crate_root().join("mruby_build_config_null.rb")
        }

        pub fn mruby_bootstrap_gembox() -> PathBuf {
            super::crate_root().join("bootstrap.gembox")
        }

        pub fn mruby_noop() -> PathBuf {
            super::crate_root().join("scripts").join("noop.rb")
        }

        fn mruby_sys_ext_source_dir() -> PathBuf {
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
    use std::process::{Command, ExitStatus, Stdio};
    use std::str;

    use target_lexicon::{Architecture, OperatingSystem, Triple};

    use super::{buildpath, enumerate_sources};

    fn gems() -> impl Iterator<Item = &'static str> {
        vec![
            "mruby-compiler",     // Ruby parser and bytecode generation
            "mruby-error",        // `mrb_raise`, `mrb_protect`
            "mruby-eval",         // eval, instance_eval, and friends
            "mruby-metaprog",     // APIs on Kernel and Module for accessing classes and variables
            "mruby-method",       // `Method`, `UnboundMethod`, and method APIs on Kernel and Module
            "mruby-toplevel-ext", // expose API for top self
            "mruby-fiber",        // Fiber class from core, required by `Enumerator`
            "mruby-pack",         // Array#pack and String#unpack
            "mruby-sprintf",      // Kernel#sprintf, Kernel#format, String#%
            "mruby-class-ext",    // NOTE(GH-32): Pending removal.
            "mruby-proc-ext",     // NOTE(GH-32): This gem is required by `mruby-method`.
        ]
        .into_iter()
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

    // From `emsdk/upstream/emscripten/tools/shared.py:emsdk_cflags`:
    //
    // ``python
    // path_from_root('system', 'include', 'compat'),
    // path_from_root('system', 'include'),
    // path_from_root('system', 'include', 'libc'),
    // path_from_root('system', 'lib', 'libc', 'musl', 'arch', 'emscripten'),
    // path_from_root('system', 'local', 'include'),
    // path_from_root('system', 'include', 'SSE'),
    // path_from_root('system', 'include', 'neon'),
    // path_from_root('system', 'lib', 'compiler-rt', 'include'),
    // path_from_root('system', 'lib', 'libunwind', 'include'),
    // ```
    fn wasm_include_dirs() -> impl Iterator<Item = PathBuf> {
        let system = buildpath::crate_root().join("vendor").join("emscripten").join("system");
        vec![
            system.join("include").join("compat"),
            system.join("include"),
            system.join("include").join("libc"),
            system
                .join("lib")
                .join("libc")
                .join("musl")
                .join("arch")
                .join("emscripten"),
            system.join("local").join("include"),
            system.join("include").join("SSE"),
            system.join("include").join("neon"),
            system.join("lib").join("compiler-rt").join("include"),
            system.join("lib").join("libunwind").join("include"),
        ]
        .into_iter()
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
        buildpath::source::mruby_sys_ext_include_dir().join("mruby-sys.h")
    }

    fn generate_mrbgem_config() {
        let mut gembox = String::from("MRuby::GemBox.new do |conf|\n");
        for gem in gems() {
            gembox.push_str("  conf.gem core: '");
            gembox.push_str(gem);
            gembox.push_str("'\n");
        }
        gembox.push_str("end\n");
        fs::write(mruby_generated_gembox(), gembox).unwrap();
    }

    /// Build the mruby static library with its built in minirake build system.
    fn staticlib(target: &Triple) {
        // minirake dynamically generates some c source files so we can't build
        // directly with the `cc` crate. We must first hijack the mruby build
        // system to do the codegen for us.
        generate_mrbgem_config();
        let status = Command::new("ruby")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg(mruby_minirake())
            .arg("--verbose")
            .env("MRUBY_BUILD_DIR", mruby_build_dir())
            .env("MRUBY_CONFIG", mruby_build_config())
            .current_dir(mruby_source_dir())
            .status()
            .unwrap();
        if !status.success() {
            panic!("minirake failed");
        }

        let mut sources = HashMap::new();
        sources.insert(
            buildpath::source::mruby_sys_ext_source_file(),
            buildpath::source::mruby_sys_ext_source_file(),
        );

        let mut mruby_sources = vec![];
        enumerate_sources(mruby_source_dir(), &mut mruby_sources).unwrap();

        for source in mruby_sources {
            let relative_source = source.strip_prefix(mruby_source_dir()).unwrap();
            let is_core_source = source.strip_prefix(mruby_source_dir().join("src")).is_ok();
            let is_required_gem_source = gems().any(|gem| {
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
                    sources.insert(relative_source.to_owned(), source.clone());
                }
            }
        }

        let mut mruby_codegen_sources = vec![];
        crate::enumerate_sources(mruby_generated_source_dir(), &mut mruby_codegen_sources).unwrap();
        for source in mruby_codegen_sources {
            let relative_source = source.strip_prefix(mruby_generated_source_dir()).unwrap();
            let is_test_source = relative_source
                .components()
                .any(|component| component.as_os_str() == OsStr::new("test"));
            if is_test_source {
                continue;
            }
            if source.extension().and_then(OsStr::to_str) == Some("c") {
                sources.insert(relative_source.to_owned(), source.clone());
            }
        }
        // Build the extension library
        let mut build = cc::Build::new();
        build
            .warnings(false)
            .files(sources.values())
            .include(mruby_include_dir())
            .include(buildpath::source::mruby_sys_ext_include_dir())
            .define("ARTICHOKE", None)
            .define("MRB_ARY_NO_EMBED", None)
            .define("MRB_GC_TURN_OFF_GENERATIONAL", None)
            .define("MRB_INT64", None)
            .define("MRB_NO_BOXING", None)
            .define("MRB_NO_PRESYM", None)
            .define("MRB_NO_STDIO", None)
            .define("MRB_UTF8_STRING", None);

        for gem in gems() {
            let dir = if gem == "mruby-compiler" { "core" } else { "include" };
            let gem_include_dir = mruby_source_dir().join("mrbgems").join(gem).join(dir);
            build.include(gem_include_dir);
        }

        if let Architecture::Wasm32 = target.architecture {
            for include_dir in wasm_include_dirs() {
                build.include(include_dir);
            }
            match target.operating_system {
                OperatingSystem::Emscripten => {
                    build.define("MRB_API", Some(r#"__attribute__((used))"#));
                }
                OperatingSystem::Unknown => {
                    build.define("MRB_API", Some(r#"__attribute__((visibility("default")))"#));
                    build.define("MRB_NO_DIRECT_THREADING", None);
                }
                _ => {}
            }
        }

        build.compile("libartichokemruby.a");
    }

    fn bindgen(target: &Triple, out_dir: &OsStr) {
        // Try to use an existing global install of bindgen
        let status = invoke_bindgen(target, out_dir, OsStr::new("bindgen"));
        if matches!(status, Some(status) if status.success()) {
            return;
        }
        // Install bindgen
        // cargo install --root target/bindgen --version 0.59.1 bindgen
        let bindgen_install_dir = PathBuf::from(out_dir).join("bindgen");
        let status = Command::new(env::var_os("CARGO").unwrap())
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("install")
            .arg("--root")
            .arg(&bindgen_install_dir)
            .arg("--version")
            .arg("0.59.1")
            .arg("bindgen")
            .status()
            .unwrap();
        if !status.success() {
            panic!("cargo install bindgen failed");
        }
        let status = invoke_bindgen(
            target,
            out_dir,
            bindgen_install_dir.join("bin").join("bindgen").as_os_str(),
        );
        if !status.unwrap().success() {
            panic!("bindgen failed");
        }
    }

    pub fn invoke_bindgen(target: &Triple, out_dir: &OsStr, bindgen_executable: &OsStr) -> Option<ExitStatus> {
        let bindings_out_path = PathBuf::from(out_dir).join("ffi.rs");
        let mut command = Command::new(bindgen_executable);
        command
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        command
            .arg("--allowlist-function")
            .arg("^mrb.*")
            .arg("--allowlist-type")
            .arg("^mrb.*")
            .arg("--allowlist-var")
            .arg("^mrb.*")
            .arg("--allowlist-var")
            .arg("^MRB.*")
            .arg("--allowlist-var")
            .arg("^MRUBY.*")
            .arg("--allowlist-var")
            .arg("REGEXP_CLASS")
            .arg("--rustified-enum")
            .arg("mrb_vtype")
            .arg("--rustified-enum")
            .arg("mrb_lex_state_enum")
            .arg("--rustified-enum")
            .arg("mrb_range_beg_len")
            .arg("--no-doc-comments")
            .arg("--size_t-is-usize")
            .arg("--output")
            .arg(bindings_out_path)
            .arg(bindgen_source_header())
            .arg("--")
            .arg("-I")
            .arg(mruby_include_dir())
            .arg("-I")
            .arg(buildpath::source::mruby_sys_ext_include_dir())
            .arg("-DARTICHOKE")
            .arg("-DMRB_ARY_NO_EMBED")
            .arg("-DMRB_GC_TURN_OFF_GENERATIONAL")
            .arg("-DMRB_INT64")
            .arg("-DMRB_NO_BOXING")
            .arg("-DMRB_NO_PRESYM")
            .arg("-DMRB_NO_STDIO")
            .arg("-DMRB_UTF8_STRING");

        if let Architecture::Wasm32 = target.architecture {
            for include_dir in wasm_include_dirs() {
                command.arg("-I").arg(include_dir);
            }
            command.arg(r#"-DMRB_API=__attribute__((visibility("default")))"#);
        }

        command.status().ok()
    }

    pub fn build(target: &Triple, out_dir: &OsStr) {
        fs::create_dir_all(mruby_build_dir()).unwrap();
        staticlib(target);
        bindgen(target, out_dir);
    }
}

mod build {
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    use super::{buildpath, libmruby};

    pub fn clean() {
        let _ignored = fs::remove_dir_all(buildpath::build_root());
    }

    pub fn setup_build_root() {
        fs::create_dir_all(buildpath::build_root()).unwrap();

        copy_dir_recursive(
            &buildpath::crate_root().join("vendor").join("mruby"),
            &libmruby::mruby_source_dir(),
        )
        .unwrap();

        let _ignored = fs::remove_file(libmruby::mruby_build_config());
        fs::create_dir_all(libmruby::mruby_build_dir()).unwrap();
        fs::copy(buildpath::source::mruby_build_config(), libmruby::mruby_build_config()).unwrap();
        fs::copy(
            buildpath::source::mruby_bootstrap_gembox(),
            libmruby::bootstrap_gembox(),
        )
        .unwrap();
        fs::copy(buildpath::source::mruby_noop(), libmruby::builder_noop()).unwrap();
    }

    pub fn rerun_if_changed() {
        let mut paths = vec![];
        buildpath::source::rerun_if_changed(&mut paths);

        for path in paths {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
        }
    }

    fn copy_dir_recursive(from: &Path, to: &Path) -> io::Result<()> {
        let mut stack = vec![from.to_owned()];
        let dest_root = to.to_owned();
        let input_root_depth = from.components().count();
        println!("copying {} -> {}", from.display(), to.display());

        while let Some(from) = stack.pop() {
            let dest = from.components().skip(input_root_depth);
            let dest = dest_root.join(dest.collect::<PathBuf>());
            let _ignored = fs::create_dir_all(&dest);

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
    let target = env::var_os("TARGET").expect("cargo-provided TARGET env variable not set");
    let target = target.to_str().expect("TARGET env variable was not valid UTF-8");
    let target = target.parse::<Triple>().unwrap_or_else(|err| {
        panic!(
            "target-lexicon could not parse build target '{}' with error: {}",
            target, err
        )
    });
    let out_dir = env::var_os("OUT_DIR").expect("cargo-provided OUT_DIR env variable not set");
    build::clean();
    build::rerun_if_changed();
    build::setup_build_root();
    libmruby::build(&target, &out_dir);
}
