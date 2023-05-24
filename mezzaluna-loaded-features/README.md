# mezzaluna-loaded-features

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/mezzaluna-loaded-features.svg)](https://crates.io/crates/mezzaluna-loaded-features)
[![API](https://docs.rs/mezzaluna-loaded-features/badge.svg)](https://docs.rs/mezzaluna-loaded-features)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/mezzaluna_loaded_features/)

A container for storing loaded features in a Ruby VM.

The Artichoke Ruby VM may load code (called "features") with several strategies.
Features can be loaded from an in-memory virtual file system (which can also
store native extensions) or natively from local disk.

The data structures in this crate track which features have been loaded with
support for deduplicating features which may reside at multiple paths.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mezzaluna-loaded-features = "0.9.0"
```

And track loaded features like this:

```rust
use mezzaluna_loaded_features::{Feature, LoadedFeatures};

let mut features = LoadedFeatures::new();
features.insert(Feature::with_in_memory_path("/src/_lib/test.rb".into()));
features.insert(Feature::with_in_memory_path("set.rb".into()));
features.insert(Feature::with_in_memory_path("artichoke.rb".into()));

for f in features.features() {
    println!("Loaded feature at: {}", f.path().display());
}
```

## Crate features

All features are enabled by default unless otherwise noted.

- **disk** - Enables a tracking features loaded from disk and deduplicating by
  physical file. Activating this feature adds a dependency on the `same-file`
  crate.

## License

`mezzaluna-loaded-features` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.
