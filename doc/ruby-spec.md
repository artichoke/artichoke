# Ruby Spec

artichoke embeds a copy of [ruby/spec](/spec-runner/spec/ruby). ruby/spec is a
set of specifications for testing the Ruby language, core, and standard library
packages.

## Running Specs

You can run these specs for mruby crate with `spec-runner`. For example, to run
the specs for the `uri` stdlib package:

```shell
shopt -s globstar
cd spec-runner/spec/ruby/library
cargo run --bin spec-runner uri/**/*.rb
```

If all of the specs provided as command line arguments pass, you will see the
list of successful specs. If any spec fails, spec-runner will exit with a
non-zero exit code and print a list of spec failures and backtraces.

For example, before
[`87678cc`](https://github.com/artichoke/artichoke/commit/87678ccd39afba73876290690b36e3e9fa051b8a)
landed, `uri` was failing one spec related to private method visibility. The
failure is reported like this:

```text
Passed 204, skipped 0, not implemented 0, failed 1 specs.

Expected NoMethodError
but no exception was raised (#<URI::HTTP http://ruby-lang.org/> was returned) in does not add a URI method to Object instances

/src/lib/uri/uri_spec.rb:24:in protect
/Users/lopopolo/dev/repos/artichoke/target/debug/build/mruby-sys-a21763ba2a043d0b/out/mruby-1685c45/mrbgems/mruby-enum-ext/mrblib/enum.rb:319:in all?
/Users/lopopolo/dev/repos/artichoke/target/debug/build/mruby-sys-a21763ba2a043d0b/out/mruby-1685c45/mrblib/array.rb:13:in each
/Users/lopopolo/dev/repos/artichoke/target/debug/build/mruby-sys-a21763ba2a043d0b/out/mruby-1685c45/mrbgems/mruby-enum-ext/mrblib/enum.rb:319:in all?
/Users/lopopolo/dev/repos/artichoke/target/debug/build/mruby-sys-a21763ba2a043d0b/out/mruby-1685c45/mrblib/numeric.rb:46:in times
/Users/lopopolo/dev/repos/artichoke/target/debug/build/mruby-sys-a21763ba2a043d0b/out/mruby-1685c45/mrblib/array.rb:13:in each
/src/lib/uri/uri_spec.rb:26
/Users/lopopolo/dev/repos/artichoke/target/debug/build/mruby-sys-a21763ba2a043d0b/out/mruby-1685c45/mrblib/array.rb:13:in each
/src/test/spec_runner:125:in run_specs
Passed 204, skipped 0, not implemented 0, failed 1 specs.
```

## Regression Testing

Once a spec suite passes, add it to
[`scripts/spec-compliance.sh`](/scripts/spec-compliance.sh). This script is run
as part of CI and will ensure that a suite that does pass continues to pass.

### Currently Passing Specs

| Type    | Suite           |
| ------- | --------------- |
| Core    | `MatchData`     |
| Core    | `Regexp`        |
| Library | `Monitor`       |
| Library | `StringScanner` |
| Library | `URI`           |
