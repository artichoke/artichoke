# mruby-sys

Crate mruby-sys provides Rust bindings for the
[mruby embedded Ruby interpreter](https://github.com/mruby/mruby).

This crate uses bindgen to generate Rust FFI bindings.

**Requires Ruby to compile.** In _very Ruby_ fashion, mruby uses Rake to build
_and_ uses Ruby to dynamically generate C sources at build time.

mruby supports
[cross compilation](https://github.com/mruby/mruby/blob/master/doc/guides/compile.md#cross-compilation-1),
but this crate does not.

## mruby-sys Extension Library

mruby-sys provides a small extension library (functions prefixed with
`mrb_sys_`) that is primarily focused on providing wrappers around macros found
in mruby headers.

## LICENSE

mruby-sys is licensed under the [MIT License](../LICENCE). Some portions of
mruby-sys are derived from
[mrusty @ 1.0.0](https://github.com/anima-engine/mrusty/tree/v1.0.0) which are
Copyright (C) 2016 Dragoș Tiselice under the
[Mozilla Public License 2.0](https://github.com/anima-engine/mrusty/blob/v1.0.0/LICENSE).
