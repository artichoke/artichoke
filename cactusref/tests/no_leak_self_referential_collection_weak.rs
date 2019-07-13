#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::{Adoptable, CactusRef, CactusWeakRef};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RArray {
    inner: Vec<CactusWeakRef<RefCell<Self>>>,
    _alloc: String,
}

#[test]
fn cactusref_self_referential_collection_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(2 * 1024 * 1024);

    // 100MB of empty buffers will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef adopted self-referential with weak",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        // each iteration creates 2MB of empty buffers
        let vec = CactusRef::new(RefCell::new(RArray {
            inner: vec![],
            _alloc: s.clone(),
        }));
        for _ in 1..10 {
            vec.borrow_mut().inner.push(CactusRef::downgrade(&vec));
            CactusRef::adopt(&vec, &vec);
        }
        drop(vec);
    });
}
