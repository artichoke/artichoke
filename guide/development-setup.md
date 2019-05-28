# Development Setup

ferrocarril includes Rust, Ruby, C, Shell, and Text sources. Developing on
ferrocarril requires several dependencies, which are orchestrated by
[Yarn](https://yarnpkg.com/).

## Linting

Once you are [set up](#dependencies), run the following to lint sources:

```shell
yarn lint
```

Merges will be blocked by CI if there are lint errors.

## Testing

A PR must have tests for it to be merged. The
[Rust book chapter on testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
is a good place to start. If you'd like to see some examples in ferrocarril,
take a look at the `Value` tests in
[`mruby/src/value/mod.rs`](/mruby/src/value/mod.rs).

To run tests:

```shell
cargo test
```

If you are only working on one package, it can speed up iteration time to only
build and run tests for that package:

```shell
cargo test -p mruby
```

`cargo test` accepts a filter argument that will limit test execution to tests
that substring match. For example, to run all of the
[`Regexp`](/mruby/src/extn/core/regexp.rs) tests:

```shell
cargo test -p mruby regexp
```

Tests are run for every PR. All builds must pass before merging a PR.

## Dependencies

### Rust Toolchain

ferrocarril depends on nightly Rust as specified in the
[toolchain file](/rust-toolchain) and several compiler plugins for linting and
formatting.

#### Installation

The recommended way to install the Rust toolchain is with
[rustup](https://rustup.rs/). On macOS, you can install rustup with
[Homebrew](https://docs.brew.sh/Installation):

```shell
brew install rustup-init
rustup-init
```

Once you have rustup, you can install the Rust toolchain needed to compile
ferrocarril.

```shell
rustup toolchain install "$(cat rust-toolchain)"
rustup component add rustfmt
rustup component add clippy
```

### Rust Crates

ferrocarril depends on several Rust libraries, or crates. Once you have the Rust
toolchain installed, you can install the crates specified in
[`Cargo.lock`](/Cargo.lock) by running:

```shell
cargo build
```

You can check to see that this worked by running the following and observing no
errors:

```shell
cargo test
cargo fmt -- --check
cargo clippy --all-targets --all-features
```

### Node.js

ferrocarril uses Yarn and Node.js for linting and orchestration.

You will need to install
[Node.js](https://nodejs.org/en/download/package-manager/) and
[Yarn](https://yarnpkg.com/en/docs/install).

On macOS, you can install Node.js and Yarn with
[Homebrew](https://docs.brew.sh/Installation):

```shell
brew install node yarn
```

### Node.js Packages

Once you have Yarn installed, you can install the packages specified in
[`package.json`](/package.json) by running:

```shell
yarn install
```

You can check to see that this worked by running `yarn lint` and observing no
errors.

## Updating Dependencies

### Rust Toolchain

Because rustfmt, clippy, and the language server sometimes break on nightly,
ferrocarril pegs a specific date archive of nightly. If you want to update the
pegged nightly version, choose one that has
[passing builds for rustfmt, clippy, and rls](https://rust-lang-nursery.github.io/rust-toolstate/);
otherwise, the build will fail on [CI](/.circleci/config.yml).

### Rust Crates

Version specifiers in `Cargo.toml` are NPM caret-style by default. A version
specifier of `4.1.2` means `4.1.2 <= version < 5.0.0`.

To see what crates are outdated, you can use
[cargo-outdated](https://github.com/kbknapp/cargo-outdated).

If you need to pull in an updated version of a crate for a bugfix or a new
feature, update the version number in `Cargo.toml`.

To update Rust crate dependencies run the following command and check in the
updated `Cargo.lock` file:

```shell
cargo update
```

### Node.js Packages

To see what packages are outdated, you can run `yarn outdated`.

To update Node.js package dependencies run the following command and check in
the updated `yarn.lock` file:

```shell
yarn upgrade
```

If after running `yarn upgrade` there are still outdated packages reported by
`yarn outdated`, there has likely been a major release of a dependency. If you
would like to update the dependency and deal with any breakage, please do;
otherwise, please
[file an issue](https://github.com/lopopolo/ferrocarril/issues/new).

## Source Code Statistics

To view statistics about the source code in ferrocarril, you can run `yarn loc`,
which depends on [loc](https://github.com/cgag/loc). You can install loc by
running:

```shell
cargo install loc
```
