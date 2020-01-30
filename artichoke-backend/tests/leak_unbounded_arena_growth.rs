#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

//! This integration test checks for memory leaks that stem from improper
//! handling of `mrb_state`.
//!
//! Checks for memory leaks stemming from improperly grabage collecting Ruby
//! objects created in C functions, like the call to exc.inspect in
//! [`ArtichokeApi::current_exception`].
//!
//! The test exposes a function that raises an exception with a 1MB `String`
//! message. The test reuses one artichoke interpreter for all `ITERATIONS`.
//!
//! This test calls [`Value::to_s`] and [`Value::to_s_debug'] on a 1MB `String`.
//!
//! If resident memory increases more than 10MB during the test, we likely are
//! leaking memory.
//!
//! This test fails before commit
//! `a450ca7c458d0a4db6fdc60375d8c2c8482c85a7` with a fairly massive leak.

use artichoke_backend::exception::RubyException;
use artichoke_backend::gc::MrbGarbageCollection;
use artichoke_backend::{Eval, ValueLike};

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 15;

#[test]
fn unbounded_arena_growth() {
    // ArtichokeApi::current_exception
    let interp = artichoke_backend::interpreter().expect("init");
    let code = r#"
def bad_code
  raise ArgumentError.new("n" * 1024 * 1024)
end
    "#;
    let _ = interp.eval(code.trim().as_bytes()).expect("eval");
    let expected = Some(vec![
        Vec::from(&b"(eval):2:in bad_code"[..]),
        Vec::from(&b"(eval):1"[..]),
    ]);
    leak::Detector::new("current exception", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let interp = interp.clone();
        let code = b"bad_code";
        let arena = interp.create_arena_savepoint();
        let result = interp.eval(code).unwrap_err();
        arena.restore();
        assert_eq!(expected, result.backtrace(&interp));
        drop(result);
        interp.incremental_gc();
    });

    // Value::to_s
    let interp = artichoke_backend::interpreter().expect("init");
    let expected = "a".repeat(1024 * 1024);
    leak::Detector::new("to_s", ITERATIONS, LEAK_TOLERANCE).check_leaks_with_finalizer(
        |_| {
            let interp = interp.clone();
            let arena = interp.create_arena_savepoint();
            let result = interp.eval(b"'a' * 1024 * 1024").expect("eval");
            arena.restore();
            assert_eq!(result.to_s(), expected.as_bytes());
            drop(result);
            interp.incremental_gc();
        },
        || interp.clone().full_gc(),
    );

    // Value::to_s_debug
    let interp = artichoke_backend::interpreter().expect("init");
    let expected = format!(r#"String<"{}">"#, "a".repeat(1024 * 1024));
    leak::Detector::new("to_s_debug", ITERATIONS, 3 * LEAK_TOLERANCE).check_leaks_with_finalizer(
        |_| {
            let interp = interp.clone();
            let arena = interp.create_arena_savepoint();
            let result = interp.eval(b"'a' * 1024 * 1024").expect("eval");
            arena.restore();
            assert_eq!(result.to_s_debug(), expected);
            drop(result);
            interp.incremental_gc();
        },
        || interp.clone().full_gc(),
    );
}
