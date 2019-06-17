# Ruby Spec

mruby crate embeds a copy of [ruby/spec](/mruby/src/extn/test/ruby-spec).
ruby/spec is a set of specifications for testing the Ruby language, core, and
standard library packages.

You can run these specs for mruby crate with `spec-runner`. For example, to run
the specs for the `uri` stdlib package:

```shell
shopt -s globstar
cd mruby/src/extn/test/ruby-spec/library
cargo run -p mruby --bin spec-runner uri/**/*.rb
```

If all of the specs provided as command line arguments pass, you will see the
list of successful specs. If any spec fails, spec-runner will panic and print a
list of spec failures and backtraces. For example, `strscan` is currently
failing one spec related to `String` encoding and the failure is reported like
this:

```text
$ cargo run -p mruby --bin spec-runner stringscanner/**/*.rb
mruby exception: /src/test/spec_runner:58:

1 spec failures:

RuntimeError: conversion error: failed to convert from ruby String to rust String in is multi-byte character sensitive

/src/lib/strscan.rb:241:in scan_full
/src/lib/strscan.rb:234:in scan
/src/lib/strscan.rb:115:in getch
/src/lib/stringscanner/getch_spec.rb:19:in protect
/src/lib/stringscanner/getch_spec.rb:35
/src/test/spec_runner:47:in run_specs
(eval):1

1 spec failures.
 (RuntimeError)
/src/test/spec_runner:58:in run_specs
(eval):1
thread 'main' panicked at 'assertion failed: !self.enforce', mruby/src/extn/test/mspec.rs:70:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
