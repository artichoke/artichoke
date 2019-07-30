#!/usr/bin/env bash

ensure_emsdk() {
  # shellcheck disable=SC2091
  if [ ! -f "target/emsdk/emsdk" ]; then
    git clone https://github.com/emscripten-core/emsdk.git target/emsdk
    ./target/emsdk/emsdk install latest
  fi
  ./target/emsdk/emsdk activate latest
}

clean_emsdk() {
  rm -rf ./target/emsdk
}

ensure_emsdk
# shellcheck disable=SC1091
. target/emsdk/emsdk_env.sh
