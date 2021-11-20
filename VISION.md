# Design and Goals

Artichoke is a platform for building Ruby implementations. You can build a
[ruby/spec]-compliant Ruby by combining Artichoke core, a VM and parser backend,
and the Artichoke frontend.

Artichoke is designed to enable experimentation. The top goals of the project
are:

- [Support WebAssembly as a build target][wasm-target].
- Support embedding and executing Ruby in untrusted environments.
- [Distribute Ruby applications as single-binary artifacts][a-single-binary].
- [Implement Ruby with state-of-the-art dependencies][a-deps].
- Experiment with VMs to support [dynamic codegen][a-codegen], [ahead of time
  compilation][a-compiler], [parallelism and eliminating the
  GIL][a-parallelism], and novel [memory management and garbage collection
  techniques][a-memory-management].

## Core

`artichoke-core` contains traits for the core set of APIs an interpreter must
implement. The traits in [`artichoke-core`] define:

- APIs a concrete VM must implement to support the Artichoke runtime and
  frontends.
- How to box polymorphic core types into [Ruby `Value`].
- [Interoperability] between the VM backend and the Rust-implemented core.

Some of the core APIs a Ruby implementation must provide are [evaluating
code][core-eval], [converting Rust data structures to boxed `Value`s on the
interpreter heap][core-converter], and [interning `Symbol`s][core-intern].

### Runtime

Artichoke core provides an implementation-agnostic Ruby runtime. The runtime in
Artichoke core will pass 100% of the [Core][a-ruby-core] and [Standard
Library][a-ruby-stdlib] Ruby specs. The runtime will be implemented in a hybrid
of Rust and Ruby. The [`Regexp` implementation][extn-regexp] is a representative
example of the approach.

### Embedding

Artichoke core will support embedding with:

- Multiple [file system backends], including an in-memory [virtual file system].
- Multiple [`ENV` backends][extn-env], including an in-memory `HashMap` backend.
- Optional C dependencies via multiple implementations of Core classes, e.g.
  [`Regexp`][extn-regexp].
- [Optional standard-library][a-optional-stdlib].
- [Optional multi-threading][a-parallelism].
- Capturable IO.

### Experimentation

A Rust-implemented Ruby runtime offers an opportunity to experiment with:

- [Improving performance][a-performance] of MRI Core and Standard Library.
- Implementing the runtime with [state-of-the-art dependencies][a-deps].
- Distributing [single-binary builds][a-single-binary].

## VM Backend

Artichoke core does not provide a parser or a VM for executing Ruby. VM backends
provide these functions.

Artichoke currently includes an [mruby backend][b-mruby]. There are plans to add
an [MRI backend][b-mri] and a [pure Rust backend][b-artichoke].

VM backends are responsible for passing 100% of the [Language][a-ruby-language]
Ruby specs.

### Experimentation

VM backends offer an opportunity to experiment with:

- [Dynamic codegen][a-codegen].
- [Compilation][a-compiler].
- [Parallelism and eliminating the GIL][a-parallelism].
- [Memory management and garbage collection techniques][a-memory-management].

## Frontend

Artichoke will include `ruby` and `irb` [binary frontends][a-frontend] with
dynamically selectable VM backends.

Artichoke will produce a [WebAssembly frontend][wasm-target].

Artichoke will include implementation-agnostic [C APIs][a-c-api] targeting:

- [MRI API][capi-mri] from Ruby.
- [`MRB_API`][capi-mruby] from mruby.

[ruby/spec]: https://github.com/ruby/spec
[wasm-target]: https://github.com/artichoke/artichoke/labels/O-wasm-unknown
[a-single-binary]: https://github.com/artichoke/artichoke/labels/A-single-binary
[a-deps]: https://github.com/artichoke/artichoke/labels/A-deps
[a-codegen]: https://github.com/artichoke/artichoke/labels/A-codegen
[a-compiler]: https://github.com/artichoke/artichoke/labels/A-compiler
[a-parallelism]: https://github.com/artichoke/artichoke/labels/A-parallelism
[a-memory-management]:
  https://github.com/artichoke/artichoke/labels/A-memory-management
[`artichoke-core`]: https://artichoke.github.io/artichoke/artichoke_core/
[ruby `value`]:
  https://artichoke.github.io/artichoke/artichoke_core/value/trait.Value.html
[interoperability]:
  https://artichoke.github.io/artichoke/artichoke_core/convert/index.html
[core-eval]:
  https://artichoke.github.io/artichoke/artichoke_core/eval/trait.Eval.html
[core-converter]:
  https://artichoke.github.io/artichoke/artichoke_core/convert/trait.ConvertMut.html
[core-intern]:
  https://artichoke.github.io/artichoke/artichoke_core/intern/trait.Intern.html
[a-ruby-core]: https://github.com/artichoke/artichoke/labels/A-ruby-core
[a-ruby-stdlib]: https://github.com/artichoke/artichoke/labels/A-ruby-stdlib
[extn-regexp]: artichoke-backend/src/extn/core/regexp
[file system backends]:
  https://github.com/artichoke/artichoke/labels/A-filesystem
[virtual file system]:
  https://artichoke.github.io/artichoke/artichoke_backend/load_path/index.html
[extn-env]: artichoke-backend/src/extn/core/env
[a-optional-stdlib]:
  https://github.com/artichoke/artichoke/labels/A-optional-stdlib
[a-performance]: https://github.com/artichoke/artichoke/labels/A-performance
[a-frontend]: https://github.com/artichoke/artichoke/labels/A-frontend
[b-mruby]: https://github.com/artichoke/artichoke/labels/B-mruby
[b-mri]: https://github.com/artichoke/artichoke/labels/B-MRI
[b-artichoke]: https://github.com/artichoke/artichoke/labels/B-Artichoke
[a-ruby-language]: https://github.com/artichoke/artichoke/labels/A-ruby-language
[a-c-api]: https://github.com/artichoke/artichoke/labels/A-C-API
[capi-mri]: https://github.com/artichoke/artichoke/labels/CAPI-MRI
[capi-mruby]: https://github.com/artichoke/artichoke/labels/CAPI-mruby
