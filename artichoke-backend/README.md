# artichoke-backend

artichoke-backend crate provides a Ruby interpreter. It currently is implemented
with [`mruby-sys`](/mruby-sys).

## Execute Ruby Code

artichoke-backend crate exposes several mechanisms for executing Ruby code on
the interpreter.

### Evaling Source Code

artichoke-backend crate exposes eval on the `State` with the `Eval` trait. Side
effects from eval are persisted across invocations.

```rust
use artichoke_backend::eval::Eval;

let interp = artichoke_backend::interpreter().unwrap();
let result = interp.eval("10 * 10").unwrap();
let result = result.try_into::<i64>();
assert_eq!(result, Ok(100));
```

### Calling Ruby Functions from Rust

The `ValueLike` trait exposes a _funcall interface_ which can call Ruby
functions on a `Value` using a `String` function name and a `Vec<Value>` of
arguments. funcall takes a type parameter bound by `TryConvert` and converts the
result of the function call to a Rust type (which may be `Value` or another
"native" type).

artichoke-backend limits functions to a maximum of 16 arguments.

## Virtual Filesystem and `Kernel#require`

The artichoke-backend `State` embeds an
[in-memory virtual Unix filesystem](/artichoke-vfs). The VFS stores Ruby sources
that are either pure Ruby, implemented with a Rust [`File`](file::File), or
both.

artichoke-backend crate implements `Kernel#require` and
`Kernel#require_relative` which loads sources from the VFS. For Ruby sources,
the source is loaded from the VFS as a `Vec<u8>` and evaled with
`Eval::eval_with_context`. For Rust sources, `File::require` methods are stored
as custom metadata on [`File`](/artichoke-vfs) nodes in the VFS.

```rust
use artichoke_backend::eval::Eval;
use artichoke_backend::load::LoadSources;

let mut interp = artichoke_backend::interpreter().unwrap();
let code = "
def source_location
  __FILE__
end
";
interp.def_rb_source_file("source.rb", code).unwrap();
interp.eval("require 'source'").unwrap();
let result = interp.eval("source_location").unwrap();
let result = result.try_into::<String>().unwrap();
assert_eq!(&result, "/src/lib/source.rb");
```

## Embed Rust Objects in `mrb_value`

The `mrb_value` struct is a data type that represents a Ruby object. The
concrete type of an `mrb_value` is specified by its type tag, an `mrb_vtype`
enum value.

One `mrb_vtype` is `MRB_TT_DATA`, which allows an `mrb_value` to store an owned
`c_void` pointer. artichoke-backend crate leverages this to store an owned copy
of an `Rc<RefCell<T>>` for any `T` that implements `RustBackedValue`.

`RustBackedValue` provides two methods for working with `MRB_TT_DATA`:

- `RustBackedValue::try_into_ruby` consumes `self` and returns a live
  `mrb_value` that wraps `T`.
- `RustBackedValue::try_from_ruby` extracts an `Rc<RefCell<T>>` from an
  `mrb_value` and manages the strong count of the `Rc` smart pointer to ensure
  that the `mrb_value` continues to point to valid memory.

These `mrb_value`s with type tag `MRB_TT_DATA` can be used to implement Ruby
`Class`es and `Module`s with Rust structs. An example of this is the
[`Regexp`](src/extn/core/regexp) class which wraps an Oniguruma regex provided
by the [`onig`] crate.

