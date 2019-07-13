#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::CactusRef;

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    _inner: String,
    _link: Option<CactusRef<RStringWrapper>>,
}

struct RStringWrapper(RefCell<RString>);

#[test]
fn cactusref_chain_no_leak() {
    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("CactusRef chain no leak", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = CactusRef::new(RStringWrapper(RefCell::new(RString {
            _inner: s.clone(),
            _link: None,
        })));
        let mut last = CactusRef::clone(&first);
        for _ in 1..10 {
            let obj = CactusRef::new(RStringWrapper(RefCell::new(RString {
                _inner: s.clone(),
                _link: Some(CactusRef::clone(&last)),
            })));
            last = obj;
        }
        drop(first);
        drop(last);
    });
}
