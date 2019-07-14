use std::collections::{HashMap, HashSet};

use crate::link::{Link, Links};
use crate::ptr::RcBoxPtr;
use crate::Rc;

mod drop;

trait DetectCycles<T: ?Sized> {
    fn can_reach(this: &Self, other: &Self) -> bool;

    fn orphaned_cycle(this: &Self) -> Option<HashMap<Link<T>, usize>>;
}

impl<T: ?Sized> DetectCycles<T> for Rc<T> {
    fn can_reach(this: &Self, other: &Self) -> bool {
        reachable_links(Link(this.ptr)).contains(&Link(other.ptr))
    }

    fn orphaned_cycle(this: &Self) -> Option<HashMap<Link<T>, usize>> {
        let cycle = cycle_refs(Link(this.ptr));
        if cycle.is_empty() {
            return None;
        }
        let has_external_owners = cycle.iter().any(|(item, cycle_owned_refs)| {
            unsafe { item.0.as_ref() }.strong() > *cycle_owned_refs
        });
        if has_external_owners {
            None
        } else {
            Some(cycle)
        }
    }
}

// Perform a breadth first search over all of the links to determine the clique
// of refs that self can reach.
fn reachable_links<T: ?Sized>(this: Link<T>) -> Links<T> {
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
fn cycle_refs<T: ?Sized>(this: Link<T>) -> HashMap<Link<T>, usize> {
    // Map of Link to number of strong references held by the cycle.
    let mut cycle_owned_refs = HashMap::default();
    // `this` may have strong references to itself.
    cycle_owned_refs.insert(this, this.self_link());
    let mut seen = HashSet::new();
    seen.insert((this, this));
    loop {
        let size = seen.len();
        for item in cycle_owned_refs.clone().keys() {
            let links = unsafe { item.0.as_ref() }.links.borrow();
            for (link, strong) in links.iter() {
                // Forward references contribute to strong ownership counts.
                if !seen.contains(&(*item, *link)) {
                    *cycle_owned_refs.entry(*link).or_insert(0) += strong;
                    seen.insert((*item, *link));
                }
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
        if size == seen.len() {
            break;
        }
    }
    trace!(
        "cactusref reachability test found (strong, cycle) counts: {:?}",
        cycle_owned_refs
            .iter()
            .map(|(item, cycle_count)| {
                let strong = unsafe { item.0.as_ref() }.strong();
                (strong, cycle_count)
            })
            .collect::<Vec<_>>()
    );
    cycle_owned_refs
}
