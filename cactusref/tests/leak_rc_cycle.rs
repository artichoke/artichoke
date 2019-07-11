#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;
use std::rc::Rc;

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    _inner: String,
    link: Option<Rc<RefCell<Self>>>,
    _object_id: usize,
}

#[test]
#[should_panic]
fn rc_cycle_leak() {
    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("Rc cycle leaks", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = Rc::new(RefCell::new(RString {
            _inner: s.clone(),
            link: None,
            _object_id: 0,
        }));
        let mut last = Rc::clone(&first);
        for object_id in 1..10 {
            let obj = Rc::new(RefCell::new(RString {
                _inner: s.clone(),
                link: Some(Rc::clone(&last)),
                _object_id: object_id,
            }));
            last = obj;
        }
        first.borrow_mut().link = Some(Rc::clone(&last));
        drop(first);
        drop(last);
    });
}
