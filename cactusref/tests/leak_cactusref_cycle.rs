#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::{CactusRef as Rc, Reachable};

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    inner: String,
    link: Option<Rc<RStringWrapper>>,
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
fn cactusref_cycle_leak() {
    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("CactusRef cycle leaks", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = Rc::new(RStringWrapper(RefCell::new(RString {
            inner: s.clone(),
            link: None,
            object_id: 0,
        })));
        let mut last = Rc::clone(&first);
        for object_id in 1..10 {
            let obj = Rc::new(RStringWrapper(RefCell::new(RString {
                inner: s.clone(),
                link: Some(Rc::clone(&last)),
                object_id,
            })));
            last = obj;
            println!("{} ...", &last.0.borrow().inner[..10]);
        }
        first.0.borrow_mut().link = Some(Rc::clone(&last));
        drop(first);
        drop(last);
    });
}
