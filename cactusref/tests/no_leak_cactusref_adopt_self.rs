#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::{Adoptable, CactusRef};

mod leak;

const ITERATIONS: usize = 100;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    inner: String,
    link: Option<CactusRef<RefCell<Self>>>,
}

#[test]
fn cactusref_adopt_self_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    let s = "a".repeat(1024 * 1024 * 5);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("CactusRef adopt self", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 5MB of `String`s
        let first = CactusRef::new(RefCell::new(RString {
            inner: s.clone(),
            link: None,
        }));
        first.borrow_mut().link = Some(CactusRef::clone(&first));
        CactusRef::adopt(&first, &first);
        assert_eq!(first.borrow().inner, s);
        drop(first);
    });
}
