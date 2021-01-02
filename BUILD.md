# Building Artichoke

To build Artichoke, install the [prerequisites](#prerequisites) and run:

```console
$ git clone https://github.com/artichoke/artichoke.git
$ cd ./artichoke
$ cargo build --release
$ ./target/release/artichoke --version
artichoke 0.1.0-pre.0
```

## WebAssembly

Artichoke can be used in WebAssembly environments via the
`wasm32-unknown-emscripten` target. This target is not tested in CI and should
be considered unstable.

```sh
rustup target add wasm32-unknown-emscripten
cargo build --release --target wasm32-unknown-unknown
```

This on its own does not produce a usable artifact. To build a WebAssembly
bundle, depend on `artichoke` in a crate with a main. See the
[artichoke/playground] repository for an example.

## Prerequisites

### Rust Toolchain

Artichoke is a collection of Rust crates and requires a Rust compiler. The
specific version of Rust Artichoke requires is specified in the
[toolchain file](rust-toolchain).

Artichoke only guarantees support for the latest stable version of the Rust
compiler.

#### Installation

The recommended way to install the Rust toolchain is with [rustup]. On macOS,
you can install rustup with [Homebrew]:

```sh
brew install rustup-init
rustup-init
```

Once you have rustup, you can install the Rust toolchain needed to compile
Artichoke.

```sh
rustup toolchain install "$(cat rust-toolchain)"
rustup component add rustfmt
rustup component add clippy
```

### Rust Crates

Artichoke depends on several Rust libraries, or crates. Once you have the Rust
toolchain installed, you can install the crates specified in
[`Cargo.lock`](Cargo.lock) by running:

```sh
cargo build --workspace
```

### C Toolchain

Some artichoke dependencies, like the mruby [`sys`](artichoke-backend/src/sys)
FFI bindings and the [`onig`] crate, build C static libraries and require a C
compiler.

Artichoke specifically requires clang. WebAssembly targets require clang-8 or
newer.

#### `cc` Crate

Artichoke and some of its dependencies use the Rust [`cc` crate] to build. `cc`
uses a [platform-dependent C compiler] to compile C sources. On Unix, `cc` crate
uses the `cc` binary.

### mruby Bindings

To build the Artichoke mruby backend, you will need a C compiler toolchain. By
default, mruby requires the following to compile:

- clang
- ar

You can override the requirement for clang by setting the `CC` and `LD`
environment variables.

[artichoke/playground]: https://github.com/artichoke/playground
[rustup]: https://rustup.rs/
[homebrew]: https://docs.brew.sh/Installation
[`onig`]: https://crates.io/crates/onig
[`cc` crate]: https://crates.io/crates/cc
[platform-dependent c compiler]:
  https://github.com/alexcrichton/cc-rs#compile-time-requirements
