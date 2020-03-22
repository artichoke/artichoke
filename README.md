# Artichoke Ruby

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/artichoke.svg)](https://crates.io/crates/artichoke)
[![API](https://docs.rs/artichoke/badge.svg)](https://docs.rs/artichoke)
[![API master](https://img.shields.io/badge/docs-master-blue.svg)](https://artichoke.github.io/artichoke/artichoke/)

<p align="center">
  <a href="https://artichoke.run">
    <img height="200" width="200" src="https://artichoke.run/logo.svg">
  </a>
</p>

Artichoke is a Ruby implementation written in Rust and Ruby. Artichoke intends
to be [MRI-compatible](https://github.com/ruby/spec) and targets Ruby 2.6.3.
Artichoke provides a Ruby runtime implemented in Rust and Ruby.

## Try Artichoke

<p align="center">
  <a href="https://artichoke.run">
    <img style="max-width: 400px" width="400" src="https://artichoke.run/playground.png?bust">
  </a>
  <br>
  <em>Artichoke Ruby Wasm Playground</em>
</p>

You can [try Artichoke in your browser](https://artichoke.run). The
[Artichoke Playground](https://github.com/artichoke/playground) runs a
[WebAssembly](https://webassembly.org/) build of Artichoke.

## Install Artichoke

You can install a pre-release build of Artichoke using `cargo`, Rust's package
manager, by running:

```sh
cargo install --git https://github.com/artichoke/artichoke --locked
```

If instead you'd like to build Artichoke locally, you'll need Rust, clang, and
Ruby. [`BUILD.md`](BUILD.md) has more detail on
[how to set up the compiler toolchain](BUILD.md#prerequisites).

## Usage

Artichoke ships with two binaries: `airb` and `artichoke`.

### `airb`

`airb` is the Artichoke implementation of `irb` and is an interactive Ruby shell
and [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).

`airb` is a readline-enabled shell, although it does not persist history.

## `artichoke`

`artichoke` is the `ruby` binary frontend to Artichoke.

`artichoke` supports executing programs via files, stdin, or inline with one or
more `-e` flags.

Artichoke does not yet support local filesystem access. A temporary workaround
is to inject data into the interpreter with the `--with-fixture` flag, which
reads file contents into a `$fixture` global.

```console
$ artichoke --help
artichoke 0.1.0-pre.0
Artichoke is a Ruby made with Rust.

USAGE:
    artichoke [FLAGS] [OPTIONS] [--] [programfile]

FLAGS:
        --copyright    print the copyright
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -e <commands>...                one line of script. Several -e's allowed. Omit [programfile]
        --with-fixture <fixture>    file whose contents will be read into the `$fixture` global

ARGS:
    <programfile>
```

## Design and Goals

Artichoke is [designed to enable experimentation](VISION.md). The top goals of
the project are:

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

## License

artichoke is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

Some portions of Artichoke are derived from third party sources. The READMEs in
each crate discuss which third party licenses are applicable to the sources and
derived works in Artichoke.
