# mruby-bin

Crate mruby-bin provides binaries for interacting with the mruby interpreter in
the [mruby crate](/mruby).

## rirb

`rirb` is a Rust implementation of `irb` and is an interactive mruby shell and
[REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).
`rirb` includes all extensions that are implemented as part of the `mruby`
crate.

`rirb` is a readline enabled shell, although it does not persist history.

To invoke `rirb`, run:

```shell
cargo run -p mruby-bin rirb
```

The REPL looks like this:

```console
[2.0 (v2.0.1)] > 12 +
[2.0 (v2.0.1)] * 25
=> 37
[2.0 (v2.0.1)] > 12.times.map do |i|
[2.0 (v2.0.1)] * i.to_s
[2.0 (v2.0.1)] * end
=> ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"]
[2.0 (v2.0.1)] > def foo
[2.0 (v2.0.1)] * 'foo'
[2.0 (v2.0.1)] * end
=> :foo
[2.0 (v2.0.1)] > foo
=> "foo"
[2.0 (v2.0.1)] > undefined_method
mruby exception: (rirb):1: undefined method 'undefined_method' (NoMethodError)
(rirb):1
[2.0 (v2.0.1)] > raise 'error'
mruby exception: (rirb):1: error (RuntimeError)
(rirb):1
[2.0 (v2.0.1)] >
```
