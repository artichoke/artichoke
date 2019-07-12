#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, CactusRef, Reachable};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

static mut CLOSED_CYCLE: bool = true;

struct RString {
    _inner: String,
    object_id: usize,
}

unsafe impl Reachable for RString {
    fn object_id(&self) -> usize {
        self.object_id
    }

    fn can_reach(&self, _object_id: usize) -> bool {
        unsafe { CLOSED_CYCLE }
    }
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
        unsafe {
            CLOSED_CYCLE = false;
        }
        // each iteration creates 2MB of `String`s
        let first = CactusRef::new(RString {
            _inner: s.clone(),
            object_id: 0,
        });
        let last = CactusRef::new(RString {
            _inner: s.clone(),
            object_id: 1,
        });
        CactusRef::adopt(&first, &last);
        CactusRef::adopt(&last, &first);
        unsafe {
            CLOSED_CYCLE = true;
        }
        drop(first);
        drop(last);
    });
}
