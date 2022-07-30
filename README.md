# Artichoke Ruby

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/artichoke.svg)](https://crates.io/crates/artichoke)
[![API](https://docs.rs/artichoke/badge.svg)](https://docs.rs/artichoke)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/artichoke/)

<p align="center">
  <a href="https://www.artichokeruby.org">
    <img alt="Artichoke Ruby logo" height="200" width="200" src="https://www.artichokeruby.org/artichoke-logo.svg">
  </a>
</p>

Artichoke is a Ruby implementation written in Rust and Ruby. Artichoke intends
to be [MRI-compatible][ruby-spec] and targets [recent MRI Ruby][mri-target].
Artichoke provides a Ruby runtime implemented in Rust and Ruby.

## Try Artichoke

<p align="center">
  <a href="https://artichoke.run">
    <img alt="Artichoke Ruby WebAssembly playground" style="max-width: 400px" width="400" src="https://artichoke.run/artichoke-playground-safari-revision-4938-light-mode.png">
  </a>
  <br>
  <em>Artichoke Ruby Wasm Playground</em>
</p>

You can [try Artichoke in your browser][playground]. The [Artichoke
Playground][playground-repo] runs a [WebAssembly] build of Artichoke.

## Install Artichoke

### Prebuilt nightly binaries

[Download a prebuilt binary from artichoke/nightly][nightlies]. Binaries are
available for Linux, Linux/musl, macOS, and Windows.

These daily binaries track the latest trunk branch of Artichoke.

Binaries are also distributed through [ruby-build]. To install with [rbenv]:

```console
$ rbenv install artichoke-dev
```

### Cargo

You can install a pre-release build of Artichoke using `cargo`, Rust's package
manager, by running:

```console
$ cargo install --git https://github.com/artichoke/artichoke --branch trunk --locked artichoke
```

To install via `cargo install` or to checkout and build locally, you'll need
Rust and clang. [`BUILD.md`](BUILD.md) has more detail on
[how to set up the compiler toolchain](BUILD.md#prerequisites).

### Docker

[Artichoke is available on Docker Hub][docker-hub].

You can launch a REPL by running:

```sh
docker run -it docker.io/artichokeruby/artichoke airb
```

## Usage

Artichoke ships with two binaries: `airb` and `artichoke`.

### `airb`

`airb` is the Artichoke implementation of `irb` and is an interactive Ruby shell
and [REPL].

`airb` is a readline-enabled shell, although it does not persist history.

### `artichoke`

`artichoke` is the `ruby` binary frontend to Artichoke.

`artichoke` supports executing programs via files, stdin, or inline with one or
more `-e` flags.

Artichoke can `require`, `require_relative`, and `load` files from the local
file system, but otherwise does not yet support local file system access. A
temporary workaround is to inject data into the interpreter with the
`--with-fixture` flag, which reads file contents into a `$fixture` global.

```console
$ artichoke --help
artichoke 0.1.0-pre.0
Artichoke is a Ruby made with Rust.

USAGE:
    artichoke [OPTIONS] [ARGS]

ARGS:
    <programfile>
    <arguments>...

OPTIONS:
        --copyright                 print the copyright
    -e <commands>                   one line of script. Several -e's allowed. Omit [programfile]
    -h, --help                      Print help information
    -V, --version                   Print version information
        --with-fixture <fixture>    file whose contents will be read into the `$fixture` global
```

## Design and Goals

Artichoke is [designed to enable experimentation](VISION.md). The top goals of
the project are:

- [Support WebAssembly as a build target][o-wasm].
- Support embedding and executing Ruby in untrusted environments.
- [Distribute Ruby applications as single-binary artifacts][a-single-binary].
- [Implement Ruby with state-of-the-art dependencies][a-deps].
- Experiment with VMs to support [dynamic codegen][a-codegen], [ahead of time
  compilation][a-compiler], [parallelism and eliminating the
  GIL][a-parallelism], and novel [memory management and garbage collection
  techniques][a-memory-management].

## Contributing

Artichoke aspires to be an [MRI Ruby-compatible][mri-target] implementation of
the Ruby programming language. [There is lots to do][github-issues].

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you [filed an issue so we can fix
it][file-an-issue].

If you would like to contribute code 👩‍💻👨‍💻, find an issue that looks interesting
and leave a comment that you're beginning to investigate. If there is no issue,
please file one before beginning to work on a PR. [Good first issues are labeled
`E-easy`][e-easy].

### Discussion

If you'd like to engage in a discussion outside of GitHub, you can [join
Artichoke's public Discord server][discord].

## License

`artichoke` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

Some portions of Artichoke are derived from third party sources. The READMEs in
each workspace crate discuss which third party licenses are applicable to the
sources and derived works in Artichoke.

[ruby-spec]: https://github.com/ruby/spec
[mri-target]:
  https://github.com/artichoke/artichoke/blob/trunk/RUBYSPEC.md#mri-target
[playground]: https://artichoke.run
[playground-repo]: https://github.com/artichoke/playground
[webassembly]: https://webassembly.org/
[nightlies]: https://github.com/artichoke/nightly/releases/latest
[docker-hub]: https://hub.docker.com/r/artichokeruby/artichoke
[ruby-build]: https://github.com/rbenv/ruby-build
[rbenv]: https://github.com/rbenv/rbenv
[repl]: https://en.wikipedia.org/wiki/Interactive_Ruby_Shell
[o-wasm]: https://github.com/artichoke/artichoke/labels/O-wasm-unknown
[a-single-binary]: https://github.com/artichoke/artichoke/labels/A-single-binary
[a-deps]: https://github.com/artichoke/artichoke/labels/A-deps
[a-codegen]: https://github.com/artichoke/artichoke/labels/A-codegen
[a-compiler]: https://github.com/artichoke/artichoke/labels/A-compiler
[a-parallelism]: https://github.com/artichoke/artichoke/labels/A-parallelism
[a-memory-management]:
  https://github.com/artichoke/artichoke/labels/A-memory-management
[github-issues]: https://github.com/artichoke/artichoke/issues
[file-an-issue]: https://github.com/artichoke/artichoke/issues/new
[discord]: https://discord.gg/QCe2tp2
[e-easy]: https://github.com/artichoke/artichoke/labels/E-easy
