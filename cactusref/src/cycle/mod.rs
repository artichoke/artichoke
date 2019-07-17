use hashbrown::{HashMap, HashSet};

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
    // These collections track compute the layout of the object graph in linear
    // time in the size of the graph.
    let mut cycle_owned_refs = HashMap::default();
    let mut discovered = vec![this];
    let mut visited = HashSet::new();

    // crawl the graph
    while let Some(node) = discovered.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        let links = unsafe { node.0.as_ref() }.links.borrow();
        for (link, strong) in links.iter() {
            // Forward references contribute to strong ownership counts.
            *cycle_owned_refs.entry(*link).or_insert(0) += strong;
            discovered.push(*link);
        }
        let links = unsafe { node.0.as_ref() }.back_links.borrow();
        for (link, _) in links.iter() {
            // Back references do not contribute to strong ownership counts,
            // but they are added to the set of cycle owned refs so BFS can
            // include them in the reachability analysis.
            cycle_owned_refs.entry(*link).or_insert(0);
        }
    }
    #[cfg(debug_assertions)]
    debug_cycle(&cycle_owned_refs);
    cycle_owned_refs
}

#[inline]
#[cfg(debug_assertions)]
fn debug_cycle<T: ?Sized>(cycle: &HashMap<Link<T>, usize>) {
    let counts = cycle
        .iter()
        .map(|(item, cycle_count)| {
            let strong = unsafe { item.0.as_ref() }.strong();
            (strong, cycle_count)
        })
        .collect::<Vec<_>>();
    trace!(
        "cactusref reachability test found (strong, cycle) counts: {:?}",
        counts
    );
}
