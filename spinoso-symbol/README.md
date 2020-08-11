# spinoso-symbol

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-symbol.svg)](https://crates.io/crates/spinoso-symbol)
[![API](https://docs.rs/spinoso-symbol/badge.svg)](https://docs.rs/spinoso-symbol)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_symbol/)

Identifier for interned bytestrings and routines for manipulating the underlying
bytestrings.

`Symbol` is a `Copy` type based on `u32`. `Symbol` is cheap to copy, store, and
compare. It is suitable for representing indexes into a string interner.

> `Symbol` objects represent names and some strings inside the Ruby interpreter.
> They are generated using the `:name` and `:"string"` literals syntax, and by
> the various `to_sym` methods. The same `Symbol` object will be created for a
> given name or string for the duration of a program's execution, regardless of
> the context or meaning of that name.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The idea is that the data structures defined in the `spinoso` family
of crates will form the backbone of Ruby Core in Artichoke.

# Artichoke integration

This crate has an `artichoke` Cargo feature. When this feature is active, this
crate implements [the `Symbol` API from Ruby Core]. These APIs require resolving
the underlying bytes associated with the `Symbol` via a type that implements
`Intern` from `artichoke-core`.

APIs that require this feature to be active are highlighted in the
documentation.

This crate provides an `AllSymbols` iterator for walking all symbols stored in
an [`Intern`]er and an extension trait for constructing it which is suitable for
implementing [`Symbol::all_symbols`] from Ruby Core.

This crate provides an `Inspect` iterator for converting `Symbol` byte content
to a debug representation suitable for implementing [`Symbol#inspect`] from Ruby
Core.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-symbol = "0.3"
```

Most of the functionality in this crate depends on a Ruby interpreter that
implements [bytestring interning APIs] and requires activating the `artichoke`
feature.

## `no_std`

This crate is `no_std` compatible when built without the `std` feature. This
crate does not depend on [`alloc`].

## Crate features

All features are enabled by default.

- **artichoke** - Enables additional methods, functions, and types for
  implementing APIs from Ruby Core. Dropping this feature removes the
  `artichoke-core`, `bstr`, and `focaccia` dependencies.
- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables [`std::error::Error`] impls on error types in this crate.

## License

`spinoso-symbol` is licensed with the [MIT License](../LICENSE) (c) Ryan
Lopopolo.

[the `symbol` api from ruby core]: https://ruby-doc.org/core-2.6.3/Symbol.html
[`intern`]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[`symbol::all_symbols`]:
  https://ruby-doc.org/core-2.6.3/Symbol.html#method-c-all_symbols
[`symbol#inspect`]: https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-inspect
[bytestring interning apis]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[`alloc`]: https://doc.rust-lang.org/alloc/
[`std::error::error`]: https://doc.rust-lang.org/std/error/trait.Error.html
