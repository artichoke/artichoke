# scolapasta-aref

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-aref.svg)](https://crates.io/crates/scolapasta-aref)
[![API](https://docs.rs/scolapasta-aref/badge.svg)](https://docs.rs/scolapasta-aref)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_aref/)

Functions for working with Ruby containers that respond to `#[]` or "aref".

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-aref = "0.1.0"
```

Convert offsets to `usize` indexes like this:

```rust
let data = "ABC, 123, XYZ";
let offset = -5;
let index = scolapasta_aref::offset_to_index(offset, data.len());
assert_eq!(index, Some(8))
```

## `no_std`

This crate is `no_std` compatible. This crate does not depend on [`alloc`].

## License

`scolapasta-aref` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`alloc`]: https://doc.rust-lang.org/alloc/
