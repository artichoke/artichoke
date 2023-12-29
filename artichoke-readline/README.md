# artichoke-readline

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/artichoke-readline.svg)](https://crates.io/crates/artichoke-readline)
[![API](https://docs.rs/artichoke-readline/badge.svg)](https://docs.rs/artichoke-readline)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/artichoke_readline/)

Helpers for integrating REPLs with GNU Readline.

This crate can be used to parse Readline editing mode from the standard set of
GNU Readline config files.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
artichoke-readline = "1.1.0"
```

And parse Readline editing mode like this:

```rust
use artichoke_readline::{get_readline_edit_mode, rl_read_init_file, EditMode};

if let Some(config) = rl_read_init_file() {
    if matches!(get_readline_edit_mode(config), Some(EditMode::Vi)) {
        println!("You have chosen wisely");
    }
}
```

## Crate Features

The **rustyline** feature (enabled by default) adds trait implementations to
allow `EditMode` to interoperate with the corresponding enum in the `rustyline`
crate.

`rustyline` major version upgrades can be made in minor version bumps.

## License

`artichoke-readline` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.
