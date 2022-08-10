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

### `Kernel#Integer`

The [`Kernel#Integer`] method in Ruby Core is a parser that takes a byte string
and interprets that string as a number literal. The literal can have `0b`, `0o`,
`0d`, `0x` and `0` prefixes indicating the radix, zero padding, and underscore
separators.

[`fuzz_targets/kernel_integer_bytes.rs`] accepts arbitrary bytes and feeds them
to [`scolapasta-int-parse`] with a variety of radixes.
[`fuzz_targets/kernel_integer_str.rs`] does the same but fuzzes with valid UTF-8
inputs. [`fuzz_targets/kernel_integer_int.rs`] takes `i64` inputs, formats them
to a `String`, and then passes the well-formed numeric string to
[`scolapasta-int-parse`].

[`kernel#integer`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-Integer
[`fuzz_targets/kernel_integer_bytes.rs`]: fuzz_targets/kernel_integer_bytes.rs
[`fuzz_targets/kernel_integer_str.rs`]: fuzz_targets/kernel_integer_str.rs
[`fuzz_targets/kernel_integer_int.rs`]: fuzz_targets/kernel_integer_int.rs
[`scolapasta-int-parse`]: ../scolapasta-int-parse

## Trophy Case

This fuzzing harness has found several bugs in Artichoke and its dependencies:

- [artichoke/artichoke#931]: Memory leak when constructing `class::Spec`.
- [mruby/mruby#5676]: Infinite loop in parser for heredocs with empty delimiter.

[artichoke/artichoke#931]: https://github.com/artichoke/artichoke/pull/931
[mruby/mruby#5676]: https://github.com/mruby/mruby/issues/5676
