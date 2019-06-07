#!/usr/bin/env bash

set -euo pipefail
set -x

pushd foolsgold/ruby >/dev/null
bundle install
# without logging
bundle exec thin -a 127.0.0.1 -p 9000 --threaded --threadpool-size 16 -R ../benches/config.ru start
