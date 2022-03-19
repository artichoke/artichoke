# scolapasta-hex

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-hex.svg)](https://crates.io/crates/scolapasta-hex)
[![API](https://docs.rs/scolapasta-hex/badge.svg)](https://docs.rs/scolapasta-hex)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_hex/)

Functions for encoding sequences of bytes into base 16 hex encoding.

[Base 16 encoding] is an encoding scheme that uses a 16 character ASCII alphabet
for encoding arbitrary octets.

This crate offers encoders that:

- Allocate and return a [`String`]: `try_encode`.
- Encode into an already allocated [`String`]: `try_encode_into`.
- Encode into a [`fmt::Write`]: `format_into`.
- Encode into a [`io::Write`]: `write_into`.

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-hex = "0.2.0"
```

Hex encode data like:

```rust
let data = b"Artichoke Ruby";
let mut buf = String::new();
let _ignored = scolapasta_hex::try_encode_into(data, &mut buf);
assert_eq!(buf, "4172746963686f6b652052756279");
```

This module also exposes an iterator:

```rust
use scolapasta_hex::Hex;

let data = "Artichoke Ruby";
let iter = Hex::from(data);
assert_eq!(iter.collect::<String>(), "4172746963686f6b652052756279");
```

## `no_std`

This crate is `no_std` compatible when built without the `std` feature. This
crate optionally depends on [`alloc`] when the `alloc` feature is enabled.

When this crate depends on `alloc`, it exclusively uses fallible allocation
APIs. The APIs in this crate will never abort due to allocation failure or
capacity overflows. Note that writers given to `format_into` and `write_into`
may have abort on allocation failure behavior.

## Crate features

All features are enabled by default.

- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables APIs that require [`std::io::Write`]. Activating this feature
  also activates the **alloc** feature.
- **alloc** - Enables a dependency on the Rust [`alloc`] crate. Activating this
  feature enables APIs that require [`alloc::string::String`].

## License

`scolapasta-hex` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[base 16 encoding]: https://tools.ietf.org/html/rfc4648#section-8
[`string`]: https://doc.rust-lang.org/alloc/string/struct.String.html
[`fmt::write`]: https://doc.rust-lang.org/core/fmt/trait.Write.html
[`io::write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`std::io::write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`alloc`]: https://doc.rust-lang.org/alloc/
[`alloc::string::string`]:
  https://doc.rust-lang.org/alloc/string/struct.String.html
