#!/usr/bin/env bash

set -euo pipefail

if command -v yarn; then
  yarn install &>/dev/null
  PATH="$(yarn bin):$PATH"
  export PATH
  cd "$(pkg-dir)"
fi

set -x

# ./scripts/run-spec.sh core array allocate
./scripts/run-spec.sh core array any
./scripts/run-spec.sh core array append
./scripts/run-spec.sh core array array
./scripts/run-spec.sh core array assoc
./scripts/run-spec.sh core array at
# ./scripts/run-spec.sh core array bsearch
# ./scripts/run-spec.sh core array bsearch_index
./scripts/run-spec.sh core array clear
# ./scripts/run-spec.sh core array clone
./scripts/run-spec.sh core array collect
./scripts/run-spec.sh core array combination
./scripts/run-spec.sh core array compact
# ./scripts/run-spec.sh core array comparison
# ./scripts/run-spec.sh core array concat
./scripts/run-spec.sh core array count
./scripts/run-spec.sh core array cycle
./scripts/run-spec.sh core array delete_at
./scripts/run-spec.sh core array delete_if
./scripts/run-spec.sh core array delete
# ./scripts/run-spec.sh core array difference
# ./scripts/run-spec.sh core array dig
./scripts/run-spec.sh core array drop
# ./scripts/run-spec.sh core array drop_while
# ./scripts/run-spec.sh core array dup
./scripts/run-spec.sh core array each_index
./scripts/run-spec.sh core array each
# ./scripts/run-spec.sh core array element_reference
# ./scripts/run-spec.sh core array element_set
./scripts/run-spec.sh core array empty
# ./scripts/run-spec.sh core array eql
# ./scripts/run-spec.sh core array equal_value
# ./scripts/run-spec.sh core array fetch
# ./scripts/run-spec.sh core array fill
# ./scripts/run-spec.sh core array filter
# ./scripts/run-spec.sh core array find_index
# ./scripts/run-spec.sh core array first
# ./scripts/run-spec.sh core array flatten
./scripts/run-spec.sh core array frozen
# ./scripts/run-spec.sh core array hash
./scripts/run-spec.sh core array include
# ./scripts/run-spec.sh core array index
# ./scripts/run-spec.sh core array initialize
# ./scripts/run-spec.sh core array insert
# ./scripts/run-spec.sh core array inspect
# ./scripts/run-spec.sh core array intersection
# ./scripts/run-spec.sh core array join
# ./scripts/run-spec.sh core array keep_if
./scripts/run-spec.sh core array last
./scripts/run-spec.sh core array length
./scripts/run-spec.sh core array map
# ./scripts/run-spec.sh core array max
# ./scripts/run-spec.sh core array min
# ./scripts/run-spec.sh core array minus
# ./scripts/run-spec.sh core array multiply
# ./scripts/run-spec.sh core array new
# ./scripts/run-spec.sh core array partition
# ./scripts/run-spec.sh core array permutation
./scripts/run-spec.sh core array plus
# ./scripts/run-spec.sh core array pop
./scripts/run-spec.sh core array prepend
# ./scripts/run-spec.sh core array product
./scripts/run-spec.sh core array push
./scripts/run-spec.sh core array rassoc
# ./scripts/run-spec.sh core array reject
# ./scripts/run-spec.sh core array repeated_combination
# ./scripts/run-spec.sh core array repeated_permutation
./scripts/run-spec.sh core array replace
./scripts/run-spec.sh core array reverse_each
./scripts/run-spec.sh core array reverse
# ./scripts/run-spec.sh core array rindex
# ./scripts/run-spec.sh core array rotate
# ./scripts/run-spec.sh core array sample
# ./scripts/run-spec.sh core array select
./scripts/run-spec.sh core array shift
# ./scripts/run-spec.sh core array shuffle
./scripts/run-spec.sh core array size
# ./scripts/run-spec.sh core array slice
./scripts/run-spec.sh core array sort_by
# ./scripts/run-spec.sh core array sort
# ./scripts/run-spec.sh core array sum
# ./scripts/run-spec.sh core array take
# ./scripts/run-spec.sh core array take_while
# ./scripts/run-spec.sh core array to_a
./scripts/run-spec.sh core array to_ary
# ./scripts/run-spec.sh core array to_h
# ./scripts/run-spec.sh core array to_s
# ./scripts/run-spec.sh core array transpose
./scripts/run-spec.sh core array try_convert
# ./scripts/run-spec.sh core array union
# ./scripts/run-spec.sh core array uniq
./scripts/run-spec.sh core array unshift
# ./scripts/run-spec.sh core array values_at
# ./scripts/run-spec.sh core array zip

./scripts/run-spec.sh core comparable
./scripts/run-spec.sh core matchdata
./scripts/run-spec.sh core regexp

./scripts/run-spec.sh library monitor
./scripts/run-spec.sh library stringscanner
./scripts/run-spec.sh library uri
