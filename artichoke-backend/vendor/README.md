# artichoke-backend Vendored Dependencies

## mruby

[mruby] is an embeddable implementation of Ruby with a rich C API. The mruby VM
is being used to bootstrap Artichoke.

artichoke-backend [vendors](mruby) a [fork of mruby][mruby-fork].

This fork is based on [mruby 3.1.0][mruby-forked-from] and [includes some
patches][mruby-patches].

[mruby]: https://github.com/mruby/mruby
[mruby-fork]: https://github.com/artichoke/mruby/tree/artichoke-vendor
[mruby-forked-from]: https://github.com/mruby/mruby/tree/3.1.0
[mruby-patches]:
  https://github.com/artichoke/mruby/compare/artichoke-mruby-branched-from-upstream...artichoke:artichoke-vendor?expand=1

## Ruby

[Ruby] is MRI Ruby, the reference implementation of Ruby. Artichoke uses Ruby as
the base for the implementation of the Ruby Standard Library in
[artichoke-backend](../src/extn/stdlib).

artichoke-backend [vendors](ruby) a [fork of Ruby][ruby-fork].

This fork is based on [Ruby 2.6.3][ruby-forked-from] and [includes some
patches][ruby-patches].

[ruby]: https://github.com/ruby/ruby
[ruby-fork]: https://github.com/artichoke/ruby/tree/artichoke-vendor
[ruby-forked-from]: https://github.com/ruby/ruby/tree/v2_6_3
[ruby-patches]:
  https://github.com/artichoke/ruby/compare/v2_6_3...artichoke:artichoke-vendor?expand=1

# Emscripten headers

[emsdk] contains headers for the C standard library that target
wasm32-unknown-emscripten and wasm32-unknown-unkown targets.

These headers should be from the same [version of emsdk used by the Artichoke
Playground][playground-emscripten-toolchain].

[emsdk]: https://github.com/emscripten-core/emsdk
[playground-emscripten-toolchain]:
  https://github.com/artichoke/playground/blob/trunk/emscripten-toolchain
