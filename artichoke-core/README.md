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

- [`DefineConstant`]: Define global, class, and module constants to be arbitrary
  Ruby [`Value`]s.
- [`Eval`]: Execute Ruby source code on an interpreter from various sources.
- [`Globals`]: Get, set, and unset interpreter-level global variables.
- [`Intern`]: Intern bytestrings to a cheap to copy and compare symbol type.
- [`Io`]: External I/O APIs, such as writing to the standard output of the
  current process.
- [`LoadSources`]: [Require][kernel#require] source code from interpreter disk
  or [`File`] gems.
- [`Parser`]: Manipulate the parser state, e.g. setting the current filename.
- [`Prng`]: An interpreter-level psuedorandom number generator that is the
  backend for [`Random::DEFAULT`].
- [`Regexp`]: Manipulate [`Regexp`] global state.
- [`ReleaseMetadata`]: Enable interpreters to describe themselves.
- [`TopSelf`]: Access to the root execution context.
- [`Warn`]: Emit warnings.

Artichoke Core also describes what capabilities a Ruby [`Value`] must have and
how to [convert] between Ruby VM and Rust types.

## Examples

[`artichoke-backend`](https://artichoke.github.io/artichoke/artichoke_backend/)
is one implementation of the `artichoke-core` traits.

To use all of the APIs defined in Artichoke Core, bring the traits into scope by
importing the prelude:

```
use artichoke_core::prelude::*;
```

## Crate features

All features are enabled by default:

- **std**: By default, `artichoke-core` is `no_std` + `alloc`. Enabling this
  feature adds several trait methods that depend on `OsStr` and `Path` as well
  as several implementations of `std::error::Error`.

## License

artichoke-core is licensed with the [MIT License](../LICENSE) (c) Ryan Lopopolo.

[kernel#require]: https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require
[`random::default`]: https://ruby-doc.org/core-2.6.3/Random.html#DEFAULT
[`regexp`]:
  https://ruby-doc.org/core-2.6.3/Regexp.html#class-Regexp-label-Special+global+variables
[convert]:
  https://artichoke.github.io/artichoke/artichoke_core/convert/index.html
[`defineconstant`]:
  https://artichoke.github.io/artichoke/artichoke_core/constant/trait.DefineConstant.html
[`value`]:
  https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html
[`eval`]:
  https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html
[`globals`]:
  https://artichoke.github.io/artichoke/artichoke_core/globals/trait.Globals.html
[`intern`]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[`io`]: https://artichoke.github.io/artichoke/artichoke_core/io/trait.Io.html
[`loadsources`]:
  https://artichoke.github.io/artichoke/artichoke_core/load/trait.LoadSources.html
[`file`]:
  https://artichoke.github.io/artichoke/artichoke_core/file/trait.File.html
[`parser`]:
  https://artichoke.github.io/artichoke/artichoke_core/parser/trait.Parser.html
[`prng`]:
  https://artichoke.github.io/artichoke/artichoke_core/prng/trait.Prng.html
[`regexp`]:
  https://artichoke.github.io/artichoke/artichoke_core/regexp/trait.Regexp.html
[`releasemetadata`]:
  https://artichoke.github.io/artichoke/artichoke_core/release_metadata/trait.ReleaseMetadata.html
[`topself`]:
  https://artichoke.github.io/artichoke/artichoke_core/top_self/trait.TopSelf.html
[`warn`]:
  https://artichoke.github.io/artichoke/artichoke_core/warn/trait.Warn.html
