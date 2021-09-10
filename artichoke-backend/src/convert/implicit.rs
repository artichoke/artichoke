use artichoke_core::debug::Debug as _;
use artichoke_core::value::Value as _;
use spinoso_exception::TypeError;

use crate::error::Error;
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

/// Attempt to implicitly convert a [`Value`] to an integer.
///
/// Attempt to extract an [`i64`] from the given `Value` by trying the following
/// conversions:
///
/// - If the given value is a Ruby integer, return the inner integer.
/// - If the given value is `nil`, return a [`TypeError`].
/// - If the given value responds to the `:to_int` method, call `value.to_int`:
///   - If `value.to_int` raises an exception, propagate that exception.
///   - If `value.to_int` returns a Ruby integer, return the inner integer.
///   - If `value.to_int` returs any other type, return a [`TypeError`].
/// - If the given value does not respond to the `:to_int` method, return a
///   [`TypeError`].
///
/// Floats and other numeric types are not coerced to integer by this implicit
/// conversion.
///
/// # Examples
///
/// ```
/// # use artichoke_backend::prelude::*;
/// # use artichoke_backend::convert::implicitly_convert_to_int;
/// # use artichoke_backend::value::Value;
/// # fn example() -> Result<(), Error> {
/// let mut interp = artichoke_backend::interpreter()?;
/// // successful conversions
/// let integer = interp.convert(17);
/// let a = interp.eval(b"class A; def to_int; 3; end; end; A.new")?;
///
/// assert!(matches!(implicitly_convert_to_int(&mut interp, integer), Ok(17)));
/// assert!(matches!(implicitly_convert_to_int(&mut interp, a), Ok(3)));
///
/// // failed conversions
/// let nil = Value::nil();
/// let b = interp.eval(b"class B; end; B.new")?;
/// let c = interp.eval(b"class C; def to_int; nil; end; end; C.new")?;
/// let d = interp.eval(b"class D; def to_int; 'not an integer'; end; end; D.new")?;
/// let e = interp.eval(b"class E; def to_int; 3.2; end; end; E.new")?;
/// let f = interp.eval(b"class F; def to_int; raise ArgumentError, 'not an integer'; end; end; F.new")?;
///
/// assert!(implicitly_convert_to_int(&mut interp, nil).is_err());
/// assert!(implicitly_convert_to_int(&mut interp, b).is_err());
/// assert!(implicitly_convert_to_int(&mut interp, c).is_err());
/// assert!(implicitly_convert_to_int(&mut interp, d).is_err());
/// assert!(implicitly_convert_to_int(&mut interp, e).is_err());
/// assert!(implicitly_convert_to_int(&mut interp, f).is_err());
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// This function returns an error if:
///
/// - The given value is `nil`.
/// - The given value is not an integer and does not respond to `:to_int`.
/// - The given value is not an integer and returns a non-integer value from its
///   `:to_int` method.
/// - The given value is not an integer and raises an error in its `:to_int`
///   method.
pub fn implicitly_convert_to_int(interp: &mut Artichoke, value: Value) -> Result<i64, Error> {
    match value.try_convert_into::<Option<i64>>(interp) {
        // successful conversion: the given value is an integer.
        Ok(Some(num)) => return Ok(num),
        // `nil` does not implicitly convert to integer:
        //
        // ```console
        // [2.6.6] > a = [1, 2, 3, 4, 5]
        // => [1, 2, 3, 4, 5]
        // [2.6.6] > a[nil]
        // Traceback (most recent call last):
        //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         3: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         2: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         1: from (irb):2
        // TypeError (no implicit conversion from nil to integer)
        // ```
        Ok(None) => return Err(TypeError::with_message("no implicit conversion from nil to integer").into()),
        Err(_) => {}
    }
    if let Ok(true) = value.respond_to(interp, "to_int") {
        // Propagate exceptions raised in `#to_int`:
        //
        // ```console
        // [2.6.6] > a = [1, 2, 3, 4, 5]
        // => [1, 2, 3, 4, 5]
        // [2.6.6] > class A; def to_int; raise ArgumentError, 'a message'; end; end
        // => :to_int
        // [2.6.6] > a[A.new]
        // Traceback (most recent call last):
        //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         2: from (irb):3
        //         1: from (irb):2:in `to_int'
        // ArgumentError (a message)
        // ```
        let maybe = value.funcall(interp, "to_int", &[], None)?;
        if let Ok(num) = maybe.try_convert_into::<i64>(interp) {
            // successful conversion: `#to_int` returned an integer.
            Ok(num)
        } else {
            // Non integer types returned from `#to_int`, even other numerics,
            // result in a `TypeError`:
            //
            // ```console
            // [2.6.6] > a = [1, 2, 3, 4, 5]
            // => [1, 2, 3, 4, 5]
            // [2.6.6] > class A; def to_int; "another string"; end; end
            // => :to_int
            // [2.6.6] > a[A.new]
            // Traceback (most recent call last):
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         2: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         1: from (irb):3
            // TypeError (can't convert A to Integer (A#to_int gives String))
            // [2.6.6] > class B; def to_int; 3.2; end; end
            // => :to_int
            // [2.6.6] > a[B.new]
            // Traceback (most recent call last):
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         2: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         1: from (irb):5
            // TypeError (can't convert B to Integer (B#to_int gives Float))
            // [2.6.6] > class C; def to_int; nil; end; end
            // => :to_int
            // [2.6.6] > a[C.new]
            // Traceback (most recent call last):
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         2: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         1: from (irb):7
            // TypeError (can't convert C to Integer (C#to_int gives NilClass))
            // [2.6.6] > module X; class Y; class Z; def to_int; "oh no"; end; end; end; end
            // => :to_int
            // [2.6.6] > a[X::Y::Z.new]
            // Traceback (most recent call last):
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         2: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         1: from (irb):9
            // TypeError (can't convert X::Y::Z to Integer (X::Y::Z#to_int gives String))
            // ```
            let mut message = String::from("can't convert ");
            let name = interp.inspect_type_name_for_value(value);
            message.push_str(name);
            message.push_str(" to Integer (");
            message.push_str(name);
            message.push_str("#to_int gives ");
            message.push_str(interp.class_name_for_value(maybe));
            message.push(')');
            Err(TypeError::from(message).into())
        }
    } else {
        // The given value is not an integer and cannot be converted with
        // `#to_int`:
        //
        // ```console
        // [2.6.6] > a = [1, 2, 3, 4, 5]
        // => [1, 2, 3, 4, 5]
        // [2.6.6] > a[true]
        // Traceback (most recent call last):
        //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         2: from (irb):10
        //         1: from (irb):10:in `rescue in irb_binding'
        // TypeError (no implicit conversion of true into Integer)
        // [2.6.6] > class A; end
        // => nil
        // [2.6.6] > a[A.new]
        // Traceback (most recent call last):
        //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         3: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         2: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         1: from (irb):5
        // TypeError (no implicit conversion of A into Integer)
        // ```
        let mut message = String::from("no implicit conversion of ");
        message.push_str(interp.inspect_type_name_for_value(value));
        message.push_str(" into Integer");
        Err(TypeError::from(message).into())
    }
}

