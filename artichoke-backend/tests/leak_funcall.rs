#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

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

use artichoke_backend::convert::Convert;
use artichoke_backend::gc::MrbGarbageCollection;
use artichoke_core::value::Value as _;

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 30;

#[test]
fn funcall_arena() {
    let interp = artichoke_backend::interpreter().expect("init");
    let s = interp.convert("a".repeat(1024 * 1024));

    leak::Detector::new("ValueLike::funcall", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let expected = format!(r#""{}""#, "a".repeat(1024 * 1024));
        // we have to call a function that calls into the Ruby VM, so we can't
        // just use `to_s`.
        let inspect = s.funcall::<String>("inspect", &[], None).unwrap();
        assert_eq!(inspect, expected);
        interp.incremental_gc();
    });
}
