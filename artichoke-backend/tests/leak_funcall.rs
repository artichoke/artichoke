#![deny(clippy::all)]
#![deny(clippy::pedantic)]

//! This integration test checks for memory leaks that stem from improper
//! arena handling in `ValueLike::funcall`.
//!
//! Checks for memory leaks stemming from improperly grabage collecting Ruby
//! objects created in C functions, like the call to `sys::mrb_funcall_argv`.
//!
//! This test creates a 1MB Ruby string and calls `dup` in a loop. The test
//! reuses one artichoke interpreter for all `ITERATIONS`.
//!
//! If resident memory increases more than 10MB during the test, we likely are
//! leaking memory.

use artichoke_backend::prelude::*;

const ITERATIONS: usize = 100;

#[test]
fn funcall_arena_leak() {
    let mut interp = artichoke_backend::interpreter().unwrap();
    let s = interp.convert_mut("a".repeat(1024 * 1024));

    let mut expected = String::from('"');
    expected.push_str(&"a".repeat(1024 * 1024));
    expected.push('"');
    let expected = expected;

    for _ in 0..ITERATIONS {
        // we have to call a function that calls into the Ruby VM, so we can't
        // just use `to_s`.
        let inspect = s.funcall(&mut interp, "inspect", &[], None).unwrap();
        let inspect = inspect.try_into_mut::<String>(&mut interp).unwrap();
        assert_eq!(inspect, expected);
        interp.incremental_gc();
    }
    interp.close();
}
