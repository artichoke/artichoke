# spinoso-array

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-array.svg)](https://crates.io/crates/spinoso-array)
[![API](https://docs.rs/spinoso-array/badge.svg)](https://docs.rs/spinoso-array)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_array/)

Contiguous growable vector types. Used to implement the backend for the [Ruby
`Array`][ruby-array] type in [Artichoke Ruby][artichoke].

> Arrays are ordered, integer-indexed collections of any object. (Ruby Array
> documentation)

`Array` types are growable vectors with potentially heap-allocated contents. The
types in this crate can be passed by pointer over FFI.

`Array`s have `O(1)` access to individual elements and slices. Mutation APIs are
`O(n)` at worst.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-array = "0.5"
```

Then construct and manipulate an `Array` like this:

```rust
use spinoso_array::Array;

let mut ary: Array<i32> = Array::from(&[1, 2, 3, 4]);
assert_eq!(ary.pop(), Some(4));

ary.unshift(0);
assert_eq!(ary, [0, 1, 2, 3]);

let other = ary.shift_n(10);
assert_eq!(ary, []);
assert_eq!(other, [0, 1, 2, 3]);
```

## Implementation

spinoso-array has two backends:

- `Array` is based on [`Vec`] from the Rust `alloc` crate and standard library.
  This Spinoso array type is enabled by default.
- `SmallArray` is based on [`SmallVec`] and implements the small vector
  optimization – small arrays are stored inline without a heap allocation.

## `no_std`

This crate is `no_std` compatible with a required dependency on [`alloc`].

## Crate features

All features are enabled by default.

- **small-array** - Enables an additional `SmallArray` array backend based on
  `SmallVec` that implements the small vector optimization. Disabling this
  feature drops the `smallvec` dependency.

## License

`spinoso-array` is licensed with the [MIT License](../LICENSE) (c) Ryan
Lopopolo.

[ruby-array]: https://ruby-doc.org/core-2.6.3/Array.html
[artichoke]: https://github.com/artichoke/artichoke
[`vec`]: https://doc.rust-lang.org/alloc/vec/struct.Vec.html
[`smallvec`]: https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html
[`alloc`]: https://doc.rust-lang.org/alloc/
