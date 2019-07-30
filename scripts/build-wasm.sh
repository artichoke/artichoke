#!/usr/bin/env bash

# This script requires sourcing scripts/activate-wasm-build-env.sh before
# executing a build.

set -euo pipefail

PATH="$(yarn bin):$PATH"
export PATH
cd "$(pkg-dir)"

set -x

target="wasm32-unknown-emscripten"
profile=""
out="target/webpack/debug/"
webpack_mode="development"

while [[ $# -gt 0 ]]; do
  key="$1"
  shift
  case $key in
    -t|--target)
    target="$1"
    shift
    ;;
    --release)
    profile="$key"
    out="target/webpack/release/"
    webpack_mode="production"
    ;;
  esac
done

export CARGO_CFG_TARGET_FAMILY="wasm"
# export CARGO_PROFILE_DEV_OPT_LEVEL="s"
export CARGO_PROFILE_RELEASE_OPT_LEVEL="s"

if [[ -z "$profile" ]]; then
  cargo build -Z config-profile --target "$target" -p artichoke-wasm
else
  cargo build -Z config-profile --target "$target" -p artichoke-wasm "$profile"
fi
yarn run webpack --mode "$webpack_mode"
yarn run webpack-dev-server --mode "$webpack_mode" --content-base "$out" --open
