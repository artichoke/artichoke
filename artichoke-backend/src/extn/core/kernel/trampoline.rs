use crate::convert::{check_string_type, check_to_int, to_i};
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
pub fn integer(interp: &mut Artichoke, mut val: Value, base: Option<Value>) -> Result<Value, Error> {
    let base = if let Some(base) = base {
        let converted_base = check_to_int(interp, base)?;
        if converted_base.is_nil() {
            None
        } else {
            Some(converted_base.try_convert_into::<i64>(interp)?)
        }
    } else {
        None
    };
    // The below routine is a port of `rb_convert_to_integer` from MRI 3.1.2.
    //
    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3109-L3155

    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3114-L3126
    if base.is_some() {
        let tmp = check_string_type(interp, val)?;
        if tmp.is_nil() {
            // TODO: handle exception kwarg and return nil here if it is false.
            return Err(ArgumentError::with_message("base specified for non string value").into());
        }
        val = tmp;
    }

    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3127-L3132
    if let Ok(f) = val.try_convert_into::<f64>(interp) {
        // TODO: handle exception kwarg and return `nil` if it is false and f is not finite.
        // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3129

        // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3131
        // https://github.com/ruby/ruby/blob/v3_1_2/bignum.c#L5230-L5235
        if f.is_infinite() {
            return Err(
                FloatDomainError::with_message(if f.is_sign_negative() { "-Infinity" } else { "Infinity" }).into(),
            );
        }
        // https://github.com/ruby/ruby/blob/v3_1_2/bignum.c#L5233-L5235
        if f.is_nan() {
            return Err(FloatDomainError::with_message("NaN").into());
        }

        // TODO: this should check to see if `f` is in range for `i64`. MRI calls
        // this check "is fixable" / `FIXABLE`. If `f` is not fixable, it should
        // be converted to a bignum.
        #[allow(clippy::cast_possible_truncation)]
        return Ok(interp.convert(f as i64));
    }

    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3133-L3135
    if let Ruby::Fixnum = val.ruby_type() {
        return Ok(val);
    }

    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3136-L3138
    if let Ok(subject) = unsafe { spinoso_string::String::unbox_from_value(&mut val, interp) } {
        // https://github.com/ruby/ruby/blob/v3_1_2/bignum.c#L4257-L4277
        //
        // TODO: handle exception kwarg and return nil here if it is false and
        // `parse` returns an error.
        //
        // TODO: handle bignum.
        let i = scolapasta_int_parse::parse(subject.as_slice(), base)?;
        return Ok(interp.convert(i));
    }

    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3139-L3142
    if val.is_nil() {
        // TODO: handle exception kwarg and return nil here if it is false.
        return Err(TypeError::with_message("can't convert nil into Integer").into());
    }

    match check_to_int(interp, val) {
        Ok(tmp) if tmp.ruby_type() == Ruby::Fixnum => return Ok(tmp),
        _ => {}
    }

    // https://github.com/ruby/ruby/blob/v3_1_2/object.c#L3148-L3154
    //
    // TODO: handle exception kwarg and return nil here if it is false.
    to_i(interp, val)
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
