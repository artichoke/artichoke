# mruby

Crate mruby provides a safe interface over the raw bindings in
[mruby-sys](/mruby-sys).

## Features

### Embed Rust Objects in `mrb_value`

`mrb_value`s can own a pointer to a Rust object via an `Rc<RefCell<_>>`. The
interpreter supports extracting Rust values from an mruby value, which can be
used to implement Ruby functions by delegating to Rust code.

### Converters between Ruby and Rust Types

The [convert module](src/convert) provides implementations for `TryFromMrb`
which can convert between Ruby `mrb_value`s and native Rust types using an `Mrb`
interpreter.

The converters support converting `nil`able Ruby types to `Option<_>` and
converting Array to `Vec`.

The converters are `unsafe`.

### Virtual Filesystem and `Kernel#require`

The mruby [State](src/state.rs) embeds an
[in-memory virtual Unix filesystem](/mruby-vfs). Both pure Ruby source files and
Rust-implemented `MrbFile`s are stored in the VFS and exposed to Ruby code in
the interpreter via a Rust implementation of `Kernel#require`.

`MrbFile::require` methods are stored as custom metadata on File nodes in the
VFS.

## API Examples

### Evaling code

```rust
use mruby::convert::TryFromMrb;
use mruby::eval::MrbEval;
use mruby::interpreter::Interpreter;

let interp = Interpreter::create().expect("mrb init");
let result = interp.eval("10 * 10").expect("eval");
let result = unsafe { i64::try_from_mrb(&interp, result) }.expect("convert");
assert_eq!(result, 100);
```

### Defining Ruby Sources

```rust
use mruby::convert::TryFromMrb;
use mruby::eval::MrbEval;
use mruby::interpreter::Interpreter;
use mruby::load::MrbLoadSources;

let mut interp = Interpreter::create().expect("mrb init");
let code = "def source_file; __FILE__; end";
interp.def_rb_source_file("source.rb", code).expect("def file");
interp.eval("require 'source'").expect("eval");
let result = interp.eval("source_file").expect("eval");
let result = unsafe { String::try_from_mrb(&interp, result) }.expect("convert");
assert_eq!(&result, "/src/lib/source.rb");
```

### Defining Rust-Backed Ruby Types

```rust
use mruby::convert::TryFromMrb;
use mruby::def::{ClassLike, Define};
use mruby::eval::MrbEval;
use mruby::file::MrbFile;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::interpreter_or_raise;
use mruby::load::MrbLoadSources;
use mruby::sys;
use mruby::unwrap_value_or_raise;
use mruby::value::Value;
use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::mem;
use std::rc::Rc;

struct Container { inner: i64 }

impl MrbFile for Container {
  fn require(interp: Mrb) {
        extern "C" fn free(_mrb: *mut sys::mrb_state, data: *mut c_void) {
            unsafe {
                let _ = mem::transmute::<*mut c_void, Rc<RefCell<Container>>>(data);
            }
        }

        extern "C" fn initialize(
            mrb: *mut sys::mrb_state,
            mut slf: sys::mrb_value,
        ) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);
                let api = interp.borrow();
                let int = mem::uninitialized::<sys::mrb_int>();
                let argspec = CString::new(sys::specifiers::INTEGER).expect("argspec");
                sys::mrb_get_args(mrb, argspec.as_ptr(), &int);
                let cont = Container { inner: int };
                let data = Rc::new(RefCell::new(cont));
                let ptr = mem::transmute::<Rc<RefCell<Container>>, *mut c_void>(data);
                let spec = api.class_spec::<Container>();
                sys::mrb_sys_data_init(&mut slf, ptr, spec.data_type());
                slf
            }
        }

        extern "C" fn value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);
                let api = interp.borrow();
                let spec = api.class_spec::<Container>();
                let ptr = sys::mrb_data_get_ptr(mrb, slf, spec.data_type());
                let data = mem::transmute::<*mut c_void, Rc<RefCell<Container>>>(ptr);
                let clone = Rc::clone(&data);
                let cont = clone.borrow();
                let value = unwrap_value_or_raise!(interp, Value::try_from_mrb(&interp, cont.inner));
                mem::forget(data);
                value
            }
        }

        {
            let mut api = interp.borrow_mut();
            api.def_class::<Self>("Container", None, Some(free));
            let spec = api.class_spec_mut::<Self>();
            spec.add_method("initialize", initialize, sys::mrb_args_req(1));
            spec.add_method("value", value, sys::mrb_args_none());
            spec.mrb_value_is_rust_backed(true);
        }
        let api = interp.borrow();
        let spec = api.class_spec::<Self>();
        spec.define(&interp).expect("class install");
    }
}

let mut interp = Interpreter::create().expect("mrb init");
interp.def_file_for_type::<_, Container>("source.rb").expect("def file");
interp.eval("require 'source'").expect("eval");
let result = interp.eval("Container.new(15).value * 24").expect("eval");
let result = unsafe { i64::try_from_mrb(&interp, result) }.expect("convert");
assert_eq!(result, 360);
```

### Garbage Collection

```rust
use mruby::convert::TryFromMrb;
use mruby::eval::MrbEval;
use mruby::gc::GarbageCollection;
use mruby::interpreter::Interpreter;
use mruby::load::MrbLoadSources;

let interp = Interpreter::create().expect("mrb init");
interp.disable_gc();
let code = "def spray_heap; 1024.times { '1234567890' }; nil; end";
interp.eval(code).expect("eval");
let live_objects = interp.live_object_count();
interp.eval("spray_heap").expect("eval");
assert_eq!(interp.live_object_count(), live_objects + 1024 + 3);
interp.enable_gc();
interp.full_gc();
assert_eq!(interp.live_object_count(), live_objects - 1);
```
