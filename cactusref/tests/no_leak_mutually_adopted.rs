#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, CactusRef};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    _inner: String,
}

#[test]
fn cactusref_mutually_adopted_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef mutually adopted pointers",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        // each iteration creates 2MB of `String`s
        let first = CactusRef::new(RString { _inner: s.clone() });
        let last = CactusRef::new(RString { _inner: s.clone() });
        CactusRef::adopt(&first, &last);
        CactusRef::adopt(&last, &first);
        drop(first);
        drop(last);
    });
}
