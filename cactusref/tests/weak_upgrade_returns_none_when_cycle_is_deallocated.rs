#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::{Adoptable, CactusRef};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RArray {
    inner: Vec<CactusRef<RefCell<Self>>>,
    _alloc: String,
}

#[test]
fn weak_upgrade_returns_none_when_cycle_is_deallocated() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(2 * 1024 * 1024);

    // 100MB of empty buffers will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef Weak::upgrade on cycle drop",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        // each iteration creates 2MB of empty buffers
        let vec = CactusRef::new(RefCell::new(RArray {
            inner: vec![],
            _alloc: s.clone(),
        }));
        for _ in 0..10 {
            vec.borrow_mut().inner.push(CactusRef::clone(&vec));
            CactusRef::adopt(&vec, &vec);
        }
        assert_eq!(CactusRef::strong_count(&vec), 11);
        let weak = CactusRef::downgrade(&vec);
        assert!(weak.upgrade().is_some());
        drop(vec);
        assert!(weak.upgrade().is_none());
    });
}
