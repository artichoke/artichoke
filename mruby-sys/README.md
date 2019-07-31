# mruby-sys

Crate mruby-sys provides Rust bindings for the
[mruby embedded Ruby interpreter](https://github.com/mruby/mruby).

This crate uses bindgen to generate Rust FFI bindings.

**Requires Ruby to compile.** In _very Ruby_ fashion, mruby uses Rake to build
_and_ uses Ruby to dynamically generate C sources at build time.

mruby supports
[cross compilation](https://github.com/mruby/mruby/blob/master/doc/guides/compile.md#cross-compilation-1),
but this crate does not.

## mruby-sys C Extension

mruby-sys provides a small C extension library (functions prefixed with
`mrb_sys_`) that is provides wrappers around macros and inline functions found
in mruby headers. The C extension also includes functions for doing fiddly
things with C unions that are more convenient to do in C.

## License

mruby-sys is licensed under the [MIT License](../LICENSE).

mruby-sys vendors and links against [mruby](https://github.com/mruby/mruby)
which is Copyright (c) 2019 mruby developers. mruby is licensed with the
[MIT License](https://github.com/mruby/mruby/blob/master/LICENSE).

Some portions of mruby-sys are derived from mrusty @
[1.0.0](https://github.com/anima-engine/mrusty/tree/v1.0.0) which is Copyright
(C) 2016 Dragoș Tiselice. mrusty is licensed with the
[Mozilla Public License 2.0](https://github.com/anima-engine/mrusty/blob/v1.0.0/LICENSE).

Some portions of mruby-sys are derived from go-mruby @
[cd6a04a](https://github.com/mitchellh/go-mruby/tree/cd6a04a) which is Copyright
(c) 2017 Mitchell Hashimoto. go-mruby is licensed with the
[MIT License](https://github.com/mitchellh/go-mruby/blob/cd6a04a/LICENSE).