/// Attempt to implicitly convert a [`Value`] to a byte slice (Ruby `String`).
///
/// Attempt to extract a `&[u8]` from the given `Value` by trying the following
/// conversions:
///
/// - If the given value is a Ruby string, return the inner byte slice.
/// - If the given value is `nil`, return a [`TypeError`].
/// - If the given value responds to the `:to_str` method, call `value.to_str`:
///   - If `value.to_str` raises an exception, propagate that exception.
///   - If `value.to_str` returns a Ruby string, return the inner byte slice.
///   - If `value.to_str` returs any other type, return a [`TypeError`].
/// - If the given value does not respond to the `:to_str` method, return a
///   [`TypeError`].
///
/// [`Symbol`]s are not coerced to byte slice by this implicit conversion.
///
/// # Examples
///
/// ```
/// # use artichoke_backend::prelude::*;
/// # use artichoke_backend::convert::implicitly_convert_to_string;
/// # use artichoke_backend::value::Value;
/// # fn example() -> Result<(), Error> {
/// let mut interp = artichoke_backend::interpreter()?;
/// // successful conversions
/// let mut string = interp.try_convert_mut("artichoke")?;
/// let mut a = interp.eval(b"class A; def to_str; 'spinoso'; end; end; A.new")?;
///
/// # unsafe {
/// assert!(matches!(implicitly_convert_to_string(&mut interp, &mut string), Ok(s) if *s == b"artichoke"[..]));
/// assert!(matches!(implicitly_convert_to_string(&mut interp, &mut a), Ok(s) if *s == b"spinoso"[..]));
///
/// // failed conversions
/// let mut nil = Value::nil();
/// let mut b = interp.eval(b"class B; end; B.new")?;
/// let mut c = interp.eval(b"class C; def to_str; nil; end; end; C.new")?;
/// let mut d = interp.eval(b"class D; def to_str; 17; end; end; D.new")?;
/// let mut e = interp.eval(b"class E; def to_str; :artichoke; end; end; E.new")?;
/// let mut f = interp.eval(b"class F; def to_str; raise ArgumentError, 'not a string'; end; end; F.new")?;
///
/// assert!(implicitly_convert_to_string(&mut interp, &mut nil).is_err());
/// assert!(implicitly_convert_to_string(&mut interp, &mut b).is_err());
/// assert!(implicitly_convert_to_string(&mut interp, &mut c).is_err());
/// assert!(implicitly_convert_to_string(&mut interp, &mut d).is_err());
/// assert!(implicitly_convert_to_string(&mut interp, &mut e).is_err());
/// assert!(implicitly_convert_to_string(&mut interp, &mut f).is_err());
/// # }
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// This function returns an error if:
///
/// - The given value is `nil`.
/// - The given value is not a string and does not respond to `:to_str`.
/// - The given value is not a string and returns a non-string value from its
///   `:to_str` method.
/// - The given value is not a string and raises an error in its `:to_str`
///   method.
///
/// # Safety
///
/// Callers must ensure that `value` does not outlive the given interpreter.
///
/// If a garbage collection can possibly run between calling this function and
/// using the returned slice, callers should convert the slice to an owned byte
/// vec.
///
/// [`Symbol`]: crate::extn::core::symbol::Symbol
pub unsafe fn implicitly_convert_to_string<'a>(
    interp: &mut Artichoke,
    value: &'a mut Value,
) -> Result<&'a [u8], Error> {
    match value.try_convert_into_mut::<Option<&'a [u8]>>(interp) {
        // successful conversion: the given value is an string.
        Ok(Some(s)) => return Ok(s),
        // `nil` does not implicitly convert to string:
        //
        // ```console
        // [2.6.6] > ENV[nil]
        // Traceback (most recent call last):
        //         6: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         4: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         3: from (irb):7
        //         2: from (irb):7:in `rescue in irb_binding'
        //         1: from (irb):7:in `[]'
        // TypeError (no implicit conversion of nil into String)
        // ```
        Ok(None) => return Err(TypeError::with_message("no implicit conversion of nil into String").into()),
        Err(_) => {}
    }
    if let Ruby::Symbol = value.ruby_type() {
        return Err(TypeError::with_message("no implicit conversion of Symbol into String").into());
    }
    if let Ok(true) = value.respond_to(interp, "to_str") {
        // Propagate exceptions raised in `#to_str`:
        //
        // ```console
        // [2.6.6] >  class A; def to_str; raise ArgumentError, 'a message'; end; end
        // => :to_str
        // [2.6.6] > ENV[A.new]
        // Traceback (most recent call last):
        //         6: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         4: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         3: from (irb):10
        //         2: from (irb):10:in `[]'
        //         1: from (irb):9:in `to_str'
        // ArgumentError (a message)
        // ```
        let maybe = value.funcall(interp, "to_str", &[], None)?;
        if let Ok(s) = maybe.try_convert_into_mut::<&[u8]>(interp) {
            // successful conversion: `#to_str` returned a string.
            Ok(s)
        } else {
            // Non `String` types returned from `#to_str` result in a
            // `TypeError`:
            //
            // ```console
            // [2.6.6] > class A; def to_str; 17; end; end
            // => :to_str
            // [2.6.6] > ENV[A.new]
            // Traceback (most recent call last):
            //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         2: from (irb):5
            //         1: from (irb):5:in `[]'
            // TypeError (can't convert A to String (A#to_str gives Integer))
            // [2.6.6] > class B; def to_str; true; end; end
            // => :to_str
            // [2.6.6] > ENV[B.new]
            // Traceback (most recent call last):
            //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         2: from (irb):7
            //         1: from (irb):7:in `[]'
            // TypeError (can't convert B to String (B#to_str gives TrueClass))
            // [2.6.6] > class C; def to_str; nil; end; end
            // => :to_str
            // [2.6.6] > ENV[C.new]
            // Traceback (most recent call last):
            //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         2: from (irb):9
            //         1: from (irb):9:in `[]'
            // TypeError (can't convert C to String (C#to_str gives NilClass))
            // [2.6.6] > module X; class Y; class Z; def to_str; 92; end; end; end; end
            // => :to_str
            // [2.6.6] > ENV[X::Y::Z.new]
            // Traceback (most recent call last):
            //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
            //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
            //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
            //         2: from (irb):11
            //         1: from (irb):11:in `[]'
            // TypeError (can't convert X::Y::Z to String (X::Y::Z#to_str gives Integer))
            // ```
            let mut message = String::from("can't convert ");
            let name = interp.inspect_type_name_for_value(*value);
            message.push_str(name);
            message.push_str(" to String (");
            message.push_str(name);
            message.push_str("#to_str gives ");
            message.push_str(interp.class_name_for_value(maybe));
            message.push(')');
            Err(TypeError::from(message).into())
        }
    } else {
        // The given value is not a string and cannot be converted with
        // `#to_str`:
        //
        // ```console
        // [2.6.6] > ENV[true]
        // Traceback (most recent call last):
        //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         2: from (irb):1
        //         1: from (irb):1:in `[]'
        // TypeError (no implicit conversion of true into String)
        // [2.6.6] > class A; end
        // => nil
        // [2.6.6] > ENV[A.new]
        // Traceback (most recent call last):
        //         5: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `<main>'
        //         4: from /usr/local/var/rbenv/versions/2.6.6/bin/irb:23:in `load'
        //         3: from /usr/local/var/rbenv/versions/2.6.6/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
        //         2: from (irb):3
        //         1: from (irb):3:in `[]'
        // TypeError (no implicit conversion of A into String)
        // ```
        let mut message = String::from("no implicit conversion of ");
        message.push_str(interp.inspect_type_name_for_value(*value));
        message.push_str(" into String");
        Err(TypeError::from(message).into())
    }
}

