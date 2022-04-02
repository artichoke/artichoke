# Artichoke UI Tests

This cargo workspace is used to run raw integration tests to validate the
console output of Artichoke's compiled binaries. These UI tests capture and
assert stdout/stderr and the return status after running the Artichoke binaries
with [snapshot testing] via usage of the [insta crate].

[snapshot testing]: https://insta.rs/#hello-snapshot-testing
[insta crate]: https://crates.io/crates/insta

Artichoke encourages unit tests to be written along side the code where it is
implemented. However, some integration tests can help prevent breaking certain
input/output where otherwise it would have been hard to detect.

In short, this workspace acts as the last line of defence against breaking
input/output of Artichoke. They are similar in spirit to [UI tests from rustc].

[ui tests from rustc]: https://rustc-dev-guide.rust-lang.org/tests/ui.html

## Usage

Run tests from the project root:

```sh
bundle exec rake test:ui
```

The above will build the Artichoke binaries into `../target/debug` which is what
this workspace will then use to execute the UI Tests.

## Writing a test

By convention, all tests in this workspace use [insta] with TOML snapshots.

[insta]: https://crates.io/crates/insta

UI Tests should ideally test:

- pure input/output of the artichoke binaries
- end user use cases (e.g. not Artichoke internals)
- stable features

### Snapshots

As per insta guidelines, it's recommended to fix run new tests, and let them
fail. Insta will create new snapshots in `tests/fixutres` with a `.new`
extension. When satisifed with the snapshots, the `.new` extension can be
removed, and the snapshot committed along with the test(s).

`cargo insta review` can be used to help the review of the `.new` files.

Note: Snapshots are not automatically pruned, so when removing a test, be sure
to remove the associated snapshots

## Testing harness

Executing the UI Tests involves locating, executing, and serializing of the io
and status codes. The testing harness in [`src/lib.rs`](src/lib.rs) simplifies
this with a `run` function (platform aware), providing an interface which
`insta` can use to serialize the results into the snapshots.

For consistency, it's best to use the test harness and/or enhance it if further
functionality is required.
