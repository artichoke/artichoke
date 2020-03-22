# Design and Goals

Artichoke is a platform for building Ruby implementations. You can build a
[ruby/spec](https://github.com/ruby/spec)-compliant Ruby by combining Artichoke
core, a VM and parser backend, and the Artichoke frontend.

Artichoke is designed to enable experimentation. The top goals of the project
are:

- [Support WebAssembly as a build target](https://github.com/artichoke/artichoke/labels/O-wasm-unknown).
- Support embedding and executing Ruby in untrusted environments.
- [Distribute Ruby applications as single-binary artifacts](https://github.com/artichoke/artichoke/labels/A-single-binary).
- [Implement Ruby with state-of-the-art dependencies](https://github.com/artichoke/artichoke/labels/A-deps).
- Experiment with VMs to support
  [dynamic codegen](https://github.com/artichoke/artichoke/labels/A-codegen),
  [ahead of time compilation](https://github.com/artichoke/artichoke/labels/A-compiler),
  [parallelism and eliminating the GIL](https://github.com/artichoke/artichoke/labels/A-parallelism),
  and novel
  [memory management and garbage collection techniques](https://github.com/artichoke/artichoke/labels/A-memory-management).

## Core

`artichoke-core` contains traits for the core set of APIs an interpreter must
implement. The traits in
[`artichoke-core`](https://artichoke.github.io/artichoke/artichoke_core/)
define:

- APIs a concrete VM must implement to support the Artichoke runtime and
  frontends.
- How to box polymorphic core types into
  [Ruby `Value`](https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html).
- [Interoperability](https://artichoke.github.io/artichoke/artichoke_core/convert/index.html)
  between the VM backend and the Rust-implemented core.

Some of the core APIs a Ruby implementation must provide are
[evaluating code](https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html),
[converting Rust data structures to boxed `Value`s on the interpreter heap](https://artichoke.github.io/artichoke/artichoke_core/convert/trait.ConvertMut.html),
and
[interning `Symbol`s](https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html).

### Runtime

Artichoke core provides an implementation-agnostic Ruby runtime. The runtime in
Artichoke core will pass 100% of the
[Core](https://github.com/artichoke/artichoke/labels/A-ruby-core) and
[Standard Library](https://github.com/artichoke/artichoke/labels/A-ruby-stdlib)
Ruby specs. The runtime will be implemented in a hybrid of Rust and Ruby. The
[`Regexp` implementation](../artichoke-backend/src/extn/core/regexp) is a
representative example of the approach.

### Embedding

Artichoke core will support embedding with:

- Multiple
  [filesystem backends](https://github.com/artichoke/artichoke/labels/A-filesystem),
  including an in-memory
  [virtual filesystem](https://artichoke.github.io/artichoke/artichoke_backend/fs/index.html).
- Multiple [`ENV` backends](../artichoke-backend/src/extn/core/env), including an
  in-memory `HashMap` backend.
- Optional C dependencies via multiple implementations of Core classes, e.g.
  [`Regexp`](../artichoke-backend/src/extn/core/regexp).
- [Optional standard-library](https://github.com/artichoke/artichoke/labels/A-optional-stdlib).
- [Optional multi-threading](https://github.com/artichoke/artichoke/labels/A-parallelism).
- Capturable IO.

### Experimentation

A Rust-implemented Ruby runtime offers an opportunity to experiment with:

- [Improving performance](https://github.com/artichoke/artichoke/labels/A-performance)
  of MRI Core and Standard Library.
- Implementing the runtime with
  [state-of-the-art dependencies](https://github.com/artichoke/artichoke/labels/A-deps).
- Distributing
  [single-binary builds](https://github.com/artichoke/artichoke/labels/A-single-binary).

## VM Backend

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

### Experimentation

VM backends offer an opportunity to experiment with:

- [Dynamic codegen](https://github.com/artichoke/artichoke/labels/A-codegen).
- [Compilation](https://github.com/artichoke/artichoke/labels/A-compiler).
- [Parallelism and eliminating the GIL](https://github.com/artichoke/artichoke/labels/A-parallelism).
- [Memory management and garbage collection techniques](https://github.com/artichoke/artichoke/labels/A-memory-management).

## Frontend

Artichoke will include `ruby` and `irb`
[binary frontends](https://github.com/artichoke/artichoke/labels/A-frontend)
with dynamically selectable VM backends.

Artichoke will produce a
[WebAssembly frontend](https://github.com/artichoke/artichoke/labels/A-build-target).

Artichoke will include implementation-agnostic
[C APIs](https://github.com/artichoke/artichoke/labels/A-C-API) targeting:

- [MRI API](https://github.com/artichoke/artichoke/labels/CAPI-MRI) from Ruby.
- [`MRB_API`](https://github.com/artichoke/artichoke/labels/CAPI-mruby) from
  mruby.
