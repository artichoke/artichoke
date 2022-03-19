# artichoke-core

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Core documentation](https://img.shields.io/badge/docs-artichoke--core-blue.svg)](https://artichoke.github.io/artichoke/artichoke_core/)

This crate provides a set of traits that, when implemented, comprise a complete
Ruby interpreter.

`artichoke-core` is `no_std` + `alloc` with an optional (enabled by default)
`std` feature.

Interpreters implement the traits in Artichoke Core to indicate which
capabilities they offer. Defining interpreters by their capabilities allows for
interpreter agnostic implementations of Ruby Core and Standard Library.

## Interpreter APIs

Artichoke Core defines traits for the following interpreter capabilities:

- [`ClassRegistry`][core-class-registry]: Define and store class specs for Ruby
  `Class`es.
- [`CoerceToNumeric`][core-coerce-numeric]: Coerce Ruby values to native
  numerics (floats and integers).
- [`Debug`][core-debug]: Provide debugging and `Exception` message support.
- [`DefineConstant`][core-define-constant]: Define global, class, and module
  constants to be arbitrary Ruby [`Value`][core-value]s.
- [`Eval`][core-eval]: Execute Ruby source code on an interpreter from various
  sources.
- [`Globals`][core-globals]: Get, set, and unset interpreter-level global
  variables.
- [`Hash`][core-hash]: Hashing functions such as building hashers.
- [`Intern`][core-intern]: Intern byte strings to a cheap to copy and compare
  symbol type.
- [`Io`][core-io]: External I/O APIs, such as writing to the standard output of
  the current process.
- [`LoadSources`][core-load-sources]: [Require][kernel#require] source code from
  interpreter disk or [`File`][core-file] gems.
- [`ModuleRegistry`][core-module-registry]: Define and store module spec for
  Ruby `Module`s.
- [`Parser`][core-parser]: Manipulate the parser state, e.g. setting the current
  filename.
- [`Prng`][core-prng]: An interpreter-level pseudorandom number generator that
  is the backend for [`Random::DEFAULT`].
- [`Regexp`][core-regexp]: Manipulate [`Regexp`][regexp-globals] global state.
- [`ReleaseMetadata`][core-releasemetadata]: Enable interpreters to describe
  themselves.
- [`TopSelf`][core-topself]: Access to the root execution context.
- [`Warn`][core-warn]: Emit warnings.

Artichoke Core also describes what capabilities a Ruby [`Value`][core-value]
must have and how to [convert][core-convert-module] between Ruby VM and Rust
types.

## Examples

[`artichoke-backend`] is one implementation of the `artichoke-core` traits.

To use all of the APIs defined in Artichoke Core, bring the traits into scope by
importing the prelude:

```rust
use artichoke_core::prelude::*;
```

## Crate features

All features are enabled by default:

- **std**: By default, `artichoke-core` is `no_std` + `alloc`. Enabling this
  feature adds several trait methods that depend on `OsStr` and `Path` as well
  as several implementations of `std::error::Error`.

## License

`artichoke-core` is licensed with the [MIT License](LICENSE) (c) Ryan Lopopolo.

[kernel#require]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
[`random::default`]: https://ruby-doc.org/core-2.6.3/Random.html#DEFAULT
[regexp-globals]:
  https://ruby-doc.org/core-2.6.3/Regexp.html#class-Regexp-label-Special+global+variables
[core-class-registry]:
  https://artichoke.github.io/artichoke/artichoke_core/class_registry/trait.ClassRegistry.html
[core-coerce-numeric]:
  https://artichoke.github.io/artichoke/artichoke_core/coerce_to_numeric/trait.CoerceToNumeric.html
[core-convert-module]:
  https://artichoke.github.io/artichoke/artichoke_core/convert/index.html
[core-debug]:
  https://artichoke.github.io/artichoke/artichoke_core/debug/trait.Debug.html
[core-define-constant]:
  https://artichoke.github.io/artichoke/artichoke_core/constant/trait.DefineConstant.html
[core-value]:
  https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html
[core-eval]:
  https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html
[core-globals]:
  https://artichoke.github.io/artichoke/artichoke_core/globals/trait.Globals.html
[core-hash]:
  https://artichoke.github.io/artichoke/artichoke_core/hash/trait.Hash.html
[core-intern]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[core-io]: https://artichoke.github.io/artichoke/artichoke_core/io/trait.Io.html
[core-load-sources]:
  https://artichoke.github.io/artichoke/artichoke_core/load/trait.LoadSources.html
[core-file]:
  https://artichoke.github.io/artichoke/artichoke_core/file/trait.File.html
[core-module-registry]:
  https://artichoke.github.io/artichoke/artichoke_core/module_registry/trait.ModuleRegistry.html
[core-parser]:
  https://artichoke.github.io/artichoke/artichoke_core/parser/trait.Parser.html
[core-prng]:
  https://artichoke.github.io/artichoke/artichoke_core/prng/trait.Prng.html
[core-regexp]:
  https://artichoke.github.io/artichoke/artichoke_core/regexp/trait.Regexp.html
[core-releasemetadata]:
  https://artichoke.github.io/artichoke/artichoke_core/release_metadata/trait.ReleaseMetadata.html
[core-topself]:
  https://artichoke.github.io/artichoke/artichoke_core/top_self/trait.TopSelf.html
[core-warn]:
  https://artichoke.github.io/artichoke/artichoke_core/warn/trait.Warn.html
[`artichoke-backend`]: https://artichoke.github.io/artichoke/artichoke_backend/
