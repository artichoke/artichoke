# spinoso-symbol

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/spinoso-symbol.svg)](https://crates.io/crates/spinoso-symbol)
[![API](https://docs.rs/spinoso-symbol/badge.svg)](https://docs.rs/spinoso-symbol)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/artichoke/spinoso_symbol/)

Identifier for interned byte strings and routines for manipulating the
underlying byte strings.

`Symbol` is a `Copy` type based on `u32`. `Symbol` is cheap to copy, store, and
compare. It is suitable for representing indexes into a string interner.

> `Symbol` objects represent names and some strings inside the Ruby interpreter.
> They are generated using the `:name` and `:"string"` literals syntax, and by
> the various `to_sym` methods. The same `Symbol` object will be created for a
> given name or string for the duration of a program's execution, regardless of
> the context or meaning of that name.

_Spinoso_ refers to _Carciofo spinoso di Sardegna_, the thorny artichoke of
Sardinia. The data structures defined in the `spinoso` family of crates form the
backbone of Ruby Core in Artichoke.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinoso-symbol = "0.1.0"
```

Most of the functionality in this crate depends on a Ruby interpreter that
implements [byte string interning APIs] and requires activating the `artichoke`
feature.

Without an interpreter, `spinoso-symbol` can determine whether a byte string is
a Ruby identifier:

```rust
use spinoso_symbol::IdentifierType;
assert_eq!("$spinoso".parse::<IdentifierType>(), Ok(IdentifierType::Global));
assert_eq!("@features".parse::<IdentifierType>(), Ok(IdentifierType::Instance));
assert_eq!("artichoke_crates".parse::<IdentifierType>(), Ok(IdentifierType::Local));
```

And print a debug representation of a byte string suitable for implementing
[`Symbol#inspect`]:

```rust
use spinoso_symbol::Inspect;
assert_eq!(Inspect::from("spinoso").collect::<String>(), ":spinoso");
assert_eq!(Inspect::from("@features").collect::<String>(), ":@features");
assert_eq!(
    Inspect::from("Artichoke is a Ruby made with Rust").collect::<String>(),
    r#":"Artichoke is a Ruby made with Rust""#,
);
```

## `no_std`

This crate is `no_std` compatible when built without the `std` feature. This
crate does not depend on [`alloc`].

## Crate features

All features are enabled by default.

- **artichoke** - Enables additional methods, functions, and types for
  implementing APIs from Ruby Core. Dropping this feature removes the
  `artichoke-core` and `focaccia` dependencies. Activating this feature also
  activates the **inspect** feature.
- **inspect** - Enables an iterator for generating debug output of a symbol byte
  string. Activating this feature also activates the **ident-parser** feature.
- **ident-parser** - Enables a parser to determing the Ruby identifier type, if
  any, for a byte string. Dropping this feature removes the `bstr` and
  `scolapasta-string-escape` dependencies.
- **std** - Enables a dependency on the Rust Standard Library. Activating this
  feature enables [`std::error::Error`] impls on error types in this crate.

### Artichoke integration

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

## License

`spinoso-symbol` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[the `symbol` api from ruby core]: https://ruby-doc.org/core-2.6.3/Symbol.html
[`intern`]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[`symbol::all_symbols`]:
  https://ruby-doc.org/core-2.6.3/Symbol.html#method-c-all_symbols
[`symbol#inspect`]: https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-inspect
[byte string interning apis]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[`symbol#inspect`]: https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-inspect
[`alloc`]: https://doc.rust-lang.org/alloc/
[`std::error::error`]: https://doc.rust-lang.org/std/error/trait.Error.html
