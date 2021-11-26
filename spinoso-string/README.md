# spinoso-string

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-string.svg)](https://crates.io/crates/spinoso-string)
[![API](https://docs.rs/spinoso-string/badge.svg)](https://docs.rs/spinoso-string)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_string/)

The Ruby String class.

A String object holds and manipulates an arbitrary sequence of bytes, typically
representing characters. String objects may be created using `::new` or as
literals.

`spinoso-string` is encoding-aware and implements support for UTF-8, ASCII, and
binary encodings.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-string = "0.10"
```

## `no_std`

This crate is `no_std` compatible with a required dependency on [`alloc`].

[`alloc`]: https://doc.rust-lang.org/alloc/

## Crate features

All features are enabled by default.

- **casecmp** - Enables ASCII and Unicode `casecmp` methods on `String`.
  Activating this feature enables a dependency on [`focaccia`].
- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables [`std::error::Error`] impls on error types in this crate.

[`focaccia`]: https://docs.rs/focaccia
[`std::error::error`]: https://doc.rust-lang.org/std/error/trait.Error.html

## License

`spinoso-regex` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.