```rust
#[macro_use]
extern crate artichoke_backend;

use artichoke_backend::convert::{Convert, RustBackedValue, TryConvert};
use artichoke_backend::def::{rust_data_free, ClassLike, Define};
use artichoke_backend::eval::Eval;
use artichoke_backend::file::File;
use artichoke_backend::load::LoadSources;
use artichoke_backend::sys;
use artichoke_backend::value::Value;
use artichoke_backend::{Artichoke, ArtichokeError};
use std::io::Write;
use std::mem;

struct Container { inner: i64 }

impl Container {
    unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, mut slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let api = interp.borrow();
        let int = mem::uninitialized::<sys::mrb_int>();
        let mut argspec = vec![];
        argspec.write_all(format!("{}\0", sys::specifiers::INTEGER).as_bytes()).unwrap();
        sys::mrb_get_args(mrb, argspec.as_ptr() as *const i8, &int);
        let cont = Self { inner: int };
        cont
            .try_into_ruby(&interp, Some(slf))
            .unwrap_or_else(|_| Value::convert(&interp, None::<Value>))
            .inner()
    }

    unsafe extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        if let Ok(cont) = Self::try_from_ruby(&interp, &Value::new(&interp, slf)) {
            let borrow = cont.borrow();
            Value::convert(&interp, borrow.inner).inner()
        } else {
            Value::convert(&interp, None::<Value>).inner()
        }
    }
}

impl RustBackedValue for Container {}

impl File for Container {
  fn require(interp: Artichoke) -> Result<(), ArtichokeError> {
        let spec = interp.borrow_mut().def_class::<Self>("Container", None, Some(rust_data_free::<Self>));
        spec.borrow_mut().add_method("initialize", Self::initialize, sys::mrb_args_req(1));
        spec.borrow_mut().add_method("value", Self::value, sys::mrb_args_none());
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow().define(&interp)?;
        Ok(())
    }
}

fn main() {
    let interp = artichoke_backend::interpreter().unwrap();
    interp.def_file_for_type::<_, Container>("container.rb").unwrap();
    interp.eval("require 'container'").unwrap();
    let result = interp.eval("Container.new(15).value * 24").unwrap();
    assert_eq!(result.try_into::<i64>(), Ok(360));
}
```

## Converters Between Ruby and Rust Types

The [`convert` module](src/convert) provides implementations for conversions
between `mrb_value` Ruby types and native Rust types like `i64` and
`HashMap<String, Option<Vec<u8>>>` using an [`Artichoke`](src/lib.rs)
interpreter.

There are two converter traits:

- `Convert` provides infallible conversions that return `Self`. Converting from
  a Rust native type to a Ruby `mrb_value` is usually an infallible conversion.
- `TryConvert` provides fallible conversions that return `Result<Self, Error>`.
  Converting from a Ruby `mrb_value` to a Rust native type is always an fallible
  conversion because an `mrb_value` may be any type tag.

Supported conversions:

- Ruby _primitive types_ to Rust types. Primitive Ruby types are `TrueClass`,
  `FalseClass`, `String` (both UTF-8 and binary), `Fixnum` (`i64`), `Float`
  (`f64`).
- Rust types to Ruby types. Supported Rust types are `bool`, `Vec<u8>`, `&[u8]`,
  integer types that losslessly convert to `i64` (`i64`, `i32`, `i16`, `i8`,
  `u32`, `u16`, `u8`), `f64`, `String`, `&str`.
- Ruby `nil`able types to Rust `Option<T>`.
- Rust `Option<T>` types to Ruby `nil` or an `mrb_value` converted from `T`.
- Ruby `Array` to Rust `Vec<T>` where `T` corresponds to a Ruby _primitive
  type_.
- Rust `Vec<T>` to Ruby `Array` where `T` corresponds to a Ruby _primitive
  type_.
- Ruby `Hash` to Rust `Vec<(Key, Value)>` or `HashMap<Key, Value>` where `Key`
  and `Value` correspond to Ruby _primitive types_.
- Rust `Vec<(Key, Value)>` or `HashMap<Key, Value>` to Ruby `Hash` where `Key`
  and `Value` correspond to Ruby _primitive types_.
- Identity conversion from `Value` to `Value`, which is useful when working with
  collection types.

The infallible converters are safe Rust functions. The fallibile converters are
`unsafe` Rust functions.

## License

artichoke-backend is licensed with the [MIT License](/LICENSE) (c) Ryan
Lopopolo.

CactusRef contains Ruby sources derived from Ruby @
[2.6.3](https://github.com/ruby/ruby/tree/v2_6_3) which is copyright Yukihiro
Matsumoto \<matz@netlab.jp\> under the
[2-clause BSDL License](https://github.com/ruby/ruby/blob/v2_6_3/COPYING).
