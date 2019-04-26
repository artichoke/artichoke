# mruby-rack

Crate mruby-rack implements a multi-threaded, shared-nothing Unicorn-like
application server using Rocket and an embedded mruby interpeter via the
`mruby` crate.

# FoolsGold

FoolsGold is a Rust webserver that executes a Ruby Rack application on an
embedded mruby interpreter.

The Rust webserver is Rocket, which is based on Hyper and Tokio. It is an
incredibly fast, async webserver with statically typed route functions and
typed, thread-safe shared state.

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

### mruby-rack

`mruby-rack` is a Rust and embedded mruby implementation of the Rack application
that uses Rocket, mruby, and a Rust implementation of the `RequestStats` class.
To bench, run the following to launch the application server:

```sh
FOOLS_GOLD_LOG=mruby=debug,rocket=info cargo run --release
```

In a separate shell, run the following to generate load:

```sh
# Rocket launches with 16 threads by default
ab -c 16 -n 5000 -l http://localhost:8000/fools-gold
```

#### Results

After warming the application server with 10,000 requests, the Rust
implementation has a p99 latency of 36ms and serves 820 requests per second.

### Pure Ruby

`mruby-rack` also includes a pure Ruby implementation of the Rack application
that uses Thin, YARV Ruby, and a pure Ruby implementation of the `RequestStats`
class. To bench, run the following to launch the application server:

```sh
cd ruby
bundle install
# set Servers to 16 to match Rocket
bundle exec rackup -o 127.0.0.1 -p 9000 -I. -O Servers=16 bench/config.ru
```

In a separate shell, run the following to generate load:

```sh
# Thin launches with 16 workers
ab -c 16 -n 5000 -l http://127.0.0.1:9000/fools-gold
```

#### Results

After warming the application server with 10,000 requests, the pure Ruby
implementation has a p99 latency of 192ms and serves 140 requests per second.
