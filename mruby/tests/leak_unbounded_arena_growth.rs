#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

//! This integration test checks for memory leaks that stem from improper
//! handling of `mrb_state`.
//!
//! Checks for memory leaks stemming from improperly grabage collecting Ruby
//! objects created in C functions, like the call to exc.inspect in
//! [`MrbApi::current_exception`].
//!
//! The test exposes a function that raises an exception with a 1MB `String`
//! message. The test reuses one mruby interpreter for all `ITERATIONS`.
//!
//! This test calls [`Value::to_s`] and [`Value::to_s_debug'] on a 1MB `String`.
//!
//! If resident memory increases more than 10MB during the test, we likely are
//! leaking memory.
//!
//! This test fails before commit
//! `a450ca7c458d0a4db6fdc60375d8c2c8482c85a7` with a fairly massive leak.

use mruby::eval::MrbEval;
use mruby::gc::MrbGarbageCollection;
use mruby::interpreter::Interpreter;
use mruby::MrbError;
use std::rc::Rc;

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 15;

#[test]
fn unbounded_arena_growth() {
    // MrbApi::current_exception
    let interp = Interpreter::create().expect("mrb init");
    let code = r#"
def bad_code
  raise ArgumentError.new("n" * 1024 * 1024)
end
    "#;
    interp.eval(code.trim()).expect("eval");
    let expected = format!(
        r#"
(eval):2: {} (ArgumentError)
(eval):2:in bad_code
(eval):1
        "#,
        "n".repeat(1024 * 1024)
    );
    leak::Detector::new("current exception", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let interp = Rc::clone(&interp);
        let code = "bad_code";
        let arena = interp.create_arena_savepoint();
        let result = interp.eval(code).map(|_| ());
        arena.restore();
        assert_eq!(result, Err(MrbError::Exec(expected.trim().to_owned())));
        drop(result);
        interp.incremental_gc();
    });

    // Value::to_s
    let interp = Interpreter::create().expect("mrb init");
    let expected = "a".repeat(1024 * 1024);
    leak::Detector::new("to_s", ITERATIONS, LEAK_TOLERANCE).check_leaks_with_finalizer(
        |_| {
            let interp = Rc::clone(&interp);
            let arena = interp.create_arena_savepoint();
            let result = interp.eval("'a' * 1024 * 1024").expect("eval");
            arena.restore();
            assert_eq!(result.to_s(), expected);
            drop(result);
            interp.incremental_gc();
        },
        || Rc::clone(&interp).full_gc(),
    );

    // Value::to_s_debug
    let interp = Interpreter::create().expect("mrb init");
    let expected = format!(r#"String<"{}">"#, "a".repeat(1024 * 1024));
    leak::Detector::new("to_s_debug", ITERATIONS, 3 * LEAK_TOLERANCE).check_leaks_with_finalizer(
        |_| {
            let interp = Rc::clone(&interp);
            let arena = interp.create_arena_savepoint();
            let result = interp.eval("'a' * 1024 * 1024").expect("eval");
            arena.restore();
            assert_eq!(result.to_s_debug(), expected);
            drop(result);
            interp.incremental_gc();
        },
        || Rc::clone(&interp).full_gc(),
    );
}
