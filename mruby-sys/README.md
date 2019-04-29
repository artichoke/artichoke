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
