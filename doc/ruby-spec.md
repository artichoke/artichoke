# Ruby Spec

artichoke embeds a copy of [ruby/spec](/spec-runner/vendor/spec). ruby/spec is a
set of specifications for testing the Ruby language, core, and standard library
packages.

Artichoke enforces that some ruby/specs pass. These specs are tracked in
[`spec-runner/enforced-specs.yaml`](/spec-runner/enforced-specs.yaml).

## Running Specs

You can run these specs for Artichoke crate with the `spec-runner` crate.

The commands exposed by this runner support running for `artichoke` or `ruby`.

### Running the enforced specs

```shell
ruby scripts/spec.rb artichoke passing
```

### Running specific specs

For the `uri` stdlib package:

```shell
ruby scripts/spec.rb artichoke library uri
```

For `Array#each`:

```shell
ruby scripts/spec.rb artichoke core array each
```

### Performance testing

#### Timing

```shell
ruby scripts/spec.rb artichoke --timed 30 passing
```

#### Profiling

```shell
ruby scripts/spec.rb artichoke --profile passing
```

This script requires the `cargo-flamegraph` binary, which can be installed with:

```shell
cargo install flamegraph
```

## Regression Testing

Once a spec suite passes, add it to
[`spec-runner/enforced-specs.yaml`](/spec-runner/enforced-specs.yaml). This
script is run as part of CI and will ensure that a suite that does pass
continues to pass.
