// This test may segfault because Reachable is implemented in a
// dangerous way.
#![cfg(__disabled__)]
#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::{CactusRef, Reachable};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    _inner: String,
    object_id: usize,
}

struct RStringWrapper(RefCell<RString>);

unsafe impl Reachable for RStringWrapper {
    fn object_id(&self) -> usize {
        self.0.borrow().object_id
    }

    fn can_reach(&self, _object_id: usize) -> bool {
        false
    }
}

#[test]
#[should_panic]
fn cactusref_adopt_no_reachability_leak() {
    let s = "a".repeat(1024 * 1024 * 25);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef adopt leaks when no objects report reachable",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = CactusRef::new(RStringWrapper(RefCell::new(RString {
            _inner: s.clone(),
            object_id: 0,
        })));
        let mut last = CactusRef::clone(&first);
        for object_id in 1..10 {
            let obj = CactusRef::new(RStringWrapper(RefCell::new(RString {
                _inner: s.clone(),
                object_id,
            })));
            CactusRef::adopt(&obj, &last);
            last = obj;
        }
        CactusRef::adopt(&first, &last);
        drop(first);
        drop(last);
    });
}
