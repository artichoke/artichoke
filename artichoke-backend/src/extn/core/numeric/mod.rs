use crate::extn::core::integer::Integer;
use crate::extn::prelude::*;

pub(in crate::extn) mod mruby;

#[derive(Debug, Clone, Copy)]
pub struct Numeric;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Outcome {
    Float(f64),
    Integer(i64),
    // TODO: Complex? Rational?
}

impl ConvertMut<Outcome, Value> for Artichoke {
    fn convert_mut(&mut self, from: Outcome) -> Value {
        match from {
            Outcome::Float(num) => self.convert_mut(num),
            Outcome::Integer(num) => self.convert(num),
        }
    }
}

const MAX_COERCE_DEPTH: u8 = 15;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Coercion {
    Float(f64, f64),
    Integer(i64, i64),
    // TODO: Complex? Rational?
}

/// If `y` is the same type as `x`, returns an array `[y, x]`. Otherwise,
/// returns an array with both `y` and `x` represented as `Float` objects.
///
/// This coercion mechanism is used by Ruby to handle mixed-type numeric
/// operations: it is intended to find a compatible common type between the two
/// operands of the operator.
///
/// See [`Numeric#coerce`][numeric].
///
/// # Coercion enum
///
/// Artichoke represents the `[y, x]` tuple Array as the [`Coercion`] enum, which
/// orders its values `Coercion::Integer(x, y)`.
///
/// # Examples
///
/// ```
/// # use artichoke_backend::prelude::*;
/// # use artichoke_backend::extn::core::numeric::{self, Coercion};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let mut interp = artichoke_backend::interpreter()?;
/// let x = interp.convert(1_i64);
/// let y = interp.convert_mut(2.5_f64);
/// assert_eq!(Coercion::Float(1.0, 2.5), numeric::coerce(&mut interp, x, y)?);
/// let x = interp.convert_mut(1.2_f64);
/// let y = interp.convert(3_i64);
/// assert_eq!(Coercion::Float(1.2, 3.0), numeric::coerce(&mut interp, x, y)?);
/// let x = interp.convert(1_i64);
/// let y = interp.convert(2_i64);
/// assert_eq!(Coercion::Integer(1, 2), numeric::coerce(&mut interp, x, y)?);
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// [numeric]: https://ruby-doc.org/core-3.1.2/Numeric.html#method-i-coerce
pub fn coerce(interp: &mut Artichoke, x: Value, y: Value) -> Result<Coercion, Error> {
    fn do_coerce(interp: &mut Artichoke, x: Value, y: Value, depth: u8) -> Result<Coercion, Error> {
        if depth > MAX_COERCE_DEPTH {
            return Err(SystemStackError::with_message("stack level too deep").into());
        }
        match (x.ruby_type(), y.ruby_type()) {
            (Ruby::Float, Ruby::Float) => Ok(Coercion::Float(
                x.try_convert_into(interp)?,
                y.try_convert_into(interp)?,
            )),
            (Ruby::Float, Ruby::Fixnum) => {
                let y = y.try_convert_into::<Integer>(interp)?;
                Ok(Coercion::Float(x.try_convert_into(interp)?, y.as_f64()))
            }
            (Ruby::Fixnum, Ruby::Float) => {
                let x = x.try_convert_into::<Integer>(interp)?;
                Ok(Coercion::Float(x.as_f64(), y.try_convert_into(interp)?))
            }
            (Ruby::Fixnum, Ruby::Fixnum) => Ok(Coercion::Integer(
                x.try_convert_into(interp)?,
                y.try_convert_into(interp)?,
            )),
            _ => {
                let class_of_numeric = interp
                    .class_of::<Numeric>()?
                    .ok_or_else(|| NotDefinedError::class("Numeric"))?;
                let is_a_numeric = y.funcall(interp, "is_a?", &[class_of_numeric], None)?;
                let is_a_numeric = interp.try_convert(is_a_numeric);
                if let Ok(true) = is_a_numeric {
                    if y.respond_to(interp, "coerce")? {
                        let coerced = y.funcall(interp, "coerce", &[x], None)?;
                        let coerced: Vec<Value> = interp
                            .try_convert_mut(coerced)
                            .map_err(|_| TypeError::with_message("coerce must return [x, y]"))?;
                        let mut coerced = coerced.into_iter();
                        let y = coerced
                            .next()
                            .ok_or_else(|| TypeError::with_message("coerce must return [x, y]"))?;
                        let x = coerced
                            .next()
                            .ok_or_else(|| TypeError::with_message("coerce must return [x, y]"))?;
                        if coerced.next().is_some() {
                            Err(TypeError::with_message("coerce must return [x, y]").into())
                        } else {
                            do_coerce(interp, x, y, depth + 1)
                        }
                    } else {
                        let mut message = String::from("can't convert ");
                        message.push_str(interp.inspect_type_name_for_value(y));
                        message.push_str(" into Float");
                        Err(TypeError::from(message).into())
                    }
                } else {
                    let mut message = String::from(interp.inspect_type_name_for_value(y));
                    message.push_str(" can't be coerced into Float");
                    Err(TypeError::from(message).into())
                }
            }
        }
    }
    do_coerce(interp, x, y, 0)
}
