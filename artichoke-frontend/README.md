# artichoke-frontend

Crate artichoke-frontend provides binaries for interacting with the Ruby
interpreter implemented in the [artichoke-backend](/artichoke-backend).

## `airb`

`airb` is the Artichoke implementation of `irb` and is an interactive Ruby shell
and [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).

`airb` is a readline enabled shell, although it does not persist history.

To invoke `airb`, run:

```shell
cargo run --bin airb
```

## `artichoke`

`artichoke`, which is also aliased to `ruby`, is the `ruby` binary frontend to
Artichoke.

`artichoke` supports executing programs via files, stdin, or inline with one or
more `-e` flags.

Artichoke does not yet support reading from the local filesystem. A temporary
workaround is to inject data into the interpreter with the `--with-fixture`
flag, which reads file contents into a `$fixture` global.

```console
$ cargo run --bin artichoke -- --help
artichoke 0.1.0
Artichoke is a Ruby made with Rust.

USAGE:
    artichoke [FLAGS] [OPTIONS] [--] [programfile]

FLAGS:
        --copyright    print the copyright
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -e <commands>...                one line of script. Several -e's allowed. Omit [programfile]
        --with-fixture <fixture>

ARGS:
    <programfile>
```
