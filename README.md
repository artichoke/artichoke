# Artichoke Ruby

[![CircleCI](https://circleci.com/gh/artichoke/artichoke.svg?style=svg)](https://circleci.com/gh/artichoke/artichoke)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
<br>
[![Core documentation](https://img.shields.io/badge/docs-artichoke--core-blue.svg)](https://artichoke.github.io/artichoke/artichoke_core/)
[![Virtual filesystem documentation](https://img.shields.io/badge/docs-artichoke--vfs-blue.svg)](https://artichoke.github.io/artichoke/artichoke_vfs/)
[![mruby backend documentation](https://img.shields.io/badge/docs-artichoke--backend-blue.svg)](https://artichoke.github.io/artichoke/artichoke_backend/)
[![mruby-sys documentation](https://img.shields.io/badge/docs-mruby--sys-blue.svg)](https://artichoke.github.io/artichoke/mruby_sys/)

<p align="center">
  <img width="200" height="200" src="https://artichoke.github.io/artichoke/logo.svg">
</p>

Artichoke is a platform for implementing
[spec-compliant](https://github.com/ruby/spec) Ruby implementations. Artichoke
provides a Ruby runtime implemented in Rust that can be loaded into many VM
backends.

## Architecture

A Ruby implementation based on Artichoke consists of three components: Artichoke
core, a VM backend, and the Artichoke frontend.

### Core

[Artichoke core](https://artichoke.github.io/artichoke/artichoke_core/) exposes
a set of traits that define:

- Capabilities of a VM backend.
- Capabilities of a
  [Ruby `Value`](https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html).
- Interoperability between the VM backend and the Rust-implemented core.

Capabilities a Ruby implementation must provide include
[evaling code](https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html),
[declaring classes and modules](https://artichoke.github.io/artichoke/artichoke_core/def/trait.DeclareClassLike.html),
and
[exposing _top self_](https://artichoke.github.io/artichoke/artichoke_core/top_self/trait.TopSelf.html).

#### Runtime

Artichoke core provides an implementation-agnostic Ruby runtime which any
implementation can load. The runtime in Artichoke core will pass 100% of the
[Core](https://github.com/artichoke/artichoke/labels/A-ruby-core) and
[Standard Library](https://github.com/artichoke/artichoke/labels/A-ruby-stdlib)
Ruby specs. The runtime will be implemented in a hybrid of Rust and Ruby. The
[`Regexp` implementation](/artichoke-backend/src/extn/core/regexp) is a
representative example of the approach.

#### Embedding

Artichoke core will support embedding with:

- Multiple
  [filesystem backends](https://github.com/artichoke/artichoke/labels/A-filesystem),
  including an in-memory
  [virtual filesystem](https://artichoke.github.io/artichoke/artichoke_vfs/).
- [Optional standard-library](https://github.com/artichoke/artichoke/labels/A-optional-stdlib).
- [Optional multi-threading](https://github.com/artichoke/artichoke/labels/A-parallelism).
- Capturable IO.

#### Experimentation

A Rust-implemented Ruby runtime offers an opportunity to experiment with:

- [Improving performance](https://github.com/artichoke/artichoke/labels/A-performance)
  of MRI Core and Standard Library.
- Implementing the runtime with
  [state-of-the-art dependencies](https://github.com/artichoke/artichoke/labels/A-deps).
- Distributing
  [single-binary builds](https://github.com/artichoke/artichoke/labels/A-single-binary).

### VM Backend

Artichoke core does not provide a parser or a VM for executing Ruby. VM backends
provide these functions.

Artichoke currently includes an
[mruby backend](https://github.com/artichoke/artichoke/labels/B-mruby). There
are plans to add an
[MRI backend](https://github.com/artichoke/artichoke/labels/B-MRI) and a pure
Rust backend.

VM backends are responsible for passing 100% of the
[Language](https://github.com/artichoke/artichoke/labels/A-ruby-language) Ruby
specs.

#### Experimentation

VM backends offer an opportunity to experiment with:

- [Dynamic codegen](https://github.com/artichoke/artichoke/labels/A-codegen).
- [Compilation](https://github.com/artichoke/artichoke/labels/A-compiler).
- [Parallelism and eliminating the GIL](https://github.com/artichoke/artichoke/labels/A-parallelism).

### Frontend

Artichoke will include `ruby` and `irb`
[binary frontends](https://github.com/artichoke/artichoke/labels/A-frontend)
with dynamically selectable VM backends.

Artichoke will produce a
[WebAssembly frontend](https://github.com/artichoke/artichoke/labels/A-cross-build).

Artichoke will include implementation-agnostic
[C APIs](https://github.com/artichoke/artichoke/labels/A-C-API) targeting:

- [MRI API](https://github.com/artichoke/artichoke/labels/CAPI-MRI) from Ruby.
- [`MRB_API`](https://github.com/artichoke/artichoke/labels/CAPI-mruby) from
  mruby.

## Try Artichoke

You can [try Artichoke in your browser](https://artichoke.github.io/artichoke/).
The Artichoke Playground runs a [WebAssembly](https://webassembly.org/) build of
Artichoke.

If you would prefer to run a local build,
[set up a Rust toolchain](/CONTRIBUTING.md#rust-toolchain) and launch an
interactive Artichoke shell with:

```shell
cargo run -p artichoke-frontend --bin airb
```

## Contributing

Artichoke aspires to be a Ruby 2.6.3-compatible implementation of the Ruby
programming language.
[There is lots to do](https://github.com/artichoke/artichoke/issues).

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you
[filed an issue so we can fix it](https://github.com/artichoke/artichoke/issues/new).

If you would like to contribute code 👩‍💻👨‍💻, find an issue that looks interesting
and leave a comment that you're beginning to investigate. If there is no issue,
please file one before beginning to work on a PR.

### Discussion

If you'd like to engage in a discussion outside of GitHub, you can
[join Artichoke's public Discord server](https://discord.gg/QCe2tp2).
