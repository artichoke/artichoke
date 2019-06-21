# hubris

hubris crate is a [Sinatra](http://sinatrarb.com/) echo server that is an
integration test for the [nemesis](/nemesis), [mruby](/mruby), and
[mruby-gems](/mruby-gems) crates. hubris uses nemesis to serve a
[Sinatra Rack application](src/config.ru) that is backed by an embedded mruby
interpreter.

Nemesis is similar to [Thin](https://github.com/macournoyer/thin) in a
traditional Ruby web stack.

## Output

```console
$ curl -i 'http://localhost:8000/ferrocarril?demo=hubris&mruby' && echo
HTTP/1.1 200 OK
Content-Length: 163
Content-Type: application/json
Server: Rocket
Date: Fri, 21 Jun 2019 14:32:34 GMT

{
  "method": "GET",
  "path": "/ferrocarril",
  "args": "demo=hubris&mruby",
  "body": "",
  "headers": {

  },
  "uuid": "fb70d164-031c-4616-aeb4-41a31295fa5b"
}
```

## Components

hubris consists of three parts: a
[patched versions of Sinatra and its dependencies](/mruby-gems/src/rubygems/),
implementations of Ruby [core](/mruby/src/extn/core) and
[standard library](/mruby/src/extn/stdlib) for mruby written in Rust and Ruby,
and a [launcher for a Nemesis server](src/main.rs).

The nemesis launcher pulls all of these components together by initializing all
of the gems that the Sinatra-based [rackup file](src/config.ru) depends on:

```rust
pub fn spawn() -> Result<(), Error> {
    Builder::default()
        .add_mount(
            Mount::from_rackup("echo", APP, "/")
                .with_init(Box::new(include_str!("config.ru")))
                .with_shared_interpreter(Some(150)),
        )
        .serve()
}

fn interp_init(interp: &Mrb) -> Result<(), MrbError> {
    rubygems::mustermann::init(&interp)?;
    rubygems::rack::init(&interp)?;
    rubygems::rack_protection::init(&interp)?;
    rubygems::sinatra::init(&interp)?;
    rubygems::tilt::init(&interp)?;
    Ok(())
}
```
