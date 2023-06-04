# scolapasta-strbuf

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-strbuf.svg)](https://crates.io/crates/scolapasta-strbuf)
[![API](https://docs.rs/scolapasta-strbuf/badge.svg)](https://docs.rs/scolapasta-strbuf)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_strbuf/)

A contiguous growable byte string, written as `Buf`, short for 'buffer'.

Buffers have _O_(1) indexing, amortized _O_(1) push (to the end) and _O_(1) pop
(from the end).

Buffers ensure they never allocate more than `isize::MAX` bytes.

Buffers are transparent wrappers around `Vec<u8>` with a minimized API
sufficient for implementing the Ruby [`String`] type.

Buffers do not assume any encoding. Encoding is a higher-level concept that
should be built on top of `Buf`.

[`String`]: https://ruby-doc.org/3.2.0/String.html

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-strbuf = "1.0.0"
```

And manipulate byte buffers like:

```rust
use scolapasta_strbuf::Buf;

let mut buf = Buf::from(b"Artichoke Ruby");
buf.push_char('!');

assert_eq!(buf, "Artichoke Ruby!");
```

## Crate Features

This crate has a required dependency on [`alloc`].

- **std**: Enabled by default. Implement [`std::io::Write`] for `Buf`. If this
  feature is disabled, this crate only depends on [`alloc`].
- **nul-terminated**: Use an alternate byte buffer backend that ensures byte
  content is always followed by a NUL byte in the buffer's spare capacity. This
  feature can be used to ensure `Buf`s are FFI compatible with C code that
  expects byte content to be NUL terminated.

[`alloc`]: https://doc.rust-lang.org/alloc/
[`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

## License

`scolapasta-strbuf` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.
