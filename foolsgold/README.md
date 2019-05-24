# FoolsGold

foolsgold crate is an integration test for the crates in this workspace that
implements a Rust web application that executes a
[Ruby Rack application](/foolsgold/ruby/config.ru) on an embedded
[mruby](/mruby) interpreter.

FoolsGold serves the Rack application with [Rocket](https://rocket.rs/) and
[nemesis](/nemesis), which acts similarly to
[Thin](https://github.com/macournoyer/thin) in a traditional Ruby web stack.
Rocket is based on [hyper](https://hyper.rs/) and [Tokio](https://tokio.rs/), so
it is asynchronous and incredibly fast.

## Execution Modes

FoolsGold serves the Rack application with mruby operating in both shared
nothing and prefork execution modes _in the same server_ as separate routes.
Both exeuction modes share the same global Rust state.

### Shared Nothing

With mruby and Rocket, FoolsGold implements a
[shared nothing](/foolsgold/src/execmodel/shared_nothing.rs) Ruby webserver.
Each request to
[`http://127.0.0.1:8000/fools-gold`](http://127.0.0.1:8000/fools-gold) creates
an isolated instance of an mruby interpreter. The route
[initializes the interpreter](/foolsgold/src/execmodel/mod.rs) by
[generating a Rack app from the rackup file](/nemesis/src/handler.rs), requiring
the [Rack sources](/mruby-gems/src/rubygems/rack.rs) which are implemented in
Ruby and requiring the FoolsGold sources which are implemented in
[Ruby](/foolsgold/ruby/lib/foolsgold) and [Rust](/foolsgold/src/foolsgold.rs).

### Prefork

With mruby and Rocket, FoolsGold implements a
["prefork"](/foolsgold/src/execmodel/prefork.rs) Ruby webserver. Each request to
[`http://127.0.0.1:8000/fools-gold/prefork`](http://127.0.0.1:8000/fools-gold/prefork)
uses a thread local mruby interpreter that is created on the first request a
thread serves. The route
[initializes the interpreter](/foolsgold/src/execmodel/mod.rs) once by
[generating a Rack app from the rackup file](/nemesis/src/handler.rs), requiring
the [Rack sources](/mruby-gems/src/rubygems/rack.rs) which are implemented in
Ruby and requiring the FoolsGold sources which are implemented in
[Ruby](/foolsgold/ruby/lib/foolsgold) and [Rust](/foolsgold/src/foolsgold.rs).

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

_Note_: These tests may not run since the introduction of [nemesis](/nemesis).

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

### Methodology

For each implementation and endpoint:

- Launch web server with 16 threads
- Warm server with `ab -c 16 -n 10000 -l $ENDPOINT`
- Wait for TCP connections in TIME_WAIT state to finalize
- Benchmark with `ab -c 16 -n 10000 -l $ENDPOINT`
- Wait for TCP connections in TIME_WAIT state to finalize
- Benchmark with `ab -c 64 -n 10000 -l $ENDPOINT`

Tests use a late-2013 MacBook Pro with a 2.6GHz Quad Core i7 processor and 16GB
of RAM.

### Rust: Shared Nothing

| Web Server |                         Ruby Interpreter                         |          FoolsGold Implementation           |
| :--------: | :--------------------------------------------------------------: | :-----------------------------------------: |
|   Rocket   | mruby @ [c078758](https://github.com/mruby/mruby/commit/c078758) | [Rust](/foolsgold/src/sources/foolsgold.rs) |

To bench, run the following to launch the application server:

```sh
cargo build --release
# without logging
cargo run --release foolsgold
```

FoolsGold will be running on <http://127.0.0.1:8000/fools-gold>.

#### Results

**Concurrency = 16**

| p99 Latency | p90 Latency | Requests Per Second |
| :---------: | :---------: | :-----------------: |
|    64ms     |    12ms     |      2025 RPS       |

**Concurrency = 64**

| p99 Latency | p90 Latency | Requests Per Second |
| :---------: | :---------: | :-----------------: |
|    79ms     |    46ms     |      2030 RPS       |

### Rust: Prefork

| Web Server |                         Ruby Interpreter                         |          FoolsGold Implementation           |
| :--------: | :--------------------------------------------------------------: | :-----------------------------------------: |
|   Rocket   | mruby @ [c078758](https://github.com/mruby/mruby/commit/c078758) | [Rust](/foolsgold/src/sources/foolsgold.rs) |

To bench, run the following to launch the application server:

```sh
cargo build --release
# without logging
cargo run --release foolsgold
```

FoolsGold will be running on <http://127.0.0.1:8000/fools-gold/prefork>.

#### Results

**Concurrency = 16**

| p99 Latency | p90 Latency | Requests Per Second |
| :---------: | :---------: | :-----------------: |
|     6ms     |     3ms     |      5220 RPS       |

**Concurrency = 64**

| p99 Latency | p90 Latency | Requests Per Second |
| :---------: | :---------: | :-----------------: |
|    18ms     |    14ms     |      4950 RPS       |

### Ruby

| Web Server |                        Ruby Interpreter                         |       FoolsGold Implementation        |
| :--------: | :-------------------------------------------------------------: | :-----------------------------------: |
|    Thin    | YARV Ruby @ [2.6.0p0](https://github.com/ruby/ruby/tree/v2_6_0) | [Ruby](/foolsgold/ruby/lib/foolsgold) |

To bench, run the following to launch the application server:

```sh
cd ruby
bundle install
# without logging
bundle exec thin -a 127.0.0.1 -p 9000 --threaded --threadpool-size 16 -R bench/config.ru start
```

FoolsGold will be running on <http://127.0.0.1:9000/fools-gold>.

#### Results

**Concurrency = 16**

| p99 Latency | p90 Latency | Requests Per Second |
| :---------: | :---------: | :-----------------: |
|     8ms     |     5ms     |      3970 RPS       |

**Concurrency = 64**

| p99 Latency | p90 Latency | Requests Per Second |
| :---------: | :---------: | :-----------------: |
|    41ms     |    18ms     |      4236 RPS       |
