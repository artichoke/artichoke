# artichoke-core

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Core documentation](https://img.shields.io/badge/docs-artichoke--core-blue.svg)](https://artichoke.github.io/artichoke/artichoke_core/)

`artichoke-core` crate provides a set of traits that, when implemented, provide
a complete Ruby interpreter.

[`artichoke-backend`](../artichoke-backend) is one implementation of the
`artichoke-core` traits.

## Core APIs

`artichoke-core` contains traits for the core set of APIs an interpreter must
implement. The traits in `artichoke-core` define:

- APIs a concrete VM must implement to support the Artichoke runtime and
  frontends.
- How to box polymorphic core types into
  [Ruby `Value`](https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html).
- [Interoperability](https://artichoke.github.io/artichoke/artichoke_core/convert/index.html)
  between the VM backend and the Rust-implemented core.

Some of the core APIs a Ruby implementation must provide are
[evaluating code](https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html),
[converting Rust data structures to boxed `Value`s on the interpreter heap](https://artichoke.github.io/artichoke/artichoke_core/convert/trait.ConvertMut.html),
and
[interning `Symbol`s](https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html).

## License

artichoke-core is licensed with the [MIT License](../LICENSE) (c) Ryan Lopopolo.
