# artichoke-backend

[![GitHub Actions](https://github.com/artichoke/artichoke/workflows/CI/badge.svg)](https://github.com/artichoke/artichoke/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Backend documentation](https://img.shields.io/badge/docs-artichoke--backend-blue.svg)](https://artichoke.github.io/artichoke/artichoke_backend/)

`artichoke-backend` crate provides a Ruby interpreter. It currently is
implemented with [mruby] bindings exported by the [`sys`](src/sys) module.

## Execute Ruby Code

`artichoke-backend` crate exposes several mechanisms for executing Ruby code on
the interpreter.

### Evaling Source Code

The `artichoke-backend` interpreter implements [`Eval` from `artichoke-core`].

```rust
use artichoke_backend::prelude::*;

fn example() -> Result<(), Error> {
    let mut interp = artichoke_backend::interpreter()?;
    let result = interp.eval(b"10 * 10")?;
    let result = result.try_convert_into::<i64>(&interp)?;
    assert_eq!(100, result);
    interp.close();
    Ok(())
}
```

### Calling Functions on Ruby Objects

`Value`s returned by the `artichoke-backend` interpreter implement [`Value` from
`artichoke-core`], which enables calling Ruby functions from Rust.

```rust
use artichoke_backend::prelude::*;

fn example() -> Result<(), Error> {
    let mut interp = artichoke_backend::interpreter()?;
    let s = interp.eval(b"'ruby funcall'")?;
    let len = s.funcall(&mut interp, "length", &[], None)?;
    let len = len.try_convert_into::<usize>(&interp)?;
    assert_eq!(12, len);
    interp.close();
    Ok(())
}
```

## Virtual Filesystem and `Kernel#require`

The `artichoke-backend` interpreter includes an in-memory virtual filesystem.
The filesystem stores Ruby sources and Rust extension functions that are similar
to MRI C extensions.

The virtual filesystem enables applications built with `artichoke-backend` to
`require` sources that are embedded in the binary without host filesystem
access.

## Embed Rust Types in Ruby `Value`s

`artichoke-backend` exposes a concept similar to `data`-typed values in MRI and
mruby.

When Rust types implement a special trait, they can be embedded in a Ruby
`Value` and passed through the Ruby VM as a Ruby object. Classes defined in this
way can define methods in Rust or Ruby.

Examples of these types include:

- `Regexp` and `MatchData`, which are backed by regular expressions from the
  `onig` and `regex` crates.
- `ENV`, which glues Ruby to an environ backend.

## Converters Between Ruby and Rust Types

The [`convert` module](src/convert) provides implementations for conversions
between boxed Ruby values and native Rust types like `i64` and
`HashMap<String, Option<Vec<u8>>>` using an `artichoke-backend` interpreter.

## License

`artichoke-backend` is licensed with the [MIT License](LICENSE) (c) Ryan
Lopopolo.

Some portions of artichoke-backend are derived from [mruby] which is Copyright
(c) 2019 mruby developers. mruby is licensed with the [MIT
License][mruby-license].

Some portions of artichoke-backend are derived from Ruby @ [2.6.3][ruby-2.6.3]
which is copyright Yukihiro Matsumoto \<matz@netlab.jp\>. Ruby is licensed with
the [2-clause BSDL License][ruby-license].

artichoke-backend vendors headers provided by [emsdk] which is Copyright (c)
2018 Emscripten authors. emsdk is licensed with the [MIT/Expat
License][emsdk-license].

[`eval` from `artichoke-core`]:
  https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html
[`value` from `artichoke-core`]:
  https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html
[mruby]: https://github.com/mruby/mruby
[mruby-license]: https://github.com/mruby/mruby/blob/master/LICENSE
[ruby-2.6.3]: https://github.com/ruby/ruby/tree/v2_6_3
[ruby-license]: https://github.com/ruby/ruby/blob/v2_6_3/COPYING
[emsdk]: https://github.com/emscripten-core/emsdk
[emsdk-license]: https://github.com/emscripten-core/emsdk/blob/master/LICENSE
