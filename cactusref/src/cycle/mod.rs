use hashbrown::{HashMap, HashSet};

use crate::link::{Kind, Link};
use crate::ptr::RcBoxPtr;
use crate::Rc;

mod drop;

trait DetectCycles<T: ?Sized> {
    fn orphaned_cycle(this: &Self) -> Option<HashMap<Link<T>, usize>>;
}

impl<T: ?Sized> DetectCycles<T> for Rc<T> {
    fn orphaned_cycle(this: &Self) -> Option<HashMap<Link<T>, usize>> {
        let cycle = cycle_refs(Link::forward(this.ptr));
        if cycle.is_empty() {
            return None;
        }
        let has_external_owners = cycle
            .iter()
            .any(|(item, cycle_owned_refs)| item.strong() > *cycle_owned_refs);
        if has_external_owners {
            None
        } else {
            Some(cycle)
        }
    }
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
        let links = node.inner().links.borrow();
        for (link, strong) in links.iter() {
            let entry = cycle_owned_refs.entry(link.as_forward()).or_insert(0);
            if let Kind::Forward = link.link_kind() {
                *entry += strong;
                discovered.push(*link);
            }
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
        .map(|(item, cycle_count)| (item.strong(), cycle_count))
        .collect::<Vec<_>>();
    trace!(
        "cactusref reachability test found (strong, cycle) counts: {:?}",
        counts
    );
}
