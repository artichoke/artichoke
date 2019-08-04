#!/usr/bin/env bash

set -euo pipefail
set -x

bench_artichoke() {
  if [[ $# -gt 0 ]]; then
    if [[ $1 == "--ascii" ]]; then
      cargo run --release --bin string_scan_bench -- artichoke-frontend/ruby/benches/string_scan.rb --ascii
    elif [[ $1 == "--utf-8" ]]; then
      cargo run --release --bin string_scan_bench -- artichoke-frontend/ruby/benches/string_scan.rb
    else
      echo >&2 "Usage: bench_artichoke [ --ascii | --utf-8 ]"
    fi
  else
    echo >&2 "Usage: bench_artichoke [ --ascii | --utf-8 ]"
  fi
}

bench_mri() {
  if [[ $# -gt 0 ]]; then
    if [[ $1 == "--ascii" ]]; then
      ruby artichoke-frontend/ruby/benches/string_scan.rb --ascii
    elif [[ $1 == "--utf-8" ]]; then
      ruby artichoke-frontend/ruby/benches/string_scan.rb
    else
      echo >&2 "Usage: bench_mri [ --ascii | --utf-8 ]"
    fi
  else
    echo >&2 "Usage: bench_mri [ --ascii | --utf-8 ]"
  fi
}

mkdir -p target/bench/regexp

if [[ $# -gt 0 ]]; then
  if [[ $1 == "--artichoke" ]]; then
    bench_artichoke --utf-8 >target/bench/regexp/artichoke.utf-8.txt
    bench_artichoke --ascii >target/bench/regexp/artichoke.ascii.txt
  elif [[ $1 == "--mri" ]]; then
    bench_mri --utf-8 >target/bench/regexp/mri.utf-8.txt
    bench_mri --ascii >target/bench/regexp/mri.ascii.txt
  else
    echo >&2 "Usage: $0 [ --artichoke | --mri ]"
  fi
else
  echo >&2 "Usage: $0 [ --artichoke | --mri ]"
fi
