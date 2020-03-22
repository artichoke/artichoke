# Building Artichoke

To build Artichoke, install the [prerequisites](#prerequisites) and run:

```sh
git clone https://github.com/artichoke/artichoke.git
cd ./artichoke
cargo build
cargo test
```

Cross-builds are supported with the normal cargo invocation (although they are
not yet tested in CI):

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

## Prerequisites

### Rust Toolchain

Artichoke is a collection of Rust crates and requires a Rust compiler. The
specific version of Rust Artichoke requires is specified in the
[toolchain file](/rust-toolchain)

#### Installation

The recommended way to install the Rust toolchain is with
[rustup](https://rustup.rs/). On macOS, you can install rustup with
[Homebrew](https://docs.brew.sh/Installation):

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
[`Cargo.lock`](/Cargo.lock) by running:

```sh
cargo build
```

### C Toolchain

Some artichoke dependencies, like the mruby [`sys`](/artichoke-backend/src/sys)
and [`onig`](https://docs.rs/onig/) build C static libraries and require a C
compiler.

Artichoke specifically requires clang. Wasm targets require clang-8 or newer.

#### `cc` Crate

Artichoke and some of its dependencies use the Rust
[`cc` crate](https://crates.io/crates/cc) to build. `cc` uses a
[platform-dependent C compiler](https://github.com/alexcrichton/cc-rs#compile-time-requirements)
to compile C sources. On Unix, `cc` crate uses the `cc` binary.

### mruby Bindings

To build the Artichoke mruby backend, you will need a C compiler toolchain. By
default, mruby requires the following to compile:

- clang
- bison
- ar

You can override the requirement for clang by setting the `CC` and `LD`
environment variables.
