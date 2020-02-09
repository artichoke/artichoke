# artichoke-backend

[![CircleCI](https://circleci.com/gh/artichoke/artichoke.svg?style=svg)](https://circleci.com/gh/artichoke/artichoke)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Backend documentation](https://img.shields.io/badge/docs-artichoke--backend-blue.svg)](https://artichoke.github.io/artichoke/artichoke_backend/)

artichoke-backend crate provides a Ruby interpreter. It currently is implemented
with [mruby](https://github.com/mruby/mruby) bindings exported by the
[`sys`](src/sys) module.

## Execute Ruby Code

artichoke-backend crate exposes several mechanisms for executing Ruby code on
the interpreter.

### Evaling Source Code

artichoke-backend crate exposes eval on the `State` with the `Eval` trait. Side
effects from eval are persisted across invocations.

```rust
use artichoke_backend::{Eval, ValueLike};

let mut interp = artichoke_backend::interpreter().unwrap();
let result = interp.eval(b"10 * 10").unwrap();
let result = result.try_into::<i64>();
assert_eq!(result, Ok(100));
```

## Virtual Filesystem and `Kernel#require`

The artichoke-backend `State` embeds an in-memory virtual filesystem. The VFS
stores Ruby sources that are either pure Ruby, implemented with a Rust `File`,
or both.

artichoke-backend crate implements
[`Kernel#require` and `Kernel#require_relative`](src/extn/core/kernel) which
loads sources from the VFS. For Ruby sources, the source is loaded from the VFS
as a `Vec<u8>` and evaled with `Eval::eval_with_context`. For Rust sources,
`File::require` methods are stored as custom metadata on `File` nodes in the
VFS.

## Embed Rust Types in Ruby `Value`s

Rust types that implement `RustBackedValue` can be injected into the interpreter
as the backend for a Ruby object.

Examples of `RustBackedValues` include:

- `Regexp` and `MatchData`, which are backed by regular expressions from the
  `onig` and `regex` crates.
- `ENV`, which glues Ruby to an environ backend.

## Converters Between Ruby and Rust Types

The [`convert` module](src/convert) provides implementations for conversions
between boxed Ruby values and native Rust types like `i64` and
`HashMap<String, Option<Vec<u8>>>` using an `Artichoke` interpreter.

## License

artichoke-backend is licensed with the [MIT License](/LICENSE) (c) Ryan
Lopopolo.

Some portions of artichoke-backend are derived from
[mruby](https://github.com/mruby/mruby) which is Copyright (c) 2019 mruby
developers. mruby is licensed with the
[MIT License](https://github.com/mruby/mruby/blob/master/LICENSE).

Some portions of artichoke-backend are derived from Ruby @
[2.6.3](https://github.com/ruby/ruby/tree/v2_6_3) which is copyright Yukihiro
Matsumoto \<matz@netlab.jp\>. Ruby is licensed with the
[2-clause BSDL License](https://github.com/ruby/ruby/blob/v2_6_3/COPYING).

artichoke-backend vendors headers provided by
[emsdk](https://github.com/emscripten-core/emsdk) which is Copyright (c) 2018
Emscripten authors. emsdk is licensed with the
[MIT/Expat License](https://github.com/emscripten-core/emsdk/blob/master/LICENSE).
