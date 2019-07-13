#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, CactusRef};

mod leak;

const ITERATIONS: usize = 1;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    inner: String,
}

#[test]
fn cactusref_adopt_with_members_in_multiple_cycles_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef pointers in multiple cycles",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = CactusRef::new(RString {
            inner: s.clone(),
        });
        let mut last = CactusRef::clone(&first);
        for _ in 1..10 {
            assert_eq!(first.inner, s);
            let obj = CactusRef::new(RString {
                inner: s.clone(),
            });
            CactusRef::adopt(&obj, &last);
            last = obj;
        }
        CactusRef::adopt(&first, &last);
        let group1 = first;
        let first = CactusRef::new(RString {
            inner: s.clone(),
        });
        let mut last = CactusRef::clone(&first);
        for _ in 101..110 {
            assert_eq!(first.inner, s);
            let obj = CactusRef::new(RString {
                inner: s.clone(),
            });
            CactusRef::adopt(&obj, &last);
            last = obj;
        }
        CactusRef::adopt(&first, &last);
        let group2 = first;
        // join the two cycles
        CactusRef::adopt(&group2, &group1);
        CactusRef::adopt(&group1, &group2);
        drop(last);
        drop(group2);
        drop(group1);
    });
}
