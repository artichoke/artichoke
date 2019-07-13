use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::link::Link;
use crate::ptr::RcBoxPtr;
use crate::{Rc, Reachable};

mod drop;

trait DetectCycles {
    fn reachable_objects(this: &Self) -> HashSet<usize>;

    fn cycle_objects(this: &Self) -> HashSet<usize>;

    fn cycle_owned_refs_for(this: &Self, object_id: usize) -> Option<usize>;

    fn is_orphaned_cycle(this: &Self) -> bool;
}

impl<T: ?Sized + Reachable> DetectCycles for Rc<T> {
    fn reachable_objects(this: &Self) -> HashSet<usize> {
        reachable_links(this)
            .iter()
            .map(|link| link.value().object_id())
            .collect::<HashSet<_>>()
    }

    fn cycle_objects(this: &Self) -> HashSet<usize> {
        cycle_refs(this)
            .keys()
            .map(|link| link.value().object_id())
            .collect::<HashSet<_>>()
    }

    fn cycle_owned_refs_for(this: &Self, object_id: usize) -> Option<usize> {
        cycle_refs(this)
            .into_iter()
            .map(|(k, v)| (k.value().object_id(), v))
            .collect::<HashMap<_, _>>()
            .get(&object_id)
            .copied()
    }

    fn is_orphaned_cycle(this: &Self) -> bool {
        let cycle = cycle_refs(this);
        if cycle.is_empty() {
            return false;
        }
        let has_external_owners = cycle.iter().any(|(item, cycle_owned_refs)| {
            unsafe { item.0.as_ref() }.strong() > *cycle_owned_refs
        });
        !has_external_owners
    }
}

pub(crate) fn reachable_links<T: ?Sized + Reachable>(this: &Rc<T>) -> HashSet<Link<T>> {
    // Perform a breadth first search over all of the links to determine the
    // clique of refs that self can reach.
    let mut clique = HashSet::new();
    clique.insert(Link(this.ptr));
    loop {
        let size = clique.len();
        for item in clique.clone() {
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

pub(crate) fn cycle_refs<T: ?Sized + Reachable>(this: &Rc<T>) -> HashMap<Link<T>, usize> {
    // Iterate over the items in the clique. For each pair of nodes, find nodes
    // that can reach each other. These nodes form a cycle.
    let mut cycle_owned_refs = HashMap::new();
    let clique = reachable_links(this);
    for (left, right) in clique
        .iter()
        .cartesian_product(clique.iter())
        .filter(|(left, right)| left != right)
    {
        let left_reaches_right = left.value().can_reach(right.value().object_id());
        let right_reaches_left = right.value().can_reach(left.value().object_id());
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
