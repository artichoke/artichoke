# spec-runner

[![CircleCI](https://circleci.com/gh/artichoke/artichoke.svg?style=svg)](https://circleci.com/gh/artichoke/artichoke)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Spec runner documentation](https://img.shields.io/badge/docs-spec--runner-blue.svg)](https://artichoke.github.io/artichoke/spec_runner/)

`spec-runner` is a binary crate that produces the `spec-runner` executable.

`spec-runner` is a wrapper around MSpec and ruby/spec that works with the
Artichoke virtual filesystem.

`spec-runner` is invokable directly by passing paths to spec files as command
line arguments. `spec-runner` is sensitive to CWD relative to the specs it
wraps, so in practice it is easier to invoke `spec-runner` via the `spec.rb`
wrapper in `scripts`.

```console
$ ruby scripts/spec.rb --help
spec.rb runs ruby/specs against Artichoke and MRI.

Usage: scripts/spec.rb artichoke [ --timed ITERATIONS | --profile ] [ passing | family [ component [ spec ] ] ]
Usage: scripts/spec.rb ruby [ --timed ITERATIONS ] family [ component [ spec ] ]

Examples:
    $ scripts/spec.rb artichoke passing
    $ scripts/spec.rb artichoke core
    $ scripts/spec.rb artichoke core string
    $ scripts/spec.rb ruby core string scan
    $ scripts/spec.rb artichoke --timed 30 core string scan
    $ scripts/spec.rb artichoke --profile passing
```
