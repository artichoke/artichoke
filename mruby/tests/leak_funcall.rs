#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

//! This integration test checks for memory leaks that stem from improper
//! arena handling in `ValueLike::funcall`.
//!
//! Checks for memory leaks stemming from improperly grabage collecting Ruby
//! objects created in C functions, like the call to `sys::mrb_funcall_argv`.
//!
//! This test creates a 1MB Ruby string and calls `dup` in a loop. The test
//! reuses one mruby interpreter for all `ITERATIONS`.
//!
//! If resident memory increases more than 10MB during the test, we likely are
//! leaking memory.

use mruby::gc::MrbGarbageCollection;
use mruby::interpreter::{Interpreter, MrbApi};
use mruby::value::ValueLike;

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 30;

#[test]
fn funcall_arena() {
    let interp = Interpreter::create().expect("mrb init");
    let s = interp.string("a".repeat(1024 * 1024));

    leak::Detector::new("ValueLike::funcall", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let expected = format!(r#""{}""#, "a".repeat(1024 * 1024));
        // we have to call a function that calls into the Ruby VM, so we can't
        // just use `to_s`.
        let inspect = s.funcall::<String, _, _>("inspect", &[]);
        assert_eq!(inspect, Ok(expected));
        interp.incremental_gc();
    });
}
