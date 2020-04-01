use crate::extn::core::kernel;
use crate::extn::core::kernel::require::RelativePath;
use crate::extn::prelude::*;
use crate::state::output::Output;

pub fn integer(
    interp: &mut Artichoke,
    arg: Value,
    base: Option<Value>,
) -> Result<Value, Exception> {
    kernel::integer::method(interp, arg, base)
}

pub fn load(interp: &mut Artichoke, path: Value) -> Result<Value, Exception> {
    kernel::require::load(interp, path)
}

pub fn print(interp: &mut Artichoke, args: Vec<Value>) -> Result<Value, Exception> {
    for value in args {
        let display = value.to_s();
        let mut borrow = interp.0.borrow_mut();
        borrow.output.print(display);
    }
    Ok(interp.convert(None::<Value>))
}

pub fn puts(interp: &mut Artichoke, args: Vec<Value>) -> Result<Value, Exception> {
    fn puts_foreach(interp: &mut Artichoke, value: &Value) {
        // TODO(GH-310): Use `Value::implicitly_convert_to_array` when
        // implemented so `Value`s that respond to `to_ary` are converted
        // and iterated over.
        if let Ok(array) = value.clone().try_into::<Vec<Value>>() {
            for value in &array {
                puts_foreach(interp, value);
            }
        } else {
            let display = value.to_s();
            let mut borrow = interp.0.borrow_mut();
            borrow.output.puts(display);
        }
    }

    if args.is_empty() {
        let mut borrow = interp.0.borrow_mut();
        borrow.output.print(b"\n");
    } else {
        for value in &args {
            puts_foreach(interp, value);
        }
    }
    Ok(interp.convert(None::<Value>))
}

pub fn require(interp: &mut Artichoke, path: Value) -> Result<Value, Exception> {
    kernel::require::require(interp, path, None)
}

pub fn require_relative(interp: &mut Artichoke, path: Value) -> Result<Value, Exception> {
    let relative_base = RelativePath::try_from_interp(interp)?;
    kernel::require::require(interp, path, Some(relative_base))
}
