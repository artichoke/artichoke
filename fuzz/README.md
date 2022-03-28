# Fuzzing Artichoke

This cargo workspace is used to set up fuzzing harnesses for Artichoke with
[cargo-fuzz]. Fuzzing or fuzz testing is a technique that looks for bugs by
exploring libraries with random inputs.

[cargo-fuzz]: https://rust-fuzz.github.io/book/cargo-fuzz.html

## Fuzzing Harnesses

cargo-fuzz allows setting up a harness that takes some random input and invokes
some code in the target library. The fuzzing harnesses in this workspace live in
[`fuzz_targets`].

[`fuzz_targets`]: fuzz_targets

This fuzzing harness is [invoked daily in CI][fuzzer-ci].

[fuzzer-ci]: ../.github/workflows/fuzz.yaml

### Eval

[`fuzz_targets/eval.rs`] accepts arbitrary bytes and feeds them to Artichoke's
[`eval`] endpoint. This fuzzer treats the arbitrary byte input as Ruby code and
attempts to execute it on a new interpreter.

[`fuzz_targets/eval.rs`]: fuzz_targets/eval.rs
[`eval`]:
  https://artichoke.github.io/artichoke/artichoke/struct.Artichoke.html#method.eval

## Trophy Case

This fuzzing harness has found several bugs in Artichoke and its dependencies:

- [artichoke/artichoke#931]: Memory leak when constructing `class::Spec`.
- [mruby/mruby#5676]: Infinite loop in parser for heredocs with empty delimiter.

[artichoke/artichoke#931]: https://github.com/artichoke/artichoke/pull/931
[mruby/mruby#5676]: https://github.com/mruby/mruby/issues/5676
