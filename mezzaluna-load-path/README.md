# mezzaluna-load-path

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/mezzaluna-load-path.svg)](https://crates.io/crates/mezzaluna-load-path)
[![API](https://docs.rs/mezzaluna-load-path/badge.svg)](https://docs.rs/mezzaluna-load-path)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/mezzaluna_load_path/)

Ruby load path builders.

An Artichoke Ruby VM may load code (called "features") from several file system
locations. These locations form the `$LOAD_PATH` global.

Code and native extensions from the Ruby Core library and Ruby Standard Library
can be loaded from an in-memory virtual file system.

Users can prepend items to the load path at interpreter boot by setting the
`RUBYLIB` environment variable.

This crate exports two builders which can be used to construct the initial load
path at interpreter boot. See their documentation for more details.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mezzaluna-load-path = "0.1.0"
```

And construct the Ruby load path like this:

```rust
use std::ffi::OsStr;
use std::path::PathBuf;
use mezzaluna_load_path::{RubyCore, Rubylib};

fn build_load_path() -> Option<Box<[PathBuf]>> {
    let core_loader = RubyCore::new();
    let rubylib_loader = Rubylib::with_rubylib(OsStr::new("lib"))?;

    // Assemble the load path in priority order.
    let load_path = rubylib_loader
        .into_load_path()
        .into_iter()
        .chain(core_loader.load_path().into_iter().map(PathBuf::from))
        .collect::<Box<[PathBuf]>>();

    assert_eq!(load_path.len(), 3);
    Some(load_path)
}
```

## Crate features

All features are enabled by default unless otherwise noted.

- **rubylib** - Enables a builder which can parse load paths from the `RUBYLIB`
  environment variable.

## License

`mezzaluna-load-path` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.
