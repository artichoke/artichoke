use futures::{stream, Future, Stream};
use hyper::{Client, Uri};
use std::iter;

const NEMESIS_PORT: u16 = 8000;
const THIN_PORT: u16 = 9000;

pub fn bench_nemesis_prefork(concurrency: u16) {
    bench_requests(concurrency, concurrency, NEMESIS_PORT);
}

pub fn bench_thin_threaded(concurrency: u16) {
    bench_requests(concurrency, concurrency, THIN_PORT);
}

fn bench_requests(amount: u16, concurrency: u16, port: u16) {
    let url = format!("http://127.0.0.1:{}/fools-gold/prefork", port)
        .parse::<Uri>()
        .unwrap();
    let urls = iter::repeat(url.clone()).take(amount.into());

    let work = stream::iter_ok(urls)
        .map(|url| Client::new().get(url))
        .buffer_unordered(concurrency.into())
        .and_then(move |res| {
            if !res.status().is_success() {
                panic!("got failed response: {}", res.status());
            }
            res.into_body()
                .concat2()
                .map_err(|e| panic!("Error collecting body: {}", e))
        })
        .for_each(|_body| Ok(()))
        .map_err(|e| panic!("Error making request: {}", e));

    tokio::run(work);
}
