# scolapasta-path

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-path.svg)](https://crates.io/crates/scolapasta-path)
[![API](https://docs.rs/scolapasta-path/badge.svg)](https://docs.rs/scolapasta-path)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_path/)

Functions for working with filesystem paths and loading Ruby source code.

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-path = "0.5.0"
```

And check for explicit relative paths like:

```rust
use scolapasta_path::is_explicit_relative;

assert!(is_explicit_relative("./test/loader"));
assert!(is_explicit_relative("../rake/test_task"));

assert!(!is_explicit_relative("json/pure"));
assert!(!is_explicit_relative("/artichoke/src/json/pure"));
```

## License

`scolapasta-path` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`alloc`]: https://doc.rust-lang.org/alloc/
