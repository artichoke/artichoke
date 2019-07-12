#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, CactusRef, Reachable};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

static mut CLOSED_CYCLE: bool = true;

struct RString {
    inner: String,
    object_id: usize,
}

unsafe impl Reachable for RString {
    fn object_id(&self) -> usize {
        self.object_id
    }

    fn can_reach(&self, object_id: usize) -> bool {
        if unsafe { CLOSED_CYCLE } {
            return true;
        }
        match self.object_id() {
            9 => object_id == 0,
            self_id => self_id + 1 == object_id,
        }
    }
}

#[test]
fn cactusref_adopt_with_reachability_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new(
        "CactusRef adopt when objects report reachable",
        ITERATIONS,
        LEAK_TOLERANCE,
    )
    .check_leaks(|_| {
        unsafe {
            CLOSED_CYCLE = false;
        }
        // each iteration creates 10MB of `String`s
        let first = CactusRef::new(RString {
            inner: s.clone(),
            object_id: 0,
        });
        let mut last = CactusRef::clone(&first);
        for object_id in 1..10 {
            assert_eq!(first.inner, s);
            let obj = CactusRef::new(RString {
                inner: s.clone(),
                object_id,
            });
            CactusRef::adopt(&obj, &last);
            last = obj;
        }
        CactusRef::adopt(&first, &last);
        unsafe {
            CLOSED_CYCLE = true;
        }
        drop(first);
        drop(last);
    });
}
