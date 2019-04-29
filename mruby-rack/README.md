# mruby-rack

Crate mruby-rack implements the [Rack](https://rack.github.io/) gem for the
mruby crate. It can be used to embed a Ruby Rack application in Rust.

# FoolsGold

FoolsGold is an included sample binary that implements a Rust web application
that executes a [Ruby Rack application](/mruby-rack/ruby/config.ru) on an
embedded [mruby](/mruby) interpreter.

FoolsGold serves the Rack application with [Rocket](https://rocket.rs/), which
acts similarly to [Unicorn](https://bogomips.org/unicorn/) or
[Thin](https://github.com/macournoyer/thin) in a traditional Ruby web stack.

With mruby and Rocket, FoolsGold implements a shared nothing, asynchronous Ruby
webserver. Each request creates an isolated instance of an mruby interpreter and
initializes the interpreter by requiring the rackup file and Rack sources which
are implemented in Ruby and the FoolsGold sources which are implemented in Rust.
Rocket is based on [hyper](https://hyper.rs/) and [Tokio](https://tokio.rs/), so
it is asynchronous and incredibly fast.

The
[`FoolsGold::Adapter::Memory`](/mruby-rack/ruby/lib/foolsgold/adapter/memory.rb)
Rack adapter injects a
[`RequestContext`](/mruby-rack/ruby/lib/foolsgold/stats.rb) instance into the
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

Rust exposes a `RequestStats` class to the mruby interpreter that grants the
Ruby code access to UUID V4 generation for request IDs and closes over a static
`AtomicI64` request counter that is shared among all mruby interpreters, all
requests, and all threads.

The Rack application is defined with a `config.ru` which is executed in the
context of a `Rack::Builder`.

The Rack response tuple is translated to a Rocket HTTP response. This setup is
similar to a threaded Unicorn with the difference that the execution model is
shared nothing.

## Performance

### Methodology

- Launch web server with default configuration
- Warm server with `ab -c 16 -n 10000 -l http://localhost:$PORT/fools-gold`
- Benchmark with `ab -c 16 -n 5000 -l http://localhost:$PORT/fools-gold`

### Rust

| Web Server |                         Ruby Interpreter                         |                FoolsGold Implementation                 |
| :--------: | :--------------------------------------------------------------: | :-----------------------------------------------------: |
|   Rocket   | mruby @ [c078758](https://github.com/mruby/mruby/commit/c078758) | [Rust](/mruby-rack/src/bin/foolsgold/ruby/foolsgold.rs) |

To bench, run the following to launch the application server:

```sh
# with logging
FOOLSGOLD_LOG=foolsgold=debug,rocket=info cargo run --release
```

FoolsGold will be running on <http://localhost:8000/fools-gold>.

#### Results

| p99 Latency | Requests Per Second |
| :---------: | :-----------------: |
|    36ms     |       820 RPS       |

### Ruby

| Web Server |                        Ruby Interpreter                         |        FoolsGold Implementation        |
| :--------: | :-------------------------------------------------------------: | :------------------------------------: |
|    Thin    | YARV Ruby @ [2.6.0p0](https://github.com/ruby/ruby/tree/v2_6_0) | [Ruby](/mruby-rack/ruby/lib/foolsgold) |

To bench, run the following to launch the application server:

```sh
cd ruby
bundle install
# with logging
bundle exec rackup -o localhost -p 9000 -I./lib bench/config.ru
```

FoolsGold will be running on <http://localhost:9000/fools-gold>.

#### Results

| p99 Latency | Requests Per Second |
| :---------: | :-----------------: |
|    192ms    |       140 RPS       |
