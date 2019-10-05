#!/usr/bin/env bash

set -euo pipefail
set -x

shopt -s globstar

spec_runner="$(pwd)/target/debug/spec-runner"

if [[ $# -eq 2 ]]; then
  family="$1"
  component="$2"
  cargo build
  pushd "spec-runner/vendor/spec" >/dev/null
  $spec_runner ./**/shared/**/*.rb ./**/fixtures/**/*.rb "./$family/$component/${spec}_spec.rb"
elif [[ $# -eq 3 ]]; then
  family="$1"
  component="$2"
  spec="$3"
  cargo build
  pushd "spec-runner/vendor/spec" >/dev/null
  $spec_runner ./**/shared/**/*.rb ./**/fixtures/**/*.rb "./$family/$component/${spec}_spec.rb"
else
  echo 1>&2 "Usage: $0 language|core|library component [spec]"
fi
