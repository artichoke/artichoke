# Artichoke Ruby

[![CircleCI](https://circleci.com/gh/artichoke/artichoke.svg?style=svg)](https://circleci.com/gh/artichoke/artichoke)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Core documentation](https://img.shields.io/badge/docs-artichoke--core-blue.svg)](https://artichoke.github.io/artichoke/artichoke_core/)
[![Virtual filesystem documentation](https://img.shields.io/badge/docs-artichoke--vfs-blue.svg)](https://artichoke.github.io/artichoke/artichoke_vfs/)
[![mruby backend documentation](https://img.shields.io/badge/docs-artichoke--backend-blue.svg)](https://artichoke.github.io/artichoke/artichoke_backend/)
[![mruby-sys documentation](https://img.shields.io/badge/docs-mruby--sys-blue.svg)](https://artichoke.github.io/artichoke/mruby_sys/)

<p align="center">
  <a href="https://artichoke.run">
    <img width="200" height="200" src="https://artichoke.run/logo.svg">
  </a>
</p>

Artichoke is a platform for building
[MRI-compatible](https://github.com/ruby/spec) Ruby implementations. Artichoke
provides a Ruby runtime implemented in Rust that can be loaded into many VM
backends. Rubies implemented with Artichoke will be source and C API compatible
with MRI Ruby 2.6.3.

## Try Artichoke

You can [try Artichoke in your browser](https://artichoke.run). The
[Artichoke Playground](https://github.com/artichoke/playground) runs a
[WebAssembly](https://webassembly.org/) build of Artichoke.

If you would prefer to run a local build, you can
[set up a Rust toolchain](/CONTRIBUTING.md#rust-toolchain) and launch an
interactive Artichoke shell with:

```shell
cargo run -p artichoke-frontend --bin airb
```

## Design and Goals

Artichoke is
[designed to enable experimentation](/doc/artichoke-design-and-goals.md). The
top goals of the project are:

- [Support WebAssembly as a build target](https://github.com/artichoke/artichoke/labels/O-wasm-unknown).
- Support embedding and executing Ruby in untrusted environments.
- [Distribute Ruby applications as single-binary artifacts](https://github.com/artichoke/artichoke/labels/A-single-binary).
- [Implement Ruby with state-of-the-art dependencies](https://github.com/artichoke/artichoke/labels/A-deps).
- Experiment with VMs to support
  [dynamic codegen](https://github.com/artichoke/artichoke/labels/A-codegen),
  [ahead of time compilation](https://github.com/artichoke/artichoke/labels/A-compiler),
  [parallelism and eliminating the GIL](https://github.com/artichoke/artichoke/labels/A-parallelism),
  and novel
  [memory management and garbage collection techniques](https://github.com/artichoke/artichoke/labels/A-memory-management).

## Contributing

Artichoke aspires to be a Ruby 2.6.3-compatible implementation of the Ruby
programming language.
[There is lots to do](https://github.com/artichoke/artichoke/issues).

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you
[filed an issue so we can fix it](https://github.com/artichoke/artichoke/issues/new).

If you would like to contribute code 👩‍💻👨‍💻, find an issue that looks interesting
and leave a comment that you're beginning to investigate. If there is no issue,
please file one before beginning to work on a PR.
[Good first issues are labeled `E-easy`](https://github.com/artichoke/artichoke/labels/E-easy).

### Discussion

If you'd like to engage in a discussion outside of GitHub, you can
[join Artichoke's public Discord server](https://discord.gg/QCe2tp2).
