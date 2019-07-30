#!/usr/bin/env bash

ensure_emsdk() {
  yarn install >/dev/null 2>&1
  # shellcheck disable=SC2091
  cd "$($(yarn bin pkg-dir))" || return 1
  if [ ! -f "target/emsdk/emsdk" ]; then
    git clone https://github.com/emscripten-core/emsdk.git target/emsdk
    ./target/emsdk/emsdk install latest
  fi
  ./target/emsdk/emsdk activate latest
  cd - || return 1
}

clean_emsdk() {
  yarn install >/dev/null 2>&1
  # shellcheck disable=SC2091
  cd "$($(yarn bin pkg-dir))" || return 1
  rm -rf ./target/emsdk
  cd - || return 1
}

ensure_emsdk
# shellcheck disable=SC1091
. target/emsdk/emsdk_env.sh
