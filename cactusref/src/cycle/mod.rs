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

pub(crate) fn reachable_links<T: ?Sized>(this: Link<T>) -> Links<T> {
    // Perform a breadth first search over all of the links to determine the
    // clique of refs that self can reach.
    let mut clique = Links::default();
    clique.insert(this);
    loop {
        let size = clique.len();
        for item in clique.clone().iter() {
            let links = unsafe { item.0.as_ref() }.links.borrow();
            for link in links.iter() {
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

pub(crate) fn graph_reachable_links<T: ?Sized>(this: Link<T>) -> Links<T> {
    // Perform a breadth first search over all of the links to determine the
    // clique of refs that self can reach.
    let mut clique = Links::default();
    clique.insert(this);
    loop {
        let size = clique.len();
        for item in clique.clone().iter() {
            let links = unsafe { item.0.as_ref() }.links.borrow();
            for link in links.iter() {
                clique.insert(*link);
            }
            let links = unsafe { item.0.as_ref() }.back_links.borrow();
            for link in links.iter() {
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

pub(crate) fn cycle_refs<T: ?Sized>(this: Link<T>) -> HashMap<Link<T>, usize> {
    // Iterate over the items in the clique. For each pair of nodes, find nodes
    // that can reach each other. These nodes form a cycle.
    let mut cycle_owned_refs = HashMap::default();
    let clique = graph_reachable_links(this);
    let reachable_by_node = clique
        .iter()
        .map(|link| (*link, reachable_links(*link).registry))
        .collect::<HashMap<_, _>>();
    for link in clique.iter() {
        cycle_owned_refs.insert(
            *link,
            reachable_by_node
                .get(link)
                .map(|links| links.len())
                .unwrap_or_default(),
        );
    }
    cycle_owned_refs
}
