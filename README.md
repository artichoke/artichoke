# Artichoke Ruby

[![CircleCI](https://circleci.com/gh/artichoke/artichoke.svg?style=svg)](https://circleci.com/gh/artichoke/artichoke)
[![Documentation](https://img.shields.io/badge/docs-artichoke-blue.svg)](https://artichoke.github.io/artichoke/)

<p align="center">
  <img width="200" height="200" src="https://raw.githubusercontent.com/artichoke/logo/master/dist/artichoke-rb.png">
</p>

Artichoke is a Ruby implementation written in Rust. Artichoke aspires to be
source-compatible with [Ruby 2.6.3](https://github.com/ruby/ruby/tree/v2_6_3)
and pass 100% of [ruby/spec](/spec-runner/spec/ruby). To view current progress
on ruby/spec compliance, see
[`scripts/spec-compliance.sh`](/scripts/spec-compliance.sh).

Artichoke is a work-in-progress. When functional, Artichoke will improve upon
MRI in the following ways:

- True parallelism with no GIL.
- Optional multi-threading.
- [Deterministic garbage collection](https://github.com/artichoke/cactusref).
- Single binary distribution (all Ruby sources from core and stdlib are embedded
  in `artichoke` executable).
- WASM build target.
- [mruby](https://github.com/mruby/mruby)-compatible C API (all 311 `MRB_API`
  functions).
- Emoji identifiers (classes, modules, methods, variables) 💪
- Configurable build: all packages in stdlib are optional.
- Native Rust extensions exposed via `require` of virtual files.
- Filesystem access is either via the system or via an in-memory virtual
  filesystem.

Artichoke will deviate from MRI in the following ways:

- The only supported encodings are UTF-8,
  [maybe UTF-8](https://github.com/BurntSushi/bstr), and binary.
- Ruby source files are always interpreted as UTF-8.
- No equivalent C API, which means C extensions are unsupported.

## Try Artichoke

Once you [set up a Rust toolchain](/doc/development-setup.md), you can launch an
interactive Artichoke shell with:

```shell
cargo run -p artichoke-frontend --bin airb
```
