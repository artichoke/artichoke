// These integration tests checks for memory leaks that stem from not
// deallocating native Rust objects, embedded `mrb_value` data pointers, and
// linked Rust data.
//
// The test exposes a `Container` class to mruby which is initialized with a
// 1MB `String`. The test creates a new mruby interpreter, loads the Container
// class into the interpreter, and initializes one instance `ITERATIONS` times.
//
// This test fails before commit
// `34ee3ddc1c5f4eb1d20f19dd772b0ca348391b2f` with a fairly massive leak.

use artichoke_backend::gc::MrbGarbageCollection;

const ITERATIONS: usize = 10_000;

#[test]
fn full_gc_repeatedly() {
    let mut interp = artichoke_backend::interpreter().unwrap();
    for _ in 0..ITERATIONS {
        interp.full_gc();
    }
    interp.close();
}

#[test]
fn incremental_gc_repeatedly() {
    let mut interp = artichoke_backend::interpreter().unwrap();
    for _ in 0..ITERATIONS {
        interp.incremental_gc();
    }
    interp.full_gc();
    interp.close();
}
