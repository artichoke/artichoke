#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::restriction)]

use std::env;

use target_lexicon::Triple;

mod paths {
    use std::env;
    use std::path::PathBuf;

    pub fn crate_root() -> PathBuf {
        let root = env::var_os("CARGO_MANIFEST_DIR").expect("cargo-provided CARGO_MANIFEST_DIR env variable not set");
        PathBuf::from(root)
    }

    pub fn mruby_root() -> PathBuf {
        crate_root().join("vendor").join("mruby")
    }

    pub fn mrbgems_root() -> PathBuf {
        crate_root().join("cext").join("mrbgems")
    }

    pub fn mrbsys_root() -> PathBuf {
        crate_root().join("cext").join("mrbsys")
    }

    pub fn emscripten_root() -> PathBuf {
        crate_root().join("vendor").join("emscripten")
    }

    pub fn bindgen_header() -> PathBuf {
        crate_root().join("cext").join("bindgen.h")
    }
}

mod libs {
    use std::env;
    use std::ffi::OsStr;
    use std::path::PathBuf;
    use std::process::{Command, ExitStatus, Stdio};
    use std::str;

    use target_lexicon::{Architecture, OperatingSystem, Triple};

    use super::paths;

    fn mruby_sources() -> impl Iterator<Item = PathBuf> {
        [
            "src/array.c",
            "src/backtrace.c",
            "src/class.c",
            "src/codedump.c",
            "src/compar.c",
            "src/debug.c",
            "src/dump.c",
            "src/enum.c",
            "src/error.c",
            "src/etc.c",
            "src/fmt_fp.c",
            "src/gc.c",
            "src/hash.c",
            "src/init.c",
            "src/kernel.c",
            "src/load.c",
            "src/numeric.c",
            "src/object.c",
            "src/pool.c",
            "src/print.c",
            "src/proc.c",
            "src/range.c",
            "src/state.c",
            "src/string.c",
            "src/symbol.c",
            "src/variable.c",
            "src/version.c",
            "src/vm.c",
        ]
        .into_iter()
        .map(|source| paths::mruby_root().join(source))
    }

    fn mruby_include_dirs() -> impl Iterator<Item = PathBuf> {
        [
            "include", // mruby core
        ]
        .into_iter()
        .map(|dir| paths::mruby_root().join(dir))
    }

    fn mrbgems_sources() -> impl Iterator<Item = PathBuf> {
        [
            "mrbgems/mruby-class-ext/src/class.c",   // NOTE(GH-32): Pending removal.
            "mrbgems/mruby-compiler/core/codegen.c", // Ruby parser and bytecode generation
            "mrbgems/mruby-compiler/core/y.tab.c",   // Ruby parser and bytecode generation
            "mrbgems/mruby-error/src/exception.c",   // `mrb_raise`, `mrb_protect`
            "mrbgems/mruby-eval/src/eval.c",         // eval, instance_eval, and friends
            "mrbgems/mruby-fiber/src/fiber.c",       // Fiber class from core, required by `Enumerator`
            "mrbgems/mruby-metaprog/src/metaprog.c", // APIs on Kernel and Module for accessing classes and variables
            "mrbgems/mruby-method/src/method.c",     // `Method`, `UnboundMethod`, and method APIs on Kernel and Module
            "mrbgems/mruby-pack/src/pack.c",         // Array#pack and String#unpack
            "mrbgems/mruby-proc-ext/src/proc.c",     // NOTE(GH-32): This gem is required by `mruby-method`.
            "mrbgems/mruby-sprintf/src/sprintf.c",   // Kernel#sprintf, Kernel#format, String#%
        ]
        .into_iter()
        .map(|source| paths::mruby_root().join(source))
        .chain(
            ["src/gem_init.c", "src/mrbgems.c"]
                .into_iter()
                .map(|source| paths::mrbgems_root().join(source)),
        )
    }

    fn mrbgems_include_dirs() -> impl Iterator<Item = PathBuf> {
        [
            "mrbgems/mruby-class-ext/include", // NOTE(GH-32): Pending removal.
            "mrbgems/mruby-compiler/core",     // Ruby parser and bytecode generation
            "mrbgems/mruby-error/include",     // `mrb_raise`, `mrb_protect`
            "mrbgems/mruby-eval/include",      // eval, instance_eval, and friends
            "mrbgems/mruby-fiber/include",     // Fiber class from core, required by `Enumerator`
            "mrbgems/mruby-metaprog/include",  // APIs on Kernel and Module for accessing classes and variables
            "mrbgems/mruby-method/include",    // `Method`, `UnboundMethod`, and method APIs on Kernel and Module
            "mrbgems/mruby-pack/include",      // Array#pack and String#unpack
            "mrbgems/mruby-proc-ext/include",  // NOTE(GH-32): This gem is required by `mruby-method`.
            "mrbgems/mruby-sprintf/include",   // Kernel#sprintf, Kernel#format, String#%
        ]
        .into_iter()
        .map(|dir| paths::mruby_root().join(dir))
        .chain(mruby_include_dirs())
    }

