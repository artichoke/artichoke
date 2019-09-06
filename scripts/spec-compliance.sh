#!/usr/bin/env bash

set -euo pipefail
set -x

shopt -s globstar

cargo build
spec_runner="$(pwd)/target/debug/spec-runner"

run_core_spec() {
  pushd "spec-runner/vendor/spec/core" >/dev/null
  $spec_runner ../**/shared/**/*.rb "$1"/**/*.rb
  popd >/dev/null
}

run_library_spec() {
  pushd "spec-runner/vendor/spec/library" >/dev/null
  $spec_runner ../**/shared/**/*.rb "$1"/**/*.rb
  popd >/dev/null
}

run_core_spec "comparable"
run_core_spec "matchdata"
run_core_spec "regexp"

run_library_spec "monitor"
run_library_spec "stringscanner"
run_library_spec "uri"
