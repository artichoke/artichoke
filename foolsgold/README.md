# foolsgold

foolsgold crate is an integration test for the [nemesis](/nemesis) and
[mruby](/mruby) crates. foolsgold uses nemesis to serve a
[Ruby Rack application](/foolsgold/ruby/config.ru) that is partially implemented
in Rust and backed by an embedded mruby interpreter.

Nemesis is similar to [Thin](https://github.com/macournoyer/thin) in a
traditional Ruby web stack.

## Components

foolsgold consists of three parts: a
[Rust implementation of the foolsgold Ruby gem](/foolsgold/src/foolsgold.rs), a
[collection of static assets](/foolsgold/src/assets.rs), and a
[launcher for a Nemesis server](/foolsgold/src/main.rs).

The nemesis launcher pulls all of these components together. A simplified
version of the launcher is:

```rust
Builder::default()
    .add_mount(
        Mount::from_rackup("foolsgold", foolsgold::RACKUP, "/fools-gold")
            .with_init(Box::new(|interp| {
                foolsgold::init(interp)?;
                interp.eval("require 'foolsgold'")?;
                Ok(())
            })),
    )
    .add_static_assets(Assets::all()?)
    .serve()
```

## Execution Modes

Unlike Thin, Nemesis allows:

- Mounting multiple [Rack](https://github.com/rack/rack) apps at unique route
  prefixes.
- Declaring interpreter isolation level per Rack app.

foolsgold mounts two copies of the Rack app:

- [`http://127.0.0.1:8000/fools-gold/shared-nothing`](http://127.0.0.1:8000/fools-gold/shared-nothing)
  creates a single use interpreter for each request.
- [`http://127.0.0.1:8000/fools-gold/prefork`](http://127.0.0.1:8000/fools-gold/prefork)
  creates an interpreter for each HTTP worker. Nemesis recycles the interpreter
  after it has served 150 requests.

## App

The
[`FoolsGold::Middleware::Request`](/foolsgold/ruby/lib/foolsgold/middleware/request.rb)
Rack adapter injects a
[`RequestContext`](/foolsgold/ruby/lib/foolsgold/stats.rb) instance into the
Rack environment. The `RequestContext` is implemented in pure Rust. The context
closes over a static `AtomicI64` request counter, making global state available
across mruby interpreter instances. The counter is readable and writable via
these APIs:

```ruby
# retrieve
env[FoolsGold::CONTEXT].metrics.total_requests.get
# increment
env[FoolsGold::CONTEXT].metrics.total_requests.inc
```

## Performance

The pieces of FoolsGold that are implemented in Rust vs Ruby are not performance
critical—a mutex and a call to /dev/urandom—so Rust is not expected to provide
any speedup for the Ruby sources.

When the webservers are configured similarly:

- The shared nothing Rust implementation is about twice as slow as the Ruby
  implementation.
- The prefork Rust implementation performs similarly to the Ruby implementation
  with better tail latency likely due to Tokio being better than EventMachine.

The application code run in the rackup file is a close to trivial HTML template.
With more complex application logic, the startup costs of the shared nothing
interpreter will not dominate response time as much.

### Setup

Thin uses Ruby [2.6.3](https://github.com/ruby/ruby/tree/v2_6_3). Nemesis uses
mruby 2.0.1 @ [c078758](https://github.com/mruby/mruby/commit/c078758).

Benches compare Thin with threads vs. Nemesis with Rocket and per worker
interpreters.

Benchmarks compare the webservers with concurrency levels of 1, 16, and 64
requests.

### Run

To run benches, first launch foolsgold with Nemesis:

```sh
cargo run --release --bin foolsgold
```

Also launch foolsgold with Thin:

```sh
./scripts/foolsgold-thin.sh
```

Then run the bench for a single concurrency level with:

```sh
RUST_BACKTRACE=1 cargo bench --benches -p foolsgold -- "concurrency(64)"
```

**NOTE**: It is important to filter tests by concurrency level because each test
may consume all of the ephmeral ports pointing to the server backend bind
address. On macOS, you should wait at least 15 seconds between bench runs to
clear all sockets in `TIME_WAIT` state.

### Results

#### 1 Concurrent Request

```txt
nemesis prefork with concurrency(1)
                        time:   [118.20 ms 119.14 ms 120.07 ms]
                        change: [-1.1571% +0.3644% +1.9143%] (p = 0.65 > 0.05)
                        No change in performance detected.

thin threaded with concurrency(1)
                        time:   [63.304 ms 64.245 ms 65.359 ms]
                        change: [-4.3226% -1.0679% +2.0184%] (p = 0.56 > 0.05)
                        No change in performance detected.
```

#### 16 Concurrent Requests

```txt
nemesis prefork with concurrency(16)
                        time:   [77.774 ms 81.302 ms 84.243 ms]
                        change: [-4.8474% -0.0760% +5.5309%] (p = 0.97 > 0.05)
                        No change in performance detected.

thin threaded with concurrency(16)
                        time:   [83.902 ms 86.927 ms 89.537 ms]
                        change: [-2.4466% +4.1192% +10.787%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
```

#### 64 Concurrent Requests

```txt
nemesis prefork with concurrency(64)
                        time:   [78.439 ms 81.457 ms 83.652 ms]
                        change: [-3.7211% -0.1831% +3.9425%] (p = 0.93 > 0.05)
                        No change in performance detected.

thin threaded with concurrency(64)
                        time:   [77.481 ms 82.224 ms 86.576 ms]
                        change: [-11.971% -3.8320% +4.7384%] (p = 0.43 > 0.05)
                        No change in performance detected.
```
