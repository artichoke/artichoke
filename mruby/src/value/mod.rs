use mruby_sys::*;

mod bool;
mod types;
mod u16;
mod u32;
mod u64;
mod u8;

// We can't impl `fmt::Debug` because `mrb_sys_value_debug_str` requires a
// `mrb_state` interpreter, which we can't store on the `Value` because we
// construct it from Rust native types.
struct Value(mrb_value);

impl Value {
    pub fn ruby_type(&self) -> types::Ruby {
        types::Ruby::from(self.0)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct ConvertError<F, T> {
    from: F,
    to: T,
}
