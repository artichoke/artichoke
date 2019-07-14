#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![allow(clippy::shadow_unrelated)]

use cactusref::{Adoptable, Rc};
use std::cell::RefCell;
use std::iter;

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct DoublyLinkedList<T> {
    pub prev: Option<Rc<RefCell<Self>>>,
    pub next: Option<Rc<RefCell<Self>>>,
    pub data: T,
}

impl<T> DoublyLinkedList<T> {
    fn from(item: Vec<T>) -> Rc<RefCell<Self>> {
        let mut nodes = item
            .into_iter()
            .map(|data| {
                Rc::new(RefCell::new(Self {
                    prev: None,
                    next: None,
                    data,
                }))
            })
            .collect::<Vec<_>>();
        for i in 0..nodes.len() - 1 {
            let prev = &nodes[i];
            let curr = &nodes[i + 1];
            curr.borrow_mut().prev = Some(Rc::clone(prev));
            Rc::adopt(curr, prev);
        }
        let prev = &nodes[nodes.len() - 1];
        let curr = &nodes[0];
        Rc::adopt(curr, prev);
        curr.borrow_mut().prev = Some(Rc::clone(prev));
        for i in (1..nodes.len()).rev() {
            let prev = &nodes[i];
            let curr = &nodes[i - 1];
            curr.borrow_mut().next = Some(Rc::clone(prev));
            Rc::adopt(curr, prev);
        }
        let prev = &nodes[0];
        let curr = &nodes[nodes.len() - 1];
        Rc::adopt(curr, prev);
        curr.borrow_mut().next = Some(Rc::clone(prev));

        nodes.remove(0)
    }
}

#[test]
fn cactusref_doubly_linked_list_no_leak() {
    env_logger::Builder::from_env("CACTUS_LOG").init();

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("CactusRef adopt self", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        let list = iter::repeat(())
            .map(|_| "a".repeat(1024 * 1024))
            .take(10)
            .collect::<Vec<_>>();
        let list = DoublyLinkedList::from(list);
        drop(list);
    });
}
