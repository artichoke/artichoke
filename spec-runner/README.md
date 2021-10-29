# spec-runner

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)

`spec-runner` is the [ruby/spec][ruby-spec] runner for Artichoke.

`spec-runner` is a wrapper around [`MSpec`][mspec-sources] and [vendored
ruby/spec sources][ruby-spec-sources] that works with the Artichoke virtual file
system. `spec-runner` runs the specs declared in a [manifest file].

## Spec Manifest

`spec-runner` is invoked with a YAML manifest that specifies which specs to run.
The manifest can run whole suites, like all of the `StringScanner` specs, or
specific specs, like the `Array#drop` spec. The manifest supports marking specs
as skipped.

```toml
[specs.core.array]
include = "set"
specs = [
  "any",
  "append",
  "drop",
]

[specs.library.stringscanner]
include = "all"

[specs.library.time]
include = "none"

[specs.library.uri]
include = "all"
skip = ["parse"]
```

## Usage

```console
$ cargo run -q -p spec-runner -- --help
spec-runner 0.3.0
ruby/spec runner for Artichoke.

USAGE:
    spec-runner <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <config>    Path to TOML config file
```

[ruby-spec]: https://github.com/ruby/spec
[mspec-sources]: vendor/mspec
[ruby-spec-sources]: vendor/spec
[manifest file]: enforced-specs.toml
