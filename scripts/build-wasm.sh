#!/usr/bin/env bash

set -euo pipefail

yarn install &>/dev/null
PATH="$(yarn bin):$PATH"
export PATH
cd "$(pkg-dir)"

set -x

ensure_emsdk() {
  if [[ ! -f "target/emsdk/emsdk" ]]; then
    git clone https://github.com/emscripten-core/emsdk.git target/emsdk
    ./target/emsdk/emsdk install latest
  fi
  ./target/emsdk/emsdk activate latest
}

export CARGO_CFG_TARGET_FAMILY="wasm"
# export CARGO_PROFILE_DEV_OPT_LEVEL="s"
export CARGO_PROFILE_RELEASE_OPT_LEVEL="s"

if [[ $# -gt 0 && $1 == '--development' ]]; then
  ensure_emsdk
  # shellcheck disable=SC1091
  . target/emsdk/emsdk_env.sh
  cargo build -Z config-profile --target wasm32-unknown-emscripten -p artichoke-wasm
  yarn run webpack --mode development
  yarn run webpack-dev-server --mode development --content-base target/webpack/debug/ --open
elif [[ $# -gt 0 && $1 == '--production' ]]; then
  ensure_emsdk
  # shellcheck disable=SC1091
  . target/emsdk/emsdk_env.sh
  cargo build -Z config-profile --target wasm32-unknown-emscripten -p artichoke-wasm --release
  yarn run webpack --mode production
  yarn run webpack-dev-server --mode production --content-base target/webpack/release/ --open
else
  echo >&2 "Usage: $0 [ --development | --production ]"
  exit 1
fi
