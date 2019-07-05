#!/usr/bin/env bash

set -euo pipefail
set -x

shopt -s globstar

run_core_spec() {
  pushd "spec-runner/spec/ruby/core" >/dev/null
  cargo run --bin spec-runner "$1"/**/*.rb
  popd >/dev/null
}

run_library_spec() {
  pushd "spec-runner/spec/ruby/library" >/dev/null
  cargo run --bin spec-runner "$1"/**/*.rb
  popd >/dev/null
}

run_core_spec "matchdata"
run_core_spec "regexp"

run_library_spec "monitor"
run_library_spec "stringscanner"
run_library_spec "uri"
