#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;
use std::rc::Rc;

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    _inner: String,
    _link: Option<Rc<RefCell<Self>>>,
    _object_id: usize,
}

#[test]
fn rc_chain_no_leak() {
    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("Rc chain no leak", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = Rc::new(RefCell::new(RString {
            _inner: s.clone(),
            _link: None,
            _object_id: 0,
        }));
        let mut last = Rc::clone(&first);
        for object_id in 1..10 {
            let obj = Rc::new(RefCell::new(RString {
                _inner: s.clone(),
                _link: Some(Rc::clone(&last)),
                _object_id: object_id,
            }));
            last = obj;
        }
        drop(first);
        drop(last);
    });
}
