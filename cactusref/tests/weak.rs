#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use cactusref::{Adoptable, Rc};
use std::cell::RefCell;

#[derive(Default)]
struct Array {
    buffer: Vec<Rc<RefCell<Self>>>,
}

#[test]
fn cactusref_weak() {
    let array = Rc::new(RefCell::new(Array::default()));
    for _ in 0..10 {
        let item = Rc::clone(&array);
        Rc::adopt(&array, &item);
        array.borrow_mut().buffer.push(item);
    }
    assert_eq!(Rc::strong_count(&array), 11);

    let weak = Rc::downgrade(&array);
    assert!(weak.upgrade().is_some());
    assert_eq!(weak.strong_count(), 11);
    assert_eq!(weak.weak_count(), Some(1));
    assert_eq!(weak.upgrade().unwrap().borrow().buffer.len(), 10);

    // 1 for the array binding, 10 for the `Rc`s in buffer, and 10
    // for the self adoptions.
    assert_eq!(Rc::strong_count(&array), 11);

    drop(array);

    assert!(weak.upgrade().is_none());
    assert_eq!(weak.weak_count(), Some(1));
}