/// Attempt to implicitly convert a [`Value`] to an optional byte slice (nilable
/// Ruby `String`).
///
/// Attempt to extract a `Option<&[u8]>` from the given `Value` by trying the
/// following conversions:
///
/// - If the given value is a Ruby string, return the inner byte slice.
/// - If the given value is `nil`, return [`None`].
/// - If the given value responds to the `:to_str` method, call `value.to_str`:
///   - If `value.to_str` raises an exception, propagate that exception.
///   - If `value.to_str` returns a Ruby string, return the inner byte slice.
///   - If `value.to_str` returs any other type, including `nil`, return a
///     [`TypeError`].
/// - If the given value does not respond to the `:to_str` method, return a
///   [`TypeError`].
///
/// [`Symbol`]s are not coerced to byte slice by this implicit conversion.
///
/// # Examples
///
/// ```
/// # use artichoke_backend::prelude::*;
/// # use artichoke_backend::convert::implicitly_convert_to_nilable_string;
/// # use artichoke_backend::value::Value;
/// # fn example() -> Result<(), Error> {
/// let mut interp = artichoke_backend::interpreter()?;
/// // successful conversions
/// let mut string = interp.try_convert_mut("artichoke")?;
/// let mut nil = Value::nil();
/// let mut a = interp.eval(b"class A; def to_str; 'spinoso'; end; end; A.new")?;
///
/// # unsafe {
/// assert!(matches!(implicitly_convert_to_nilable_string(&mut interp, &mut string), Ok(Some(s)) if *s == b"artichoke"[..]));
/// assert!(matches!(implicitly_convert_to_nilable_string(&mut interp, &mut nil), Ok(None)));
/// assert!(matches!(implicitly_convert_to_nilable_string(&mut interp, &mut a), Ok(Some(s)) if *s == b"spinoso"[..]));
///
/// // failed conversions
/// let mut b = interp.eval(b"class B; end; B.new")?;
/// let mut c = interp.eval(b"class C; def to_str; nil; end; end; C.new")?;
/// let mut d = interp.eval(b"class D; def to_str; 17; end; end; D.new")?;
/// let mut e = interp.eval(b"class E; def to_str; :artichoke; end; end; E.new")?;
/// let mut f = interp.eval(b"class F; def to_str; raise ArgumentError, 'not a string'; end; end; F.new")?;
/// let mut g = interp.eval(b"class G; def to_str; nil; end; end; G.new")?;
///
/// assert!(implicitly_convert_to_nilable_string(&mut interp, &mut b).is_err());
/// assert!(implicitly_convert_to_nilable_string(&mut interp, &mut c).is_err());
/// assert!(implicitly_convert_to_nilable_string(&mut interp, &mut d).is_err());
/// assert!(implicitly_convert_to_nilable_string(&mut interp, &mut e).is_err());
/// assert!(implicitly_convert_to_nilable_string(&mut interp, &mut f).is_err());
/// assert!(implicitly_convert_to_nilable_string(&mut interp, &mut g).is_err());
/// # }
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// This function returns an error if:
///
/// - The given value is `nil`.
/// - The given value is not a string and does not respond to `:to_str`.
/// - The given value is not a string and returns a non-string value from its
///   `:to_str` method.
/// - The given value is not a string and raises an error in its `:to_str`
///   method.
///
/// # Safety
///
/// Callers must ensure that `value` does not outlive the given interpreter.
///
/// [`Symbol`]: crate::extn::core::symbol::Symbol
pub unsafe fn implicitly_convert_to_nilable_string<'a>(
    interp: &mut Artichoke,
    value: &'a mut Value,
) -> Result<Option<&'a [u8]>, Error> {
    if value.is_nil() {
        Ok(None)
    } else {
        let string = implicitly_convert_to_string(interp, value)?;
        Ok(Some(string))
    }
}
