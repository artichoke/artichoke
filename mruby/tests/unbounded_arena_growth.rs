//! This integration test checks for memory leaks that stem from improper
//! handling of `mrb_state`.
//!
//! # unbounded_arena_growth
//!
//! Checks for memory leaks stemming from improperly grabage collecting Ruby
//! objects created in C functions, like the call to exc.inspect in
//! [`MrbApi::current_exception`].
//!
//! The test exposes a function that raises an exception with a 1MB `String`
//! message. The test reuses one mruby interpreter for all `ITERATIONS`.
//!
//! If resident memory increases more than 10MB during the test, we likely are
//! leaking memory.
//!
//! This test fails up to commit
//! `a450ca7c458d0a4db6fdc60375d8c2c8482c85a7` with a fairly massive leak and
//! the following message:
//!
//! ```txt
//! running 1 test
//! test tests::unbounded_arena_growth ... FAILED
//!
//! failures:
//!
//! ---- tests::unbounded_arena_growth stdout ----
//! thread 'tests::unbounded_arena_growth' panicked at 'Plausible memory leak!
//! After 2000 iterations, usage before: 3518464, usage after: 2419949568',
//! mruby/tests/unbounded_arena_growth.rs:119:9
//! note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
//!
//!
//! failures:
//!     tests::unbounded_arena_growth
//!
//! test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out
//! ```

#[cfg(test)]
mod tests {
    use mruby::interpreter::{Interpreter, MrbApi, MrbError};
    use std::mem;
    use std::rc::Rc;

    const LEAK_TOLERANCE: i64 = 1024 * 1024 * 10;
    const ITERATIONS: usize = 2000;

    fn check_leaks<F>(mut execute: F)
    where
        F: FnMut() -> (),
    {
        let start_mem = resident_memsize();
        for _ in 0..ITERATIONS {
            execute();
        }
        let end_mem = resident_memsize();
        assert!(
            end_mem <= start_mem + LEAK_TOLERANCE,
            "Plausible memory leak!\nAfter {} iterations, usage before: {}, usage after: {}",
            ITERATIONS,
            start_mem,
            end_mem
        );
    }

    fn resident_memsize() -> i64 {
        let mut out: libc::rusage = unsafe { mem::zeroed() };
        assert!(unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut out) } == 0);
        out.ru_maxrss
    }

    #[test]
    fn unbounded_arena_growth() {
        let interp = Interpreter::create().expect("mrb init");
        let code = r#"
            def bad_code
              raise ArgumentError.new("n" * 1024 * 1024)
            end
        "#;
        interp.eval(code).expect("eval");
        let expected = Err(MrbError::Exec(format!(
            "ArgumentError: {}",
            "n".repeat(1024 * 1024)
        )));
        check_leaks(|| {
            let interp = Rc::clone(&interp);
            let code = "bad_code";
            let arena = interp.create_arena_savepoint();
            let result = interp.eval(code).map(|_| ());
            interp.restore_arena(arena);
            assert_eq!(result, expected);
            drop(result);
            interp.incremental_gc();
        })
    }
}
