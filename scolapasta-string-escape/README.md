# scolapasta-string-escape

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-string-escape.svg)](https://crates.io/crates/scolapasta-string-escape)
[![API](https://docs.rs/scolapasta-string-escape/badge.svg)](https://docs.rs/scolapasta-string-escape)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_string_escape/)

Routines for debug escaping Ruby Strings.

Ruby Strings are conventionally UTF-8 byte sequences. When calling
[`String#inspect`] or [`Symbol#inspect`], these maybe UTF-8 byte strings are
escaped to have a valid UTF-8 representation.

This crate exposes functions and iterators for encoding arbitrary byte slices as
valid, printable UTF-8.

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Ruby debug escapes

Ruby produces debug escapes that look like:

```console
[2.6.3] > "Artichoke Ruby is made with Rust.

Invalid UTF-8: \xFF.

Slash \\ and quote \" are escaped."
=> "Artichoke Ruby is made with Rust.\n\nInvalid UTF-8: \xFF.\n\nSlash \\ and quote \" are escaped."
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-string-escape = "0.3.0"
```

To debug escape a conventionally UTF-8 byte string:

```rust
use scolapasta_string_escape::format_debug_escape_into;

const EXAMPLE: &[u8] = b"Artichoke Ruby is made with Rust.

Invalid UTF-8: \xFF.

Slash \\ and quote \" are escaped.";

fn example() -> Result<(), core::fmt::Error> {
    let mut escaped = String::new();
    format_debug_escape_into(&mut escaped, EXAMPLE)?;
    assert_eq!(
        escaped,
        r#"Artichoke Ruby is made with Rust.\n\nInvalid UTF-8: \xFF.\n\nSlash \\ and quote \" are escaped."#,
    );
    Ok(())
}
```

This crate exposes low level utilities for accessing escape internals. To escape
a single character:

```rust
use scolapasta_string_escape::Literal;

let literal = Literal::from(b'a');
assert_eq!(literal.collect::<String>(), "a");

let literal = Literal::from(b'\\');
assert_eq!(literal.as_str(), r"\\");

let literal = Literal::debug_escape(b'\0');
assert_eq!(literal, r"\x00");

let literal = Literal::from(b'\x0A');
assert_eq!(literal.as_str(), r"\n");

let literal = Literal::from(b'\x0C');
assert_eq!(literal.collect::<String>(), r"\f");
```

## `no_std`

This crate is `no_std`. This crate does not depend on [`alloc`].

## License

`scolapasta-string-escape` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.

[`string#inspect`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-inspect
[`symbol#inspect`]: https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-inspect
[`alloc`]: https://doc.rust-lang.org/alloc/
