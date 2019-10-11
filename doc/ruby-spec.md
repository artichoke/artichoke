# Ruby Spec

artichoke embeds a copy of [ruby/spec](/spec-runner/vendor/spec). ruby/spec is a
set of specifications for testing the Ruby language, core, and standard library
packages.

## Running Specs

You can run these specs for mruby crate with `spec-runner`. For example, to run
the specs for the `uri` stdlib package:

```shell
./scripts/run-spec.sh library uri
```

Or to run the specs for `Array#each`:

```shell
./scripts/run-spec.sh core array each
```

## Regression Testing

Once a spec suite passes, add it to
[`scripts/spec-compliance.sh`](/scripts/spec-compliance.sh). This script is run
as part of CI and will ensure that a suite that does pass continues to pass.

### Currently Passing Specs

| Type    | Suite           |
| ------- | --------------- |
| Core    | `Comparable`    |
| Core    | `MatchData`     |
| Core    | `Regexp`        |
| Library | `Monitor`       |
| Library | `StringScanner` |
| Library | `URI`           |
