#!/usr/bin/env bash

set -euo pipefail

if command -v yarn; then
  yarn install &>/dev/null
  PATH="$(yarn bin):$PATH"
  export PATH
  cd "$(pkg-dir)"
fi

set -x
shopt -s globstar
declare -a specs

register_spec() {
  if [[ $# -eq 2 ]]; then
    family="$1"
    component="$2"
    pushd "spec-runner/vendor/spec" >/dev/null
    for spec in "./$family/$component/"*_spec.rb; do
      specs+=("$spec")
    done
    popd >/dev/null
  elif [[ $# -eq 3 ]]; then
    family="$1"
    component="$2"
    spec="$3"
    pushd "spec-runner/vendor/spec" >/dev/null
    specs+=("./$family/$component/${spec}_spec.rb")
    popd >/dev/null
  else
    echo 1>&2 "Usage: $0 language|core|library component [spec]"
  fi
}

run_specs_artichoke() {
  bin="$(pwd)/target/debug/spec-runner"
  pushd "spec-runner/vendor/spec" >/dev/null
  if [[ $# -eq 1 && $1 -eq "--with-timings" ]]; then
    if command -v precise-time; then
      precise-time "$bin" ./**/shared/**/*.rb ./**/fixtures/**/*.rb "${specs[@]}"
    else
      time "$bin" ./**/shared/**/*.rb ./**/fixtures/**/*.rb "${specs[@]}"
    fi
  else
    "$bin" ./**/shared/**/*.rb ./**/fixtures/**/*.rb "${specs[@]}"
  fi
  popd >/dev/null
}

run_specs_ruby() {
  bin="$(pwd)/spec-runner/src/spec_runner.rb"
  pushd "spec-runner/vendor/spec" >/dev/null
  if [[ $# -eq 1 && $1 -eq "--with-timings" ]]; then
    if command -v precise-time; then
      precise-time "$bin" ./**/shared/**/*.rb ./**/fixtures/**/*.rb "${specs[@]}"
    else
      time "$bin" ./**/shared/**/*.rb ./**/fixtures/**/*.rb "${specs[@]}"
    fi
  else
    "$bin" ./**/shared/**/*.rb ./**/fixtures/**/*.rb "${specs[@]}"
  fi
  popd >/dev/null
}

# register_spec core array allocate
register_spec core array any
register_spec core array append
register_spec core array array
register_spec core array assoc
register_spec core array at
# register_spec core array bsearch
# register_spec core array bsearch_index
register_spec core array clear
# register_spec core array clone
register_spec core array collect
register_spec core array combination
register_spec core array compact
# register_spec core array comparison
# register_spec core array concat
register_spec core array count
register_spec core array cycle
register_spec core array delete_at
register_spec core array delete_if
register_spec core array delete
# register_spec core array difference
# register_spec core array dig
register_spec core array drop
# register_spec core array drop_while
# register_spec core array dup
register_spec core array each_index
register_spec core array each
# register_spec core array element_reference
# register_spec core array element_set
register_spec core array empty
# register_spec core array eql
# register_spec core array equal_value
# register_spec core array fetch
# register_spec core array fill
# register_spec core array filter
# register_spec core array find_index
# register_spec core array first
# register_spec core array flatten
register_spec core array frozen
# register_spec core array hash
register_spec core array include
# register_spec core array index
# register_spec core array initialize
# register_spec core array insert
# register_spec core array inspect
# register_spec core array intersection
# register_spec core array join
# register_spec core array keep_if
register_spec core array last
register_spec core array length
register_spec core array map
# register_spec core array max
# register_spec core array min
# register_spec core array minus
# register_spec core array multiply
# register_spec core array new
# register_spec core array partition
# register_spec core array permutation
register_spec core array plus
# register_spec core array pop
register_spec core array prepend
# register_spec core array product
register_spec core array push
register_spec core array rassoc
# register_spec core array reject
# register_spec core array repeated_combination
# register_spec core array repeated_permutation
register_spec core array replace
register_spec core array reverse_each
register_spec core array reverse
# register_spec core array rindex
# register_spec core array rotate
# register_spec core array sample
# register_spec core array select
register_spec core array shift
# register_spec core array shuffle
register_spec core array size
# register_spec core array slice
register_spec core array sort_by
# register_spec core array sort
# register_spec core array sum
# register_spec core array take
# register_spec core array take_while
# register_spec core array to_a
register_spec core array to_ary
# register_spec core array to_h
# register_spec core array to_s
# register_spec core array transpose
register_spec core array try_convert
# register_spec core array union
# register_spec core array uniq
register_spec core array unshift
# register_spec core array values_at
# register_spec core array zip

register_spec core comparable
register_spec core matchdata
register_spec core regexp
register_spec core string scan

register_spec library monitor
register_spec library stringscanner
register_spec library uri

if [[ $# -eq 1 ]]; then
  if [[ $1 == "--ruby" ]]; then
    run_specs_ruby
  elif [[ $1 == "--artichoke" ]]; then
    cargo build
    run_specs_artichoke
  else
    echo 1>&2 "Usage: $0 [ --artichoke | --ruby ] [ --with-timings ]"
    exit 1
  fi
elif [[ $# -eq 2 ]]; then
  if [[ $1 == "--ruby" && $2 == "--with-timings" ]]; then
    run_specs_ruby --with-timings
  elif [[ $1 == "--artichoke" ]]; then
    cargo build
    run_specs_artichoke --with-timings
  else
    echo 1>&2 "Usage: $0 [ --artichoke | --ruby ] [ --with-timings ]"
    exit 1
  fi
else
  echo 1>&2 "Usage: $0 [ --artichoke | --ruby ] [ --with-timings ]"
  exit 1
fi
