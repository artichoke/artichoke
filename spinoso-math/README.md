# spinoso-math

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-math.svg)](https://crates.io/crates/spinoso-math)
[![API](https://docs.rs/spinoso-math/badge.svg)](https://docs.rs/spinoso-math)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_math/)

The Ruby Math module.

The Math module contains module functions for basic trigonometric and
transcendental functions. See class [`Float`] for a list of constants that
define Ruby's floating point accuracy.

This crate defines math operations as free functions. These functions differ
from those defined in Rust [`core`] by returning a `DomainError` when an input
is outside of the domain of the function and results in [`NaN`].

`spinoso-math` assumes the Ruby VM uses double precision [`f64`] floats.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-math = "0.3.0"
```

Compute the hypotenuse:

```rust
use spinoso_math as math;
assert_eq!(math::hypot(3.0, 4.0), 5.0);
```

Compute log with respect to the base 10 and handle domain errors:

```rust
use spinoso_math as math;
assert_eq!(math::log10(1.0), Ok(0.0));
assert_eq!(math::log10(10.0), Ok(1.0));
assert_eq!(math::log10(1e100), Ok(100.0));

assert_eq!(math::log10(0.0), Ok(f64::NEG_INFINITY));
assert!(math::log10(-0.1).is_err());

// A NaN return value is distinct from a `DomainError`.
assert!(matches!(math::log10(f64::NAN), Ok(result) if result.is_nan()));
```

## Crate features

All features are enabled by default.

- **full** - Enables implementations of math functions that do not have
  implementations in Rust [`core`]. Dropping this feature removes the [`libm`]
  dependency.

## License

`spinoso-math` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[`float`]: https://ruby-doc.org/core-2.6.3/Float.html
[`core`]: https://doc.rust-lang.org/core/
[`nan`]: https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.NAN
[`f64`]: https://doc.rust-lang.org/std/primitive.f64.html
[`libm`]: https://crates.io/crates/libm
