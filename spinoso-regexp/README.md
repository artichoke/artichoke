# spinoso-regexp

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-regexp.svg)](https://crates.io/crates/spinoso-regexp)
[![API](https://docs.rs/spinoso-regexp/badge.svg)](https://docs.rs/spinoso-regexp)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_regexp/)

The Ruby Regexp class.

A Regexp holds a regular expression, used to match a pattern against strings.
Regexps are created using the `/.../` and `%r{...}` literals, and by the `::new`
constructor.

`spinoso-regexp` includes several `Regexp` implementations with support for
multiple underlying regex engines, including the Rust [`regex` crate] and
[Oniguruma].

[`regex` crate]: https://docs.rs/regex
[oniguruma]: https://github.com/kkos/oniguruma

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-regexp = "0.3.0"
```

## Crate features

All features are enabled by default.

- **oniguruma** - Enables a `Regexp` backend based on the [`onig` crate] and the
  Oniguruma regex engine.
- **regex-full** - Enables the **regex-perf** and **regex-unicode** features.
  These features impact the `regex` crate engine.
- **regex-perf** - Enables the **perf** feature in the `regex` crate.
- **regex-unicode** - Enables the **unicode** feature in the `regex` crate.

[`onig` crate]: https://docs.rs/onig

## License

`spinoso-regex` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.
