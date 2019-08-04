#!/usr/bin/env bash

set -euo pipefail
set -x

shopt -s globstar

cargo build
spec_runner="$(pwd)/target/debug/spec-runner"

run_core_spec() {
  pushd "spec-runner/vendor/ruby/core" >/dev/null
  $spec_runner "$1"/**/*.rb
  popd >/dev/null
}

run_library_spec() {
  pushd "spec-runner/vendor/ruby/library" >/dev/null
  $spec_runner "$1"/**/*.rb
  popd >/dev/null
}

run_core_spec "matchdata"
run_core_spec "regexp"

run_library_spec "monitor"
run_library_spec "stringscanner"
run_library_spec "uri"
