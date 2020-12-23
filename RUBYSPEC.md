# Ruby Spec

Artichoke embeds a copy of [ruby/spec][ruby-spec-sources].
[ruby/spec][ruby-spec] is a set of specifications for testing the Ruby language,
core, and standard library packages.

Artichoke enforces that some ruby/specs pass. These specs are tracked in
[`spec-runner/enforced-specs.toml`].

## Running Specs

You can run these specs for Artichoke crate with the `spec-runner` crate.

### Running the enforced specs

```shell
bundle exec rake spec
```

### Running specific specs

To run specific specs, create a custom spec manifest and pass it as the
positional argument to the `spec-runner` binary.

For the `uri` stdlib package:

```toml
[specs.library.uri]
include = "all"
skip = ["parse"]
```

For `Array#each` and `Array#length`:

```toml
[specs.core.array]
include = "set"
specs = [
  "each",
  "length",
]
```

[ruby-spec-sources]: spec-runner/vendor/spec
[ruby-spec]: https://github.com/ruby/spec
[`spec-runner/enforced-specs.toml`]: spec-runner/enforced-specs.toml
