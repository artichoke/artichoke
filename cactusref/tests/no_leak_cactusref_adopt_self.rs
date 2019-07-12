#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, CactusRef, Reachable};

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    _inner: String,
    object_id: usize,
}

unsafe impl Reachable for RString {
    fn object_id(&self) -> usize {
        self.object_id
    }

    fn can_reach(&self, _object_id: usize) -> bool {
        false
    }
}

#[test]
fn cactusref_adopt_self_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(1024 * 1024 * 5);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("CactusRef adopt self", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 25MB of `String`s
        let first = CactusRef::new(RString {
            _inner: s.clone(),
            object_id: 0,
        });
        CactusRef::adopt(&first, &first);
        drop(first);
    });
}
