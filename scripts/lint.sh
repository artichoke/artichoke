#!/usr/bin/env bash

set -euo pipefail

yarn install
PATH="$(yarn bin):$PATH"
export PATH
cd "$(pkg-dir)"

set -x

# Yarn orchestration

## Lint package.json
pjv

# Rust sources

## Format with rustfmt
cargo fmt
## Lint with Clippy
cargo clippy --all-targets --all-features
## Lint docs
cargo doc --no-deps --all

# Lint Ruby sources

lint_ruby_sources() {
  pushd "$@" >/dev/null
  bundle install >/dev/null
  bundle exec rubocop -a
  popd >/dev/null
}

## mruby::extn
lint_ruby_sources mruby/src/extn
## spec-runner
lint_ruby_sources spec-runner/src
## mruby bins
lint_ruby_sources mruby-bin/ruby

# C sources

## Format with clang-format
./scripts/format-c.sh

# Shell sources

## Format with shfmt
shfmt -f . | grep -v target/ | grep -v node_modules/ | xargs shfmt -i 2 -ci -s -w
## Lint with shellcheck
shfmt -f . | grep -v target/ | grep -v node_modules/ | xargs shellcheck

# Text sources (e.g. HTML, Markdown)

## Format with prettier
prettier --write --prose-wrap always '**/*.{css,html,js,json,md}'
