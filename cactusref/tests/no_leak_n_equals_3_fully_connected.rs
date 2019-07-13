#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, CactusRef};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

#[test]
fn cactusref_n_equals_3_fully_connected_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(1024 * 1024);

    // 300MB of `String`s will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef mutually adopted pointers",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        // each iteration creates 6MB of `String`s
        let n1 = CactusRef::new(s.clone());
        let n2 = CactusRef::new(s.clone());
        let n3 = CactusRef::new(s.clone());
        CactusRef::adopt(&n1, &n2);
        CactusRef::adopt(&n1, &n3);
        CactusRef::adopt(&n2, &n1);
        CactusRef::adopt(&n2, &n3);
        CactusRef::adopt(&n3, &n1);
        CactusRef::adopt(&n3, &n2);
        drop(n1);
        drop(n2);
        drop(n3);
    });
}
