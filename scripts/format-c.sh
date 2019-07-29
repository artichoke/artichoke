#!/usr/bin/env bash

set -euo pipefail

yarn install &>/dev/null
PATH="$(yarn bin):$PATH"
export PATH
cd "$(pkg-dir)"

set -x

format() {
  find . -type f \
    -and \( -name '*.h' -or -name '*.c' \) \
    -and -not -path '*vendor*' \
    -and -not -path '*target*' \
    -and -not -path '*node_modules*' \
    -and -not -path '*spec/ruby*' -print0 |
    xargs -0 yarn run clang-format -i
}

_check_clang_format() {
  if yarn run clang-format -output-replacements-xml "$1" | grep -q "offset"; then
    echo >&2 "KO: $1"
    echo >&2 "    Please run 'yarn lint' to resolve C formatting issues"
    return 1
  else
    echo "OK: $1"
  fi
}

export -f _check_clang_format

check() {
  find . -type f \
    -and \( -name '*.h' -or -name '*.c' \) \
    -and -not -path '*vendor*' \
    -and -not -path '*target*' \
    -and -not -path '*node_modules*' \
    -and -not -path '*spec/ruby*' -print0 |
    xargs -0 -n1 bash -c '_check_clang_format "$@"' _
}

if [[ $# -gt 0 && $1 == '--check' ]]; then
  yarn run clang-format --version
  check
elif [[ $# -gt 0 && $1 == '--format' ]]; then
  yarn run clang-format --version
  format
else
  echo >&2 "Usage: $0 [ --check | --format ]"
  exit 1
fi
