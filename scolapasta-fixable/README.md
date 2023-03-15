# scolapasta-fixable

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-fixable.svg)](https://crates.io/crates/scolapasta-fixable)
[![API](https://docs.rs/scolapasta-fixable/badge.svg)](https://docs.rs/scolapasta-fixable)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_fixable/)

Functions for converting numeric immediates to integer or "fixnum" immediates.

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-fixable = "0.1.0"
```

## License

`scolapasta-fixable` is licensed under the [MIT License](LICENSE) (c) Ryan Lopopolo.

This repository includes a vendored copy of the arithmetic headers from Ruby
3.2.0, which is licensed under the [Ruby license] or [BSD 2-clause license]. See
[`vendor/README.md`] for more details. These sources are not distributed on
[crates.io].

[ruby license]: vendor/ruby-3.2.0/COPYING
[bsd 2-clause license]: vendor/ruby-3.2.0/BSDL
[`vendor/readme.md`]: vendor/README.md
[crates.io]: https://crates.io/
