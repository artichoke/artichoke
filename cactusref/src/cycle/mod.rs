use std::collections::HashMap;

use crate::link::{Link, Links};
use crate::ptr::RcBoxPtr;
use crate::Rc;

mod drop;

trait DetectCycles {
    fn can_reach(this: &Self, other: &Self) -> bool;

    fn is_orphaned_cycle(this: &Self) -> bool;
}

impl<T: ?Sized> DetectCycles for Rc<T> {
    fn can_reach(this: &Self, other: &Self) -> bool {
        reachable_links(Link(this.ptr)).contains(&Link(other.ptr))
    }

    fn is_orphaned_cycle(this: &Self) -> bool {
        let cycle = cycle_refs(Link(this.ptr));
        if cycle.is_empty() {
            return false;
        }
        let has_external_owners = cycle.iter().any(|(item, cycle_owned_refs)| {
            unsafe { item.0.as_ref() }.strong() > *cycle_owned_refs
        });
        !has_external_owners
    }
}

// Perform a breadth first search over all of the links to determine the clique
// of refs that self can reach.
pub(crate) fn reachable_links<T: ?Sized>(this: Link<T>) -> Links<T> {
    let mut clique = Links::default();
    clique.insert(this);
    loop {
        let size = clique.len();
        for (item, _) in clique.clone().iter() {
            let links = unsafe { item.0.as_ref() }.links.borrow();
            for (link, _) in links.iter() {
                clique.insert(*link);
            }
        }
        // BFS has found no new refs in the clique.
        if size == clique.len() {
            break;
        }
    }
    clique
}

// Perform a breadth first search over all of the forward and backward links to
// determine the clique of nodes in a cycle and their strong counts.
pub(crate) fn cycle_refs<T: ?Sized>(this: Link<T>) -> HashMap<Link<T>, usize> {
    // Map of Link to number of strong references held by the cycle.
    let mut cycle_owned_refs = HashMap::default();
    // `this` may have strong references to itself.
    cycle_owned_refs.insert(this, this.self_link());
    loop {
        let size = cycle_owned_refs.len();
        for item in cycle_owned_refs.clone().keys() {
            let links = unsafe { item.0.as_ref() }.links.borrow();
            for (link, strong) in links.iter() {
                // Forward references contribute to strong ownership counts.
                *cycle_owned_refs.entry(*link).or_insert(0) += strong;
            }
            let links = unsafe { item.0.as_ref() }.back_links.borrow();
            for (link, _) in links.iter() {
                // Back references do not contribute to strong ownership counts,
                // but they are added to the set of cycle owned refs so BFS can
                // include them in the reachability analysis.
                cycle_owned_refs.entry(*link).or_insert(0);
            }
        }
        // BFS has found no new refs in the clique.
        if size == cycle_owned_refs.len() {
            break;
        }
    }
    cycle_owned_refs
}
