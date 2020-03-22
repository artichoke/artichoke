# artichoke-backend Vendored Dependencies

## mruby

[mruby](https://github.com/mruby/mruby) is an embeddable implementation of Ruby
with a rich C API. The mruby VM is being used to bootstrap Artichoke.

artichoke-backend [vendors](mruby) a
[fork of mruby](https://github.com/artichoke/mruby/tree/artichoke-vendor).

This fork tracks [mruby master](https://github.com/mruby/mruby/tree/master)
closely and
[includes some patches](https://github.com/artichoke/mruby/compare/master...artichoke:artichoke-vendor?expand=1).

## Ruby

[Ruby](https://github.com/ruby/ruby) is MRI Ruby, the reference implementation
of Ruby. Artichoke uses Ruby as the base for the implementation of the Ruby
Standard Library in [artichoke-backend](../src/extn/stdlib).

artichoke-backend [vendors](ruby) a
[fork of Ruby](https://github.com/artichoke/ruby/tree/artichoke-vendor).

This fork is based on [Ruby 2.6.3](https://github.com/ruby/ruby/tree/v2_6_3) and
[includes some patches](https://github.com/artichoke/ruby/compare/v2_6_3...artichoke:artichoke-vendor?expand=1).

# Emscripten headers

[emsdk](https://github.com/emscripten-core/emsdk) contains headers for the C
standard library that target wasm32-unknown-emscripten and wasm32-unknown-unkown
targets.
