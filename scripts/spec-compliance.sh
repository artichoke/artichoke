#!/usr/bin/env bash

set -euo pipefail
set -x

shopt -s globstar

cargo build
spec_runner="$(pwd)/target/debug/spec-runner"

run_core_spec() {
  pushd "spec-runner/vendor/spec/core" >/dev/null
  $spec_runner "$1"/**/*.rb
  popd >/dev/null
}

run_library_spec() {
  pushd "spec-runner/vendor/spec/library" >/dev/null
  $spec_runner "$1"/**/*.rb
  popd >/dev/null
}

run_kernel_spec() {
  pushd "spec-runner/vendor/spec/core" >/dev/null
  $spec_runner kernel/shared/**/*.rb kernel/fixtures/**/*.rb kernel/Integer_spec.rb
  # kernel/Array_spec.rb kernel/Integer_spec.rb kernel/Float_spec.rb kernel/String_spec.rb kernel/Hash_spec.rb
  # kernel/fail_spec.rb kernel/caller_spec.rb kernel/__method___spec.rb
  popd >/dev/null
}

run_kernel_spec

# run_core_spec "comparable"
# run_core_spec "matchdata"
# run_core_spec "regexp"
#
# run_library_spec "monitor"
# run_library_spec "stringscanner"
# run_library_spec "uri"
