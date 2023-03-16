# scolapasta-fixable

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/scolapasta-fixable.svg)](https://crates.io/crates/scolapasta-fixable)
[![API](https://docs.rs/scolapasta-fixable/badge.svg)](https://docs.rs/scolapasta-fixable)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/scolapasta_fixable/)

Functions for converting numeric immediates to integer or "fixnum" immediates.

Fixnums have range of a 63-bit unsigned int and are returned as a native
representation `i64`.

_Scolapasta_ refers to a specialized colander used to drain pasta. The utilities
defined in the `scolapasta` family of crates are the kitchen tools for preparing
Artichoke Ruby.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scolapasta-fixable = "0.1.0"
```

Check whether a numeric value is able to be converted to an in-range "fixnum":

```rust
use scolapasta_fixable::RB_FIXABLE;

assert!(RB_FIXABLE(23_u8));
assert!(RB_FIXABLE(u16::MIN));
assert!(RB_FIXABLE(i32::MAX));
assert!(RB_FIXABLE(1024_u64));
assert!(RB_FIXABLE(1024_i64));
assert!(RB_FIXABLE(1.0_f32));
assert!(RB_FIXABLE(-9000.27_f64));
```

`scolapasta-fixable` also exports a `Fixable` trait which provides methods on
numeric types to check if they are fixable and to do a fallible conversion to an
`i64` fixnum.

```rust
use scolapasta_fixable::Fixable;

assert!(23_u8.is_fixable());
assert_eq!(23_u8.to_fix(), Some(23_i64));
assert!((-9000.27_f64).is_fixable());
assert_eq!((-9000.27_f64).to_fix(), Some(-9000_i64));
```

Some numeric types, such as `u64`, `i128`, and `f64` have values that exceed
fixnum range. Conversions on values of these types which are outside the 63-bit
int range will fail:

```rust
use scolapasta_fixable::Fixable;

assert_eq!(u64::MAX.to_fix(), None);
assert_eq!(i128::MIN.to_fix(), None);
assert_eq!(4_611_686_018_427_387_904.0_f64.to_fix(), None);
assert_eq!(f64::INFINITY.to_fix(), None);
assert_eq!(f64::NAN.to_fix(), None);
```

For non-integer fixable types, the fractional part is discarded when converting
to fixnum, i.e. converting to fixnum rounds to zero.

## `no_std`

This crate is `no_std` compatible. This crate does not depend on [`alloc`].

## License

`scolapasta-fixable` is licensed under the [MIT License](LICENSE) (c) Ryan
Lopopolo.

This repository includes a vendored copy of the arithmetic headers from Ruby
3.2.0, which is licensed under the [Ruby license] or [BSD 2-clause license]. See
[`vendor/README.md`] for more details. These sources are not distributed on
[crates.io].

[ruby license]: vendor/ruby-3.2.0/COPYING
[bsd 2-clause license]: vendor/ruby-3.2.0/BSDL
[`vendor/readme.md`]: vendor/README.md
[crates.io]: https://crates.io/
