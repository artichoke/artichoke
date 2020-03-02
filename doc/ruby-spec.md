# Ruby Spec

artichoke embeds a copy of [ruby/spec](/spec-runner/vendor/spec). ruby/spec is a
set of specifications for testing the Ruby language, core, and standard library
packages.

Artichoke enforces that some ruby/specs pass. These specs are tracked in
[`spec-runner/enforced-specs.yaml`](/spec-runner/enforced-specs.yaml).

## Running Specs

You can run these specs for Artichoke crate with the `spec-runner` crate.

### Running the enforced specs

```shell
cargo run -q --bin spec-runner -- spec-runner/enforced-specs.yaml
```

### Running specific specs

To run specific specs, create a custom spec manifest and pass it as the
positional argument to the `spec-runner` binary.

For the `uri` stdlib package:

```yaml
library:
  - suite: uri
```

For `Array#each`:

```yaml
core:
  - suite: array
    specs:
      - each
```
