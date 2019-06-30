#!/usr/bin/env bash

set -euo pipefail
set -x

scratch="$(mktemp -d)"
pushd "$scratch"
pid="$(pgrep "[r]ruby")"

sudo dtrace -x ustackframes=100 -n "profile-99 /pid == $pid/ { @[ustack()] = count(); } tick-10s { exit(0); }" -o out.stacks

inferno-collapse-dtrace <out.stacks >stacks.folded
inferno-flamegraph <stacks.folded >flamegraph.svg

open -a "Google Chrome.app" flamegraph.svg
echo "$(pwd)/flamegraph.svg"
