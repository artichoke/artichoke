# spec-runner

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Spec runner documentation](https://img.shields.io/badge/docs-spec--runner-blue.svg)](https://artichoke.github.io/artichoke/spec_runner/)

`spec-runner` is the ruby/spec runner for Artichoke.

`spec-runner` is a wrapper around `MSpec` and ruby/spec that works with the
Artichoke virtual filesystem. `spec-runner` runs the specs declared in a
manifest file.

## Spec Manifest

`spec-runner` is invoked with a YAML manifest that specifies which specs to run.
The manifest can run whole suites, like all of the `StringScanner` specs, or
specific specs, like the `Array#drop` spec. The manifest supports marking specs
as skipped.

```yaml
core:
  - suite: array
    specs:
      - any
      - append
      - array
  - suite: comparable
  - suite: string
    specs:
      - scan
library:
  - suite: stringscanner
  - suite: uri
    skip:
      - parse
```

## Usage

```console
$ cargo run -q -p spec-runner -- --help
spec-runner 0.1.0
ruby/spec runner for Artichoke.

USAGE:
    spec-runner <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <config>    Path to YAML config file
```
