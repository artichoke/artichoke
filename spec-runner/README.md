# spec-runner

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)

`spec-runner` is the [ruby/spec][ruby-spec] runner for Artichoke.

[ruby-spec]: https://github.com/ruby/spec

`spec-runner` is a wrapper around [`MSpec`][mspec-sources] and [vendored
ruby/spec sources][ruby-spec-sources] that works with the Artichoke virtual file
system. `spec-runner` runs the specs declared in a [manifest file].

[mspec-sources]: vendor/mspec
[ruby-spec-sources]: vendor/spec
[manifest file]: enforced-specs.toml

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
spec-runner 0.6.0
ruby/spec runner for Artichoke.

USAGE:
    spec-runner [OPTIONS] [config]

ARGS:
    <config>    Path to TOML config file

OPTIONS:
    -f, --format <formatter>    Choose an output formatter [default: artichoke] [possible values:
                                artichoke, summary, tagger, yaml]
    -h, --help                  Print help information
    -q, --quiet                 Suppress spec failures when exiting
    -V, --version               Print version information
```

## Profiling

The `spec-runner` can be used as a profiling harness for Articoke Ruby using a
combination of Cargo features and a customized spec manifest file.

### Heap Profiling

Heap profiling with the Rust [`dhat`] crate was added in
[artichoke/artichoke#2042].

[`dhat`]: https://docs.rs/dhat/0.3.0/dhat/index.html
[artichoke/artichoke#2042]: https://github.com/artichoke/artichoke/pull/2042

To run a heap profile:

1. Create a spec manifest. If you'd like a general purpose workload, you can use
   [`all-core-specs.toml`].
2. Run `spec-runner` with Cargo:

   ```console
   $ cargo run --release --bin spec-runner -q --features dhat-heap -- --quiet path/to/spec-manifest.toml
   ```

3. If this worked, you'll see `dhat` print some summary statistics and output a
   file called `dhat-heap.json` to your shell's current working directory.
4. Navigate to the [online DHAT viewer] and click the `Load...` button to load
   the generated `dhat-heap.json` file. The `dhat` crate documentation has an
   explainer on [how to use this viewer].

[`all-core-specs.toml`]: all-core-specs.toml
[online dhat viewer]: https://nnethercote.github.io/dh_view/dh_view.html
[how to use this viewer]: https://docs.rs/dhat/0.3.0/dhat/index.html#viewing
