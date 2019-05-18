# ferrocarril

[![CircleCI](https://circleci.com/gh/lopopolo/ferrocarril.svg?style=svg)](https://circleci.com/gh/lopopolo/ferrocarril)

ferrocarril aims to embed a [Ruby on Rails](https://github.com/rails/rails) web
application that talks to an external MySQL database in Rust and serve the app
with [Rocket](https://rocket.rs/).

_ferrocarril_ means _railway_ in Spanish and sounds like _ferrous_ which means
_containing iron_.

## Usage

```bash
cargo run --bin foolsgold
```

Then, open `http://localhost:8000` on your browser.

## foolsgold

The [foolsgold crate](/foolsgold) is an early attempt to achieve this goal. The
foolsgold crate:

- Embeds a safe [interpreter](/mruby) that wraps
  [generated C bindings](/mruby-sys) for
  [mruby](https://github.com/mruby/mruby).
- Implements a web application server similar to
  [Thin](https://github.com/macournoyer/thin) that supports shared nothing and
  prefork execution modes.
- Loads pure Ruby sources into a virtual filesystem such that Ruby code can
  require them.
- Defines classes and modules in Rust and loads them into the virtual filesystem
  such that Ruby code can require them.
- Shares Rust objects across mruby interpreter instances.
- Defines Ruby classes whose instances are backed by Rust structs.
- Converts a [Rack-compatible response](https://rack.github.io/) into a
  [Rocket response](https://rocket.rs/v0.4/guide/responses/#responses).

## TODOs

### Core

mruby does not implement all
[Ruby 2.6 core classes](https://ruby-doc.org/core-2.6.3/).

Required classes include:

- `Regexp`
- `File`
- `IO`

### Standard Library

mruby does not implement any of the
[Ruby 2.6 standard library](https://ruby-doc.org/stdlib-2.6.3/).

Required packages include:

- `json`
- `set`
- `stringio`
- `time`
- `uri`
- `zlib`
