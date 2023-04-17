# mezzaluna-type-registry

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/mezzaluna-type-registry.svg)](https://crates.io/crates/mezzaluna-type-registry)
[![API](https://docs.rs/mezzaluna-type-registry/badge.svg)](https://docs.rs/mezzaluna-type-registry)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/mezzaluna_type_registry/)

A registry for "type spec" values that uses types as keys.

This crate can be used to implement a store for static configuration associated
with Rust types, such as name strings and function pointers.

This crate is used in `artichoke-backend` to store `mrb_data_type` information
for foreign types stored in mruby `mrb_value`s.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mezzaluna-type-registry = "1.0.1"
```

And store "specs" for Rust types like this:

```rust
use mezzaluna_type_registry::Registry;

let mut reg: Registry<&'static str> = Registry::with_capacity(10);
reg.insert::<i32>(Box::new("Numeric"));
reg.insert::<Vec<u8>>(Box::new("String"));

assert_eq!(reg.get::<i32>(), Some(&"Numeric"));
assert_eq!(reg.get::<Vec<u8>>(), Some(&"String"));
assert_eq!(reg.get::<f64>(), None);
```

## License

`mezzaluna-type-registry` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.
