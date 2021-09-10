// These integration tests checks for memory leaks that stem from improper
// arena handling in `Value::funcall`.
//
// Checks for memory leaks stemming from improperly grabage collecting Ruby
// objects created in C functions, like the call to `sys::mrb_funcall_argv`.
//
// This test creates a 1MB Ruby string and calls `dup` in a loop. The test
// reuses one artichoke interpreter for all `ITERATIONS`.

use artichoke_backend::prelude::*;

const ITERATIONS: usize = 100;

#[test]
fn arena() {
    let mut interp = artichoke_backend::interpreter().unwrap();
    let s = interp.try_convert_mut("a".repeat(1024 * 1024)).unwrap();

    let mut expected = String::from('"');
    expected.push_str(&"a".repeat(1024 * 1024));
    expected.push('"');
    let expected = expected;

    for _ in 0..ITERATIONS {
        // we have to call a function that calls into the Ruby VM, so we can't
        // just use `to_s`.
        let inspect = s.funcall(&mut interp, "inspect", &[], None).unwrap();
        let inspect = inspect.try_convert_into_mut::<String>(&mut interp).unwrap();
        assert_eq!(inspect, expected);
        interp.incremental_gc().unwrap();
    }
    interp.close();
}