    fn mrbsys_sources() -> impl Iterator<Item = PathBuf> {
        ["src/ext.c"]
            .into_iter()
            .map(|source| paths::mrbsys_root().join(source))
    }

    fn mrbsys_include_dirs() -> impl Iterator<Item = PathBuf> {
        ["include"]
            .into_iter()
            .map(|dir| paths::mrbsys_root().join(dir))
            .chain(mruby_include_dirs())
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
        [
            "system/include/compat",
            "system/include",
            "system/include/libc",
            "system/lib/libc/musl/arch/emscripten",
            "system/local/include",
            "system/include/SSE",
            "system/include/neon",
            "system/lib/compiler-rt/include",
            "system/lib/libunwind/include",
        ]
        .into_iter()
        .map(|dir| paths::emscripten_root().join(dir))
    }

    fn staticlib(
        target: &Triple,
        name: &str,
        include_dirs: impl Iterator<Item = PathBuf>,
        sources: impl Iterator<Item = PathBuf>,
    ) {
        assert!(
            name.starts_with("lib"),
            "Static lib name must be of the format libXXX.a, got {name}"
        );
        assert_eq!(
            name.rsplit('.').next().map(|ext| ext.eq_ignore_ascii_case("a")),
            Some(true),
            "Static lib name must be of the format libXXX.a, got {name}"
        );
        assert!(
            name.len() > 5,
            "Static lib name must be of the format libXXX.a, got {name}"
        );

        let mut build = cc::Build::new();
        build
            .warnings(false)
            .define("ARTICHOKE", None)
            .define("MRB_ARY_NO_EMBED", None)
            .define("MRB_GC_TURN_OFF_GENERATIONAL", None)
            .define("MRB_INT64", None)
            .define("MRB_NO_BOXING", None)
            .define("MRB_NO_PRESYM", None)
            .define("MRB_NO_STDIO", None)
            .define("MRB_UTF8_STRING", None);

        for source in sources {
            println!("cargo:rerun-if-changed={}", source.to_str().unwrap());
            build.file(source);
        }

        for include_dir in include_dirs {
            build.include(include_dir);
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

        build.compile(name);
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
        assert!(status.success(), "cargo install bindgen failed");

        let status = invoke_bindgen(
            target,
            out_dir,
            bindgen_install_dir.join("bin").join("bindgen").as_os_str(),
        );
        assert!(status.unwrap().success(), "bindgen failed");
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
            .arg(paths::bindgen_header())
            .arg("--")
            .arg("-DARTICHOKE")
            .arg("-DMRB_ARY_NO_EMBED")
            .arg("-DMRB_GC_TURN_OFF_GENERATIONAL")
            .arg("-DMRB_INT64")
            .arg("-DMRB_NO_BOXING")
            .arg("-DMRB_NO_PRESYM")
            .arg("-DMRB_NO_STDIO")
            .arg("-DMRB_UTF8_STRING");

        for include_dir in mruby_include_dirs().chain(mrbsys_include_dirs()) {
            command.arg("-I").arg(include_dir);
        }

        if let Architecture::Wasm32 = target.architecture {
            for include_dir in wasm_include_dirs() {
                command.arg("-I").arg(include_dir);
            }
            command.arg(r#"-DMRB_API=__attribute__((visibility("default")))"#);
        }

        command.status().ok()
    }

    pub fn build(target: &Triple, out_dir: &OsStr) {
        staticlib(target, "libmruby.a", mruby_include_dirs(), mruby_sources());
        staticlib(target, "libmrbgems.a", mrbgems_include_dirs(), mrbgems_sources());
        staticlib(target, "libmrbsys.a", mrbsys_include_dirs(), mrbsys_sources());
        bindgen(target, out_dir);
    }
}

fn main() {
    let target = env::var_os("TARGET").expect("cargo-provided TARGET env variable not set");
    let target = target.to_str().expect("TARGET env variable was not valid UTF-8");
    let target = target
        .parse::<Triple>()
        .unwrap_or_else(|err| panic!("target-lexicon could not parse build target '{target}' with error: {err}"));
    let out_dir = env::var_os("OUT_DIR").expect("cargo-provided OUT_DIR env variable not set");
    libs::build(&target, &out_dir);
}
