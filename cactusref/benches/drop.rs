#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate criterion;

use cactusref::{Adoptable, Rc};
use criterion::black_box;
use criterion::Criterion;
use std::cell::RefCell;

struct Node<T> {
    _data: T,
    links: Vec<Rc<RefCell<Self>>>,
}

fn circular_graph(count: usize) -> Vec<Rc<RefCell<Node<usize>>>> {
    let mut nodes = vec![];
    for i in 0..count {
        nodes.push(Rc::new(RefCell::new(Node {
            _data: i,
            links: vec![],
        })));
    }
    for i in 0..count - 1 {
        let link = Rc::clone(&nodes[i + 1]);
        Rc::adopt(&nodes[i], &link);
        nodes[i].borrow_mut().links.push(link);
    }
    let link = Rc::clone(&nodes[0]);
    Rc::adopt(&nodes[count - 1], &link);
    nodes[count - 1].borrow_mut().links.push(link);
    nodes
}

fn fully_connected_graph(count: usize) -> Vec<Rc<RefCell<Node<usize>>>> {
    let mut nodes = vec![];
    for i in 0..count {
        nodes.push(Rc::new(RefCell::new(Node {
            _data: i,
            links: vec![],
        })));
    }
    for left in &nodes {
        for right in &nodes {
            let link = Rc::clone(right);
            Rc::adopt(left, &link);
            left.borrow_mut().links.push(link);
        }
    }
    nodes
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "drop a circular graph",
        |b, &&size| b.iter_with_large_setup(|| circular_graph(black_box(size)), drop),
        &[10, 20, 30, 40, 50, 100],
    );
    c.bench_function_over_inputs(
        "drop a fully connected graph",
        |b, &&size| b.iter_with_large_setup(|| fully_connected_graph(black_box(size)), drop),
        &[10, 20, 30, 40, 50, 100],
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
