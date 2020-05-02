#![deny(clippy::all)]
#![deny(clippy::pedantic)]

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

use artichoke_backend::prelude::core::*;
use artichoke_backend::prelude::*;

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 15;

#[test]
fn unbounded_arena_growth() {
    // ArtichokeApi::current_exception
    let mut interp = artichoke_backend::interpreter().unwrap();
    let code = r#"
def bad_code
  raise ArgumentError.new("n" * 1024 * 1024)
end
    "#;
    let _ = interp.eval(code.trim().as_bytes()).unwrap();
    let expected = Some(vec![
        Vec::from(&b"(eval):2:in bad_code"[..]),
        Vec::from(&b"(eval):1"[..]),
    ]);
    leak::Detector::new("current exception", &mut interp)
        .with_iterations(ITERATIONS)
        .with_tolerance(LEAK_TOLERANCE)
        .check_leaks(|interp| {
            let code = b"bad_code";
            let arena = interp.create_arena_savepoint();
            let result = interp.eval(code).unwrap_err();
            arena.restore();
            assert_eq!(expected, result.vm_backtrace(interp));
            drop(result);
            interp.incremental_gc();
        });
    interp.close();

    // Value::to_s
    let mut interp = artichoke_backend::interpreter().unwrap();
    let expected = "a".repeat(1024 * 1024);
    leak::Detector::new("to_s", &mut interp)
        .with_iterations(ITERATIONS)
        .with_tolerance(LEAK_TOLERANCE)
        .check_leaks_with_finalizer(
            |interp| {
                let mut interp = interp.clone();
                let arena = interp.create_arena_savepoint();
                let result = interp.eval(b"'a' * 1024 * 1024").unwrap();
                arena.restore();
                assert_eq!(result.to_s(&mut interp), expected.as_bytes());
                drop(result);
                interp.incremental_gc();
            },
            |interp| interp.full_gc(),
        );
    interp.close();

    // Value::inspect
    let mut interp = artichoke_backend::interpreter().unwrap();
    let expected = format!(r#""{}""#, "a".repeat(1024 * 1024)).into_bytes();
    leak::Detector::new("inspect", &mut interp)
        .with_iterations(ITERATIONS)
        .with_tolerance(LEAK_TOLERANCE)
        .check_leaks_with_finalizer(
            |interp| {
                let arena = interp.create_arena_savepoint();
                let result = interp.eval(b"'a' * 1024 * 1024").unwrap();
                arena.restore();
                assert_eq!(result.inspect(interp), expected);
                drop(result);
                interp.incremental_gc();
            },
            |interp| interp.full_gc(),
        );
    interp.close();
}
