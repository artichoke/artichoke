use itertools::Itertools;
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

pub(crate) fn cycle_refs<T: ?Sized>(this: Link<T>) -> HashMap<Link<T>, usize> {
    // Iterate over the items in the clique. For each pair of nodes, find nodes
    // that can reach each other. These nodes form a cycle.
    let mut cycle_owned_refs = HashMap::default();
    let clique = reachable_links(this);
    for (left, right) in clique
        .iter()
        .cartesian_product(clique.iter())
        .filter(|(left, right)| left != right)
    {
        let left_reaches_right = reachable_links(*left).contains(right);
        let right_reaches_left = reachable_links(*right).contains(left);
        let is_new = !cycle_owned_refs
            .keys()
            .any(|item: &Link<T>| *item == *right);
        if left_reaches_right && right_reaches_left && is_new {
            let count = *cycle_owned_refs.entry(*right).or_insert(0);
            cycle_owned_refs.insert(*right, count + 1);
        }
    }
    cycle_owned_refs
}
