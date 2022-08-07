# scolapasta-int-parse

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-int-parse.svg)](https://crates.io/crates/scolapasta-int-parse)
[![API](https://docs.rs/scolapasta-int-parse/badge.svg)](https://docs.rs/scolapasta-int-parse)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_int_parse/)

Functions for parsing a byte string as an integer with an optional radix.

This crate can be used to implement the Ruby API [`Kernel#Integer`]. Input byte
strings are normalized before delegating to [`i64::from_str_radix`] in Rust
core.

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-int-parse = "0.1.0"
```

Parse strings into integers like:

```rust
use scolapasta_int_parse::{parse, ArgumentError, Radix};

fn example() -> Result<(), ArgumentError<'static>> {
    let int_max = parse("9_223_372_036_854_775_807", None)?;
    assert_eq!(int_max, i64::MAX);

    let deadbeef = parse("                       0x000000000deadbeef", None)?;
    assert_eq!(deadbeef, 3_735_928_559);

    let num = parse("32xyz", Radix::new(36))?;
    assert_eq!(num, 5_176_187);

    Ok(())
}
```

## `no_std`

This crate is `no_std` compatible when built without the `std` feature. This
crate has a required dependency on [`alloc`].

## Crate features

All features are enabled by default.

- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables implementations of [`std::error::Error`] on error types in
  this crate.

## License

`scolapasta-int-parse` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.

[`kernel#integer`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-Integer
[`i64::from_str_radix`]:
  https://doc.rust-lang.org/std/primitive.i64.html#method.from_str_radix
[`std::error::error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`alloc`]: https://doc.rust-lang.org/alloc/
