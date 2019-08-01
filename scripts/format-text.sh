#!/usr/bin/env bash

set -euo pipefail

yarn install &>/dev/null
PATH="$(yarn bin):$PATH"
export PATH
cd "$(pkg-dir)"

set -x

wrap() {
  if [[ $1 == "md" ]]; then
    echo "--prose-wrap always"
  fi
}

format() {
  # shellcheck disable=SC2046
  find . -type f \
    -and -name "*.$1" \
    -and -not -path '*vendor*' \
    -and -not -path '*target*' \
    -and -not -path '*node_modules*' \
    -and -not -path '*spec/ruby*' -print0 |
    xargs -0 yarn run prettier --write $(wrap "$1")
}

check() {
  # shellcheck disable=SC2046
  find . -type f \
    -and -name "*.$1" \
    -and -not -path '*vendor*' \
    -and -not -path '*target*' \
    -and -not -path '*node_modules*' \
    -and -not -path '*spec/ruby*' -print0 |
    xargs -0 yarn run prettier --check $(wrap "$1")
}

if [[ $# -gt 1 && $1 == '--check' ]]; then
  yarn run prettier --version
  check "$2"
elif [[ $# -gt 1 && $1 == '--format' ]]; then
  yarn run prettier --version
  format "$2"
else
  echo >&2 "Usage: $0 [ --check | --format ] [file type]"
  exit 1
fi
