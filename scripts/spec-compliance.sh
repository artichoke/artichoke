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

cargo build
spec_runner="$(pwd)/target/debug/spec-runner"

run_spec() {
  if [[ $# -eq 2 ]]; then
    family="$1"
    component="$2"
    pushd "spec-runner/vendor/spec" >/dev/null
    $spec_runner ./**/shared/**/*.rb ./**/fixtures/**/*.rb "./$family/$component/"*_spec.rb
    popd
  elif [[ $# -eq 3 ]]; then
    family="$1"
    component="$2"
    spec="$3"
    pushd "spec-runner/vendor/spec" >/dev/null
    $spec_runner ./**/shared/**/*.rb ./**/fixtures/**/*.rb "./$family/$component/${spec}_spec.rb"
    popd
  else
    echo 1>&2 "Usage: $0 language|core|library component [spec]"
  fi
}

# run_spec core array allocate
run_spec core array any
run_spec core array append
run_spec core array array
run_spec core array assoc
run_spec core array at
# run_spec core array bsearch
# run_spec core array bsearch_index
run_spec core array clear
# run_spec core array clone
run_spec core array collect
run_spec core array combination
run_spec core array compact
# run_spec core array comparison
# run_spec core array concat
run_spec core array count
run_spec core array cycle
run_spec core array delete_at
run_spec core array delete_if
run_spec core array delete
# run_spec core array difference
# run_spec core array dig
run_spec core array drop
# run_spec core array drop_while
# run_spec core array dup
run_spec core array each_index
run_spec core array each
# run_spec core array element_reference
# run_spec core array element_set
run_spec core array empty
# run_spec core array eql
# run_spec core array equal_value
# run_spec core array fetch
# run_spec core array fill
# run_spec core array filter
# run_spec core array find_index
# run_spec core array first
# run_spec core array flatten
run_spec core array frozen
# run_spec core array hash
run_spec core array include
# run_spec core array index
# run_spec core array initialize
# run_spec core array insert
# run_spec core array inspect
# run_spec core array intersection
# run_spec core array join
# run_spec core array keep_if
run_spec core array last
run_spec core array length
run_spec core array map
# run_spec core array max
# run_spec core array min
# run_spec core array minus
# run_spec core array multiply
# run_spec core array new
# run_spec core array partition
# run_spec core array permutation
run_spec core array plus
# run_spec core array pop
run_spec core array prepend
# run_spec core array product
run_spec core array push
run_spec core array rassoc
# run_spec core array reject
# run_spec core array repeated_combination
# run_spec core array repeated_permutation
run_spec core array replace
run_spec core array reverse_each
run_spec core array reverse
# run_spec core array rindex
# run_spec core array rotate
# run_spec core array sample
# run_spec core array select
run_spec core array shift
# run_spec core array shuffle
run_spec core array size
# run_spec core array slice
run_spec core array sort_by
# run_spec core array sort
# run_spec core array sum
# run_spec core array take
# run_spec core array take_while
# run_spec core array to_a
run_spec core array to_ary
# run_spec core array to_h
# run_spec core array to_s
# run_spec core array transpose
run_spec core array try_convert
# run_spec core array union
# run_spec core array uniq
run_spec core array unshift
# run_spec core array values_at
# run_spec core array zip

run_spec core comparable
run_spec core matchdata
run_spec core regexp

run_spec library monitor
run_spec library stringscanner
run_spec library uri
