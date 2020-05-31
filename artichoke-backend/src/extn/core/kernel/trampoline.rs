use crate::extn::core::kernel;
use crate::extn::core::kernel::require::RelativePath;
use crate::extn::prelude::*;

pub fn integer(
    interp: &mut Artichoke,
    arg: Value,
    base: Option<Value>,
) -> Result<Value, Exception> {
    let base = base.and_then(|base| interp.convert(base));
    let arg = interp.try_convert_mut(&arg)?;
    let base = interp.try_convert_mut(base)?;
    let integer = kernel::integer::method(interp, arg, base)?;
    Ok(interp.convert(integer))
}

pub fn load(interp: &mut Artichoke, path: Value) -> Result<Value, Exception> {
    let success = kernel::require::load(interp, path)?;
    Ok(interp.convert(success))
}

pub fn print<T>(interp: &mut Artichoke, args: T) -> Result<Value, Exception>
where
    T: IntoIterator<Item = Value>,
{
    for value in args {
        let display = value.to_s(interp);
        interp.print(display)?;
    }
    Ok(Value::nil())
}

pub fn puts<T>(interp: &mut Artichoke, args: T) -> Result<Value, Exception>
where
    T: IntoIterator<Item = Value>,
{
    fn puts_foreach(interp: &mut Artichoke, value: &Value) -> Result<(), Exception> {
        // TODO(GH-310): Use `Value::implicitly_convert_to_array` when
        // implemented so `Value`s that respond to `to_ary` are converted
        // and iterated over.
        if let Ok(array) = value.try_into_mut::<Vec<_>>(interp) {
            for value in &array {
                puts_foreach(interp, value)?;
            }
        } else {
            let display = value.to_s(interp);
            interp.puts(display)?;
        }
        Ok(())
    }

    let mut args = args.into_iter().peekable();
    if args.peek().is_none() {
        interp.print(b"\n")?;
    } else {
        for value in args {
            puts_foreach(interp, &value)?;
        }
    }
    Ok(Value::nil())
}

pub fn p<T>(interp: &mut Artichoke, args: T) -> Result<Value, Exception>
where
    T: IntoIterator<Item = Value>,
{
    let args = args.into_iter().collect::<Vec<_>>();
    for value in &args {
        let display = value.inspect(interp);
        interp.puts(display)?;
    }

    match args {
        0 => Ok(Value::nil()),
        1 => Ok(interp.convert(args[0].to_owned())),
        _ => Ok(interp.convert_mut(args)),
    }
}

pub fn require(interp: &mut Artichoke, path: Value) -> Result<Value, Exception> {
    let success = kernel::require::require(interp, path, None)?;
    Ok(interp.convert(success))
}

pub fn require_relative(interp: &mut Artichoke, path: Value) -> Result<Value, Exception> {
    let relative_base = RelativePath::try_from_interp(interp)?;
    let success = kernel::require::require(interp, path, Some(relative_base))?;
    Ok(interp.convert(success))
}
