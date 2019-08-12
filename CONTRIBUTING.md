# Contributing to Artichoke

👋 Hi and welcome to [Artichoke](https://github.com/artichoke). Thanks for
taking the time to contribute! 💪💎🙌

Artichoke aspires to be a Ruby 2.6.3-compatible implementation of the Ruby
programming language.
[There is lots to do](https://github.com/artichoke/artichoke/issues).

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you
[filed an issue so we can fix it](https://github.com/artichoke/artichoke/issues/new).

If you would like to contribute code 👩‍💻👨‍💻, find an issue that looks interesting
and leave a comment that you're beginning to investigate. If there is no issue,
please file one before beginning to work on a PR.

## Discussion

If you'd like to engage in a discussion outside of GitHub, you can
[join Artichoke's public Discord server](https://discord.gg/QCe2tp2).

## Implementation Philosophy

- Prefer pure Ruby implementations when initially implementing features.
- A feature is not done until it passes [ruby/spec](/doc/ruby-spec.md).
- Move implementations to Rust for performance, e.g.
  [using Serde to implement the JSON package](https://github.com/artichoke/artichoke/issues/77).
- If there is a Rust crate that does what we need, prefer to use it. Forking is
  OK, too, e.g.
  [artichoke/rust-onig](https://github.com/artichoke/rust-onig/tree/wasm).

## Setup

Artichoke includes Rust, Ruby, C, Shell, and Text sources. Developing on
Artichoke requires configuring several dependencies, which are orchestrated by
[Yarn](https://yarnpkg.com/).

### Rust Toolchain

Artichoke depends on nightly Rust and several compiler plugins for linting and
formatting. The specific version of Rust Artichoke requires is specified in the
[toolchain file](/rust-toolchain)

#### Installation

The recommended way to install the Rust toolchain is with
[rustup](https://rustup.rs/). On macOS, you can install rustup with
[Homebrew](https://docs.brew.sh/Installation):

```shell
brew install rustup-init
rustup-init
```

Once you have rustup, you can install the Rust toolchain needed to compile
Artichoke.

```shell
rustup toolchain install "$(cat rust-toolchain)"
rustup component add rustfmt
rustup component add clippy
```

### Rust Crates

Artichoke depends on several Rust libraries, or crates. Once you have the Rust
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

### C Toolchain

#### `cc` Crate

Artichoke and some of its dependencies use the Rust
[`cc` crate](https://crates.io/crates/cc) to build. `cc` uses a
[platform-dependent C compiler](https://github.com/alexcrichton/cc-rs#compile-time-requirements)
to compile C sources. On Unix, `cc` crate uses the `cc` binary.

#### mruby Backend

To build the Artichoke mruby backend, you will need a C compiler toolchain. By
default, mruby requires the following to compile:

- clang
- bison
- ar

You can override the requirement for clang by setting the `CC` and `LD`
environment variables.

### Node.js

Artichoke uses Yarn and Node.js for linting and orchestration.

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

### Ruby

Artichoke requires a recent Ruby 2.x and [bundler](https://bundler.io/) 2.x. The
[`.ruby-version`](/.ruby-version) file in the root of Artichoke specifies Ruby
2.6.3.

If you use [RVM](https://rvm.io/), you can install Ruby dependencies by running:

```shell
rvm install "$(cat .ruby-version)"
gem install bundler
```

If you use [rbenv](https://github.com/rbenv/rbenv) and
[ruby-build](https://github.com/rbenv/ruby-build), you can install Ruby
dependencies by running:

```shell
rbenv install "$(cat .ruby-version)"
gem install bundler
rbenv rehash
```

To lint Ruby sources, Artichoke uses
[RuboCop](https://github.com/rubocop-hq/rubocop). `yarn lint` installs RuboCop
and all other gems automatically.

### Shell

Artichoke uses [shfmt](https://github.com/mvdan/sh) for formatting and
[shellcheck](https://github.com/koalaman/shellcheck) for linting Shell scripts.

On macOS, you can install shfmt and shellcheck with
[Homebrew](https://docs.brew.sh/Installation):

```shell
brew install shfmt shellcheck
```

## Code Quality

### Linting

Once you [configure a development environment](#setup), run the following to
lint sources:

```shell
yarn lint
```

Merges will be blocked by CI if there are lint errors.

### Testing

A PR must have tests for it to be merged. The
[Rust book chapter on testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
is a good place to start. If you'd like to see some examples in Artichoke, take
a look at the `Value` tests in
[`artichoke-backend/src/value/mod.rs`](/artichoke-backend/src/value/mod.rs).

To run tests:

```shell
cargo test
```

If you are only working on one package, it can speed up iteration time to only
build and run tests for that package:

```shell
cargo test -p artichoke-backend
```

`cargo test` accepts a filter argument that will limit test execution to tests
that substring match. For example, to run all of the
[`Regexp`](/artichoke-backend/src/extn/core/regexp) tests:

```shell
cargo test -p artichoke-backend regexp
```

Tests are run for every PR. All builds must pass before merging a PR.

## Updating Dependencies

### Rust Toolchain

Because rustfmt, clippy, and the language server sometimes break on nightly,
Artichoke pegs a specific date archive of nightly. If you want to update the
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
[file an issue](https://github.com/artichoke/artichoke/issues/new).

## Code Analysis

### Source Code Statistics

To view statistics about the source code in Artichoke, you can run `yarn loc`,
which depends on [loc](https://github.com/cgag/loc). You can install loc by
running:

```shell
cargo install loc
```

### Flamegraphs

To generate flamegraphs with, you need the
[inferno flamegraph implementation](https://github.com/jonhoo/inferno). You can
install inferno by running:

```shell
cargo install inferno
```
