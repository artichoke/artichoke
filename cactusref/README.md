# 🌵 `CactusRef`

Single-threaded, cycle-aware, reference-counting pointers. 'Rc' stands for
'Reference Counted'.

The type
[`Rc<T>`](https://lopopolo.github.io/ferrocarril/cactusref/struct.Rc.html)
provides shared ownership of a value of type `T`, allocated in the heap.
Invoking
[`clone`](https://lopopolo.github.io/ferrocarril/cactusref/struct.Rc.html#impl-Clone)
on [`Rc`](https://lopopolo.github.io/ferrocarril/cactusref/struct.Rc.html)
produces a new pointer to the same value in the heap. When the last externally
reachable
[`Rc`](https://lopopolo.github.io/ferrocarril/cactusref/struct.Rc.html) pointer
to a given value is destroyed, the pointed-to value is also destroyed.

`Rc` can **detect and deallocate cycles** of `Rc`s through the use of
[`Adoptable`](https://lopopolo.github.io/ferrocarril/cactusref/trait.Adoptable.html).
Cycle detection is a zero-cost abstraction.

🌌 `CactusRef` depends on _several_ unstable Rust features and can only be built
on nightly. `CactusRef` implements
[`std::rc`](https://doc.rust-lang.org/std/rc/index.html)'s pinning APIs which
requires at least Rust 1.33.0.

## `CactusRef` vs. `std::rc`

The `Rc` in `CactusRef` is derived from
[`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) and `CactusRef`
implements most of the API from `std`.

`cactusref::Rc` does not implement the following APIs that are present on
[`rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html):

- [`std::rc::Rc::downcast`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.downcast)
- [`CoerceUnsized`](https://doc.rust-lang.org/nightly/core/ops/trait.CoerceUnsized.html)
- [`DispatchFromDyn`](https://doc.rust-lang.org/nightly/core/ops/trait.DispatchFromDyn.html)

If you do not depend on these APIs, `cactusref` is a drop-in replacement for
[`std::rc`](https://doc.rust-lang.org/std/rc/index.html).

## Cycle Detection

`Rc` implements
[`Adoptable`](https://lopopolo.github.io/ferrocarril/cactusref/trait.Adoptable.html)
to log bookkeeping entries for strong ownership links to other `Rc`s that may
form a cycle. The ownership links tracked by these bookkeeping entries form an
object graph of reachable `Rc`s. On `drop`, `Rc` uses these entries to conduct a
reachability trace of the object graph to determine if it is part of an
_orphaned cycle_. An orphaned cycle is a cycle where the only strong references
to all nodes in the cycle come from other nodes in the cycle.

Cycle detection is a zero-cost abstraction. If you never
`use cactusref::Adoptable;`, `Drop` uses the same implementation as
`std::rc::Rc` (and leaks in the same way as `std::rc::Rc` if you form a cycle of
strong references). The only costs you pay are the memory costs of two empty
[`RefCell`](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html)`<`[`HashMap`](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html)`<T>>`
for tracking adoptions and an if statement to check if these structures are
empty on `drop`.

Cycle detection uses breadth-first search for traversing the object graph. The
algorithm supports arbitrarily large object graphs and will not overflow the
stack during the reachability trace.

## Self-Referential Structures

`CactusRef` can be used to implement collections that own strong references to
themselves. The following implements a doubly-linked list that is fully
deallocated once the `list` binding is dropped.

```rust
use cactusref::{Adoptable, Rc};
use std::cell::RefCell;
use std::iter;

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

let list = iter::repeat(())
    .map(|_| "a".repeat(1024 * 1024))
    .take(10)
    .collect::<Vec<_>>();
let list = DoublyLinkedList::from(list);
drop(list);
// all memory consumed by the list nodes is reclaimed.
```

## License

CactusRef is licensed with the MIT License (c) Ryan Lopopolo.

CactusRef is derived from `Rc` in the Rust standard library v1.38.0 which is
dual licensed with the MIT License and Apache 2.0 License.
