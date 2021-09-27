# spinoso-env

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-env.svg)](https://crates.io/crates/spinoso-env)
[![API](https://docs.rs/spinoso-env/badge.svg)](https://docs.rs/spinoso-env)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_env/)

ENV is a hash-like accessor for environment variables.

This module implements the [`ENV`] singleton object from Ruby Core.

In Artichoke, the enviroment variable store is modeled as a hash map of byte
vector keys and values, e.g. `HashMap<Vec<u8>, Vec<u8>>`. Backends are expected
to convert their internals to this representation in their public APIs. For this
reason, all APIs exposed by ENV backends in this crate are fallible.

You can use this object in your application by accessing it directly. As a Core
API, it is globally available:

```ruby
ENV['PATH']
ENV['PS1'] = 'artichoke> '
```

There are two `ENV` implementations in this crate:

- `Memory`, enabled by default, implements an `ENV` store and accessor on top of
  a Rust `HashMap`. This backend does not query or modify the host system.
- `System`, enabled when the **system-env** feature is activated, is a proxy for
  the system environment and uses platform-specific APIs defined in the [Rust
  Standard Library].

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-env = "0.1"
```

Using the in-memory backend allows safely manipulating an emulated environment:

```rust
use spinoso_env::Memory;
fn example() -> Result<(), spinoso_env::Error> {
    let mut env = Memory::new();
    // This does not alter the behavior of the host Rust process.
    env.put(b"PATH", None)?;
    // `Memory` backends start out empty.
    assert_eq!(env.get(b"HOME")?, None);
    Ok(())
}
```

System backends inherit and mutate the environment from the current Rust
process:

```rust
use spinoso_env::System;
fn example() -> Result<(), spinoso_env::Error> {
    const ENV: System = System::new();
    ENV.put(b"RUBY", Some(b"Artichoke"))?;
    assert!(ENV.get(b"PATH")?.is_some());
    Ok(())
}
```

## Crate features

This crate requires [`std`], the Rust Standard Library.

All features are enabled by default:

- **system-env** - Enable an `ENV` backend that accesses the host system's
  environment variables via the [`std::env`](module@std::env) module.

## License

`spinoso-env` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`env`]: https://ruby-doc.org/core-2.6.3/ENV.html
[`hashmap`]: std::collections::HashMap
[rust standard library]: https://doc.rust-lang.org/std/
[`std`]: https://doc.rust-lang.org/std/
