use crate::convert::{
    float_to_int, implicitly_convert_to_int, implicitly_convert_to_string, maybe_to_int, MaybeToInt,
};
use crate::extn::core::kernel;
use crate::extn::core::kernel::require::RelativePath;
use crate::extn::prelude::*;

// FIXME: handle float and integer arguments.
//
// ```
// [3.1.2] > Integer 999
// => 999
// [3.1.2] > Integer 999.9
// => 999
// [3.1.2] >
// ^C
// [3.1.2] > Integer -999.9
// => -999
// [3.1.2] > Integer Float::NAN
// (irb):43:in `Integer': NaN (FloatDomainError)
//         from (irb):43:in `<main>'
//         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
//         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
//         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
// [3.1.2] > Integer Float::INFINITY
// (irb):44:in `Integer': Infinity (FloatDomainError)
//         from (irb):44:in `<main>'
//         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
//         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
//         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
// ```
pub fn integer(interp: &mut Artichoke, mut subject: Value, base: Option<Value>) -> Result<Value, Error> {
    // Flatten explicit `nil` argument with missing argument
    let base = base.and_then(|base| interp.convert(base));

    let result = if subject.is_nil() {
        if let Some(base) = base {
            if matches!(maybe_to_int(interp, base)?, MaybeToInt::Int(..)) {
                return Err(ArgumentError::with_message("base specified for non string value").into());
            }
        }
        return Err(TypeError::with_message("can't convert nil into Integer").into());
    } else if let Ok(subject) = subject.try_convert_into_mut::<&[u8]>(interp) {
        let base = if let Some(base) = base {
            if let MaybeToInt::Int(int) = maybe_to_int(interp, base)? {
                Some(int)
            } else {
                None
            }
        } else {
            None
        };
        scolapasta_int_parse::parse(subject, base)?
    } else if let Ok(float) = subject.try_convert_into::<f64>(interp) {
        if let Some(base) = base {
            if matches!(maybe_to_int(interp, base)?, MaybeToInt::Int(..)) {
                return Err(ArgumentError::with_message("base specified for non string value").into());
            }
        }
        float_to_int(float)?
    } else if let Some(base) = base {
        let base = if let MaybeToInt::Int(int) = maybe_to_int(interp, base)? {
            Some(int)
        } else {
            None
        };
        if let Some(base) = base {
            if let Ok(s) = subject.try_convert_into_mut::<&[u8]>(interp) {
                scolapasta_int_parse::parse(s, Some(base))?
            } else if subject.respond_to(interp, "to_str")? {
                // SAFETY: the extracted byte slice is used and discarded before
                // the interpreter is accessed again.
                let s = unsafe { implicitly_convert_to_string(interp, &mut subject)? };
                scolapasta_int_parse::parse(s, Some(base))?
            } else {
                return Err(ArgumentError::with_message("base specified for non string value").into());
            }
        } else {
            implicitly_convert_to_int(interp, subject).map_err(|_| {
                let message = format!("can't convert {} into Integer", interp.class_name_for_value(subject));
                TypeError::from(message)
            })?
        }
    } else {
        match maybe_to_int(interp, subject) {
            Ok(MaybeToInt::Int(int)) => int,
            Ok(MaybeToInt::Err(err)) => return Err(err.into()),
            Ok(MaybeToInt::UncriticalReturn(result)) => {
                let class = interp.class_name_for_value(subject).to_owned();
                let result = interp.class_name_for_value(result);
                let message = format!("can't convert {class} to Integer ({class}#to_i gives {result})");
                return Err(TypeError::from(message).into());
            }
            Ok(MaybeToInt::NotApplicable) | Err(_) => {
                let message = format!("can't convert {} into Integer", interp.class_name_for_value(subject));
                return Err(TypeError::from(message).into());
            }
        }
    };

    Ok(interp.convert(result))
}

pub fn load(interp: &mut Artichoke, path: Value) -> Result<Value, Error> {
    let success = kernel::require::load(interp, path)?;
    Ok(interp.convert(bool::from(success)))
}

pub fn print<T>(interp: &mut Artichoke, args: T) -> Result<Value, Error>
where
    T: IntoIterator<Item = Value>,
{
    for value in args {
        let display = value.to_s(interp);
        interp.print(&display)?;
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
            interp.puts(&display)?;
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
        interp.puts(&display)?;
        if args.peek().is_none() {
            return Ok(first);
        }
        let mut result = vec![first];
        for value in args {
            let display = value.inspect(interp);
            interp.puts(&display)?;
            result.push(value);
        }
        interp.try_convert_mut(result)
    } else {
        Ok(Value::nil())
    }
}

pub fn require(interp: &mut Artichoke, path: Value) -> Result<Value, Error> {
    let success = kernel::require::require(interp, path)?;
    Ok(interp.convert(bool::from(success)))
}

pub fn require_relative(interp: &mut Artichoke, path: Value) -> Result<Value, Error> {
    let relative_base = RelativePath::try_from_interp(interp)?;
    let success = kernel::require::require_relative(interp, path, relative_base)?;
    Ok(interp.convert(bool::from(success)))
}
