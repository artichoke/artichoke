# artichoke-repl-history

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/artichoke-repl-history.svg)](https://crates.io/crates/artichoke-repl-history)
[![API](https://docs.rs/artichoke-repl-history/badge.svg)](https://docs.rs/artichoke-repl-history)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/artichoke_repl_history/)

Helpers for persisting Artichoke `airb` REPL history to disk.

This crate provides platform support for resolving the Artichoke Ruby `airb`
REPL's application data folder and path to a history file within it.

## Platform Support

On macOS, the history file is located in the current user's Application Support
directory.

On Windows, the history file is located in the current user's `LocalAppData`
known folder.

On Linux and other non-macOS Unix targets, the history file is located in the
`XDG_STATE_HOME` according to the [XDG Base Directory Specification], with the
specified fallback if the environment variable is not set.

[xdg base directory specification]:
  https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
artichoke-repl-history = "1.0.0"
```

And parse Readline editing mode like this:

```rust
use artichoke_repl_history::repl_history_file;

if let Some(hist_file) = repl_history_file() {
    // load history ...
}
```

## License

`artichoke-repl-history` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.
