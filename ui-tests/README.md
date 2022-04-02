# Artichoke UI Tests

This cargo workspace is used to run raw io integration tests on the compiled
binaries (UI) of Artichoke. These UI tests capture and assert stdout/stderr and
the return status after running the Artichoke binaries via the usage of Snapshot
testing via usage of the [insta crate](https://crates.io/crates/insta)

Artichoke encourages unit tests to be written along side the code where it is
implemented. However, some integration tests can help prevent breaking certain
input/output where otherwise it would have been hard to detect.

In short, this workspace acts as the last line of defence against breaking
input/output of Artichoke.

## Usage

Run tests from the project root:

```sh
rake test:ui
```

The above will build the Artichoke binaries into `../target/debug` which is what
this workspace will then use to execute the UI Tests.

## Writing a test

By convention, all tests in this workspace use
[insta](https://crates.io/crates/insta) with TOML snapshots.

UI Tests should ideally test:

- pure input/output of the artichoke binaries
- end user use cases (e.g. not Artchoke internals)
- stable features

### Snapshots

As per insta guidelines, it's recommended to fix run new tests, and let them
fail. Insta will create new snapshots in `tests/fixutres` with a
`.new` extension. When satisifed with the snapshots, the `.new` extension can be
removed, and the snapshot committed along with the test(s).

`cargo insta review` can be used to help the review of the `.new` files.

Note: Snapshots are not automatically pruned, so when removing a test, be sure
to remove the associated snapshots

## Testing harness

`src/lib.rs` provides a `run` function which:

- executes artichoke binary based on the current platform
- captures the output of the binary execution
- provides an interface to serialize the execution and it's results
- ... which can then be used to test the snapshot with `insta`
