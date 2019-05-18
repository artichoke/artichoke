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
mruby 2.0 [2.0.1-sys.3.c078758]
[Compiled with rustc 1.36.0-nightly a19cf18 2019-05-06]
>>> 12 +
... 25
=> 37
>>> 12.times.map do |i|
...   i.to_s
... end
=> ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"]
>>> def foo
...   'bar'
... end
=> :foo
>>> foo
=> "bar"
>>> not_foo
Backtrace:
    (rirb):10: undefined method 'not_foo' (NoMethodError)
    (rirb):10
>>> raise "oh no!"
Backtrace:
    (rirb):11: oh no! (RuntimeError)
    (rirb):11
>>> [3, 6, 9].inject(0) do |sum, i|
...   sum += i
...
^C
>>>
```
