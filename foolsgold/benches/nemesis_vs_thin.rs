// Performs timing tests on Nemesis and Thin with 1, 16, and 64 concurrent
// requests.
//
// To better compare Nemesis and Thin they must be started both externally so
// that the load::benchmark code here only executes HTTP client timing code.
//
// foolsgold with Nemesis must be started with:
//
// ```sh
// cargo run --release --bin foolsgold
// ```
//
// foolsgold with Thin must be started with:
//
// ```sh
// ./scripts/foolsgold-thin.sh
// ```
//
// Execute with `cargo load::bench`.
//
// Running all tests in this suite will result in a hang due to ephemeral port
// exhaustion. Run tests for a single concurrency with:
//
// ```sh
// cargo load::bench -p foolsgold -- 'concurrency(16)'

#[macro_use]
extern crate criterion;
extern crate futures;
extern crate hyper;
extern crate tokio;

use criterion::{black_box, Criterion};
use std::time::Duration;

mod load;

fn c1(c: &mut Criterion) {
    c.bench_function("nemesis prefork with concurrency(1)", |b| {
        b.iter(|| load::bench_nemesis_prefork(black_box(1)))
    });
    c.bench_function("thin threaded with concurrency(1)", |b| {
        b.iter(|| load::bench_thin_threaded(black_box(1)))
    });
}

fn c16(c: &mut Criterion) {
    c.bench_function("nemesis prefork with concurrency(16)", |b| {
        b.iter(|| load::bench_nemesis_prefork(black_box(16)))
    });
    c.bench_function("thin threaded with concurrency(16)", |b| {
        b.iter(|| load::bench_thin_threaded(black_box(16)))
    });
}

fn c64(c: &mut Criterion) {
    c.bench_function("nemesis prefork with concurrency(64)", |b| {
        b.iter(|| load::bench_nemesis_prefork(black_box(64)))
    });
    c.bench_function("thin threaded with concurrency(64)", |b| {
        b.iter(|| load::bench_thin_threaded(black_box(64)))
    });
}

// These parameters are carefully chosen to avoid issuing more than 2**16
// requests, which is the number of available ephemeral ports.
criterion_group!(
    name = bench_c1;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(100))
        .sample_size(100);
    targets = c1
);
criterion_group!(
    name = bench_c16;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(100))
        .sample_size(40);
    targets = c16
);
criterion_group!(
    name = bench_c64;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(100))
        .sample_size(20);
    targets = c64
);
criterion_main!(bench_c1, bench_c16, bench_c64);
