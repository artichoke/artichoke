# Artichoke Ruby VM Design

Artichoke aims to be a source-compatible, [ruby/spec](/spec-runner/spec/ruby)
compliant implementation of Ruby 2.6.3 written in safe Rust (excluding crate
dependencies).

These documents discuss the design of the Artichoke VM.

## Design Document Index

- [Value Representation](value.md)
- [String and Encoding](string.md)
- [Memory Management](memory-management.md): the heap and using Rust memory
  management to get a reference counting GC for free.
- [Threading and Concurrency](threading-and-concurrency.md): True concurrency
  with no GIL.
- [Parser](parser.md)
- [AST](ast.md)
