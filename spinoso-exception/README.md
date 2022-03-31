# spinoso-exception

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-exception.svg)](https://crates.io/crates/spinoso-exception)
[![API](https://docs.rs/spinoso-exception/badge.svg)](https://docs.rs/spinoso-exception)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_exception/)

Built in Ruby exception types.

Descendants of class [`Exception`] are used to communicate between
[`Kernel#raise`] and `rescue` statements in `begin ... end` blocks. Exception
objects carry information about the exception – its type (the exception's class
name), an optional descriptive string, and optional traceback information.
`Exception` subclasses may add additional information like [`NameError#name`].

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-exception = "0.1.0"
```

Create exceptions and return Ruby errors:

```rust
use spinoso_exception::{RuntimeError, StandardError};
const ERR: StandardError = StandardError::new();
let exc = RuntimeError::from("failed to generate random bytes");
```

This crate exposes a `RubyException` trait that unifies all of the exception
types it defines. `RubyException` is [object safe] and can be used to create
trait objects of any Ruby exception.

```rust
use spinoso_exception::{FrozenError, NotImplementedError, RubyException};

/// Ruby Core Array.
struct Array(());

impl Array {
    pub fn is_frozen(&self) -> bool {
        true
    }
}

pub fn array_concat(slf: Array, other: Array) -> Result<Array, Box<dyn RubyException>> {
    if slf.is_frozen() {
        return Err(Box::new(FrozenError::new()));
    }
    Err(Box::new(NotImplementedError::new()))
}
```

## `no_std`

This crate is `no_std` compatible when built without the `std` feature. This
crate has a required dependency on [`alloc`].

## Crate features

All features are enabled by default.

- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables [`std::error::Error`] impls on error types in this crate.

## License

`spinoso-exception` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.

[`exception`]: https://ruby-doc.org/core-2.6.3/Exception.html
[`kernel#raise`]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-raise
[`nameerror#name`]: https://ruby-doc.org/core-2.6.3/NameError.html#method-i-name
[object safe]:
  https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
[`alloc`]: https://doc.rust-lang.org/alloc/
[`std::error::error`]: https://doc.rust-lang.org/std/error/trait.Error.html
