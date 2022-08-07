use crate::convert::implicitly_convert_to_string;
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
    // SAFETY: Extract the `Copy` radix integer first since implicit conversions
    // can trigger garbage collections.
    let base = if let Some(base) = base {
        Some(base.try_convert_into::<i64>(interp)?)
    } else {
        None
    };

    // Implicit conversions are only performed if a non-nil radix argument is
    // given:
    //
    // ```
    // [3.1.2] > class A; def to_str; "1234"; end; end
    // => :to_str
    // [3.1.2] > Integer(A.new)
    // (irb):20:in `Integer': can't convert A into Integer (TypeError)
    //         from (irb):20:in `<main>'
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // [3.1.2] > Integer(A.new, nil)
    // (irb):2:in `Integer': can't convert A into Integer (TypeError)
    //         from (irb):2:in `<main>'
    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
    // [3.1.2] > Integer(A.new, 10)
    // => 1234
    // ```
    let integer = if base.is_none() {
        let subject = subject.try_convert_into_mut::<&[u8]>(interp)?;
        scolapasta_int_parse::parse(subject, base)?
    } else {
        // SAFETY: no interpreter access occurs between extracting this slice
        // and the slice going out of scope, so the buffer backing it will not
        // be invalidated.
        let subject = unsafe { implicitly_convert_to_string(interp, &mut subject)? };
        // This line needs to appear in both branches because the lifetimes of
        // `subject` differ.
        scolapasta_int_parse::parse(subject, base)?
    };
    Ok(interp.convert(integer))
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
