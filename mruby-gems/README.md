# mruby-gems

Crate mruby-gems is a rubygems repository for the `mruby` crate.

Gems are declared with exact version constraints in [`Gemfile`](Gemfile) and
vendored locally with [bundler](https://bundler.io/). Pure Ruby gems are loaded
directly into the an mruby `Mrb` interpreter using
`MrbLoadSources::def_rb_source_file`. C extensions are reimplemented in Rust.

Crate mruby-gems exposes a `Gem` top-level trait which when implemented allows a
Ruby and/or Rust-backed gem to be installed into an interpreter.

## Implementing a Gem

### Vendor Gem Dependencies

The first step in implementing a new gem is adding it to the
[`Gemfile`](Gemfile). Peg a gem to a _specific_ version with an `= x.y.z`
version constraint. For example, to vendor Rack, add

```ruby
gem 'rack', '= 2.0.7'
```

Then run `bundle install` and check in the resulting [lock file](Gemfile.lock)
and vendored gem sources.

### Public Interface

Gem modules should expose one public function:

```rust
pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    unimplemented!()
}
```

### Pure Ruby

mruby-gems crate includes the `rust-embed` crate which can embed a directory of
source files in the compiled Rust binary. Use this crate to embed gem sources.

An example of a pure Ruby gem is [`Rack`](src/rubygems/rack.rs):

```rust
use mruby::interpreter::Mrb;
use mruby::load::MrbLoadSources;
use mruby::MrbError;
use std::borrow::Cow;
use std::convert::AsRef;

use crate::Gem;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    Rack::init(interp)
}

#[derive(RustEmbed)]
#[folder = "mruby-gems/vendor/ruby/2.6.0/gems/rack-2.0.7/lib"]
struct Rack;

impl Rack {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))
    }
}

impl Gem for Rack {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        Ok(())
    }
}
```

### Ruby and Rust

TODO
