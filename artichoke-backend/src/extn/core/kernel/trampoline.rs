use crate::extn::core::kernel;
use crate::extn::core::kernel::require::RelativePath;
use crate::extn::prelude::*;

pub fn integer(interp: &mut Artichoke, mut arg: Value, base: Option<Value>) -> Result<Value, Error> {
    let base = base.and_then(|base| interp.convert(base));
    // Safety:
    //
    // Extract the `Copy` radix integer first since implicit conversions can
    // trigger garbage collections.
    let base = interp.try_convert_mut(base)?;
    let arg = interp.try_convert_mut(&mut arg)?;
    let integer = kernel::integer::method(arg, base)?;
    Ok(interp.convert(integer))
}

pub fn load(interp: &mut Artichoke, path: Value) -> Result<Value, Error> {
    let success = kernel::require::load(interp, path)?;
    Ok(interp.convert(success))
}

pub fn print<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    for value in args {
        let display = value.to_s(interp);
        interp.print(display)?;
    }
    Ok(Value::nil())
}

pub fn puts<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    fn puts_foreach(interp: &mut Artichoke, value: &Value) -> Result<(), Error> {
        // TODO(GH-310): Use `Value::implicitly_convert_to_array` when
        // implemented so `Value`s that respond to `to_ary` are converted
        // and iterated over.
        if let Ok(array) = value.try_convert_into_mut::<Vec<_>>(interp) {
            for value in &array {
                puts_foreach(interp, value)?;
            }
        } else {
            let display = value.to_s(interp);
            interp.puts(display)?;
        }
        Ok(())
    }

    let mut args = args.into_iter();
    if let Some(first) = args.next() {
        puts_foreach(interp, &first)?;
        for value in args {
            puts_foreach(interp, &value)?;
        }
    } else {
        interp.print(b"\n")?;
    }
    Ok(Value::nil())
}

pub fn p<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    let mut args = args.into_iter().peekable();
    if let Some(first) = args.next() {
        let display = first.inspect(interp);
        interp.puts(display)?;
        if args.peek().is_none() {
            return Ok(first);
        }
        let mut result = vec![first];
        for value in args {
            let display = value.inspect(interp);
            interp.puts(display)?;
            result.push(value);
        }
        interp.try_convert_mut(result)
    } else {
        Ok(Value::nil())
    }
}

pub fn require(interp: &mut Artichoke, path: Value) -> Result<Value, Error> {
    let success = kernel::require::require(interp, path)?;
    Ok(interp.convert(success))
}

pub fn require_relative(interp: &mut Artichoke, path: Value) -> Result<Value, Error> {
    let relative_base = RelativePath::try_from_interp(interp)?;
    let success = kernel::require::require_relative(interp, path, relative_base)?;
    Ok(interp.convert(success))
}
