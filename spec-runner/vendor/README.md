# spec-runner Vendored Dependencies

## ruby/spec

[ruby/spec](https://github.com/ruby/spec) is a black box testing suite for the
Ruby programming language. Artichoke uses ruby/spec to test the
[artichoke-backend runtime](../../artichoke-backend/src/extn) for compliance with MRI Ruby.

spec-runner [vendors](spec) a
[fork of ruby/spec](https://github.com/artichoke/spec/tree/artichoke-vendor).

This fork tracks [ruby/spec master](https://github.com/ruby/spec/tree/master)
closely and
[includes some patches](https://github.com/artichoke/spec/compare/master...artichoke:artichoke-vendor?expand=1).

## MSpec

[MSpec](http://github.com/ruby/mspec) is an RSpec-like test framework for
running ruby/spec. Artichoke uses MSpec to
[run ruby/spec](../src/spec_runner.rb).

spec-runner [vendors](mspec) a
[fork of mspec](https://github.com/artichoke/mspec/tree/artichoke-vendor).

This fork tracks [mspec master](https://github.com/ruby/mspec/tree/master)
closely and
[includes some patches](https://github.com/artichoke/mspec/compare/master...artichoke:artichoke-vendor?expand=1).
