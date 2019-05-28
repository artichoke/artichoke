#!/usr/bin/env bash

set -euo pipefail
set -x

lint_ruby_sources() {
  pushd "$@" >/dev/null
  bundle exec rubocop -a
  popd >/dev/null
}

# Rust sources

## Format with rustfmt
cargo fmt
## Lint with Clippy
cargo clippy --all-targets --all-features

# Lint Ruby sources

## mruby::extn
lint_ruby_sources mruby/src/extn
## nemesis
lint_ruby_sources nemesis/ruby
## foolsgold
lint_ruby_sources foolsgold/ruby

# C sources

## Format with clang-format
find . \( -name '*.h' -or -name '*.c' \) -and -not -path '*vendor*' -and -not -path '*target*' -not -path '*node_modules*' -print0 | xargs -0 yarn run clang-format -i

# Shell sources

## Format with shfmt
find . \( -name '*.sh' -or -name '*.bash' \) -and -not -path '*vendor*' -and -not -path '*target*' -not -path '*node_modules*' -print0 | xargs -0 shfmt -w -i 2

## Lint with shellcheck
find . \( -name '*.sh' -or -name '*.bash' \) -and -not -path '*vendor*' -and -not -path '*target*' -not -path '*node_modules*' -print0 | xargs -0 shellcheck

# Text sources (e.g. HTML, Markdown)

## Format with prettier
yarn run prettier --write --prose-wrap always './*.{css,html,js,json,md}' '{!(target),!(node_modules)}**/*.{css,html,js,json,md}'
