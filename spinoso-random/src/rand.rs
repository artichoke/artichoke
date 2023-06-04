use core::fmt;

use rand::Rng;

use crate::{ArgumentError, Random};

/// A range constraint for generating random numbers.
///
/// This enum is an input to the [`rand()`] function. See its documentation for
/// more details.
// TODO: Add range variants
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(docsrs, doc(cfg(feature = "rand-method")))]
pub enum Max {
    /// A maximum float bound.
    ///
    /// This bound is exclusive.
    Float(f64),
    /// A maximum integer bound.
    ///
    /// This bound is exclusive.
    Integer(i64),
    /// The default bound when no bound is supplied.
    ///
    /// This bound corresponds to [`Max::Float(1.0)`](Self::Float).
    None,
}

impl Default for Max {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for Max {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(max) => write!(f, "{max}"),
            Self::Integer(max) => write!(f, "{max}"),
            Self::None => f.write_str("Infinity"),
        }
    }
}

/// A generated random number.
///
/// This enum is returned by the [`rand()`] function. See its documentation for
/// more details.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(docsrs, doc(cfg(feature = "rand-method")))]
pub enum Rand {
    /// A random float.
    ///
    /// A random float is returned from [`rand()`] when given [`Max::Float`] or
    /// [`Max::None`] max constraint.
    Float(f64),
    /// A random integer.
    ///
    /// A random integer is returned from [`rand()`] when given [`Max::Integer`]
    /// max constraint.
    Integer(i64),
}

/// Generate random numbers bounded from below by 0 and above by the given max.
///
/// When `max` is an `i64`, `rand` returns a random integer greater than or
/// equal to zero and less than `max`.
///
/// When `max` is an `f64`, `rand` returns a random floating point number
/// between 0.0 and max, including 0.0 and excluding `max`.
///
/// # Implementation notes
///
/// This function does not yet support range constraints. When support is added,
/// when `max` is a `Range`, `rand` will return a random number where
/// `range.member?(number) == true`.
///
/// # Examples
///
/// Generate floats from `(0.0..1.0)`:
///
/// ```
/// use spinoso_random::{rand, ArgumentError, InitializeError, Max, Rand, Random};
///
/// # #[derive(Debug)]
/// # enum ExampleError { Argument(ArgumentError), Initialize(InitializeError) }
/// # impl From<ArgumentError> for ExampleError { fn from(err: ArgumentError) -> Self { Self::Argument(err) } }
/// # impl From<InitializeError> for ExampleError { fn from(err: InitializeError) -> Self { Self::Initialize(err) } }
/// # fn example() -> Result<(), ExampleError> {
/// let mut random = Random::new()?;
/// let max = Max::None;
/// let rand = rand(&mut random, max)?;
/// assert!(matches!(rand, Rand::Float(x) if x < 1.0));
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// Generate random integers:
///
/// ```
/// use spinoso_random::{rand, ArgumentError, InitializeError, Max, Rand, Random};
///
/// # #[derive(Debug)]
/// # enum ExampleError { Argument(ArgumentError), Initialize(InitializeError) }
/// # impl From<ArgumentError> for ExampleError { fn from(err: ArgumentError) -> Self { Self::Argument(err) } }
/// # impl From<InitializeError> for ExampleError { fn from(err: InitializeError) -> Self { Self::Initialize(err) } }
/// # fn example() -> Result<(), ExampleError> {
/// let mut random = Random::new()?;
/// let max = Max::Integer(10);
/// let rand = rand(&mut random, max)?;
/// assert!(matches!(rand, Rand::Integer(x) if x < 10));
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// # Errors
///
/// When `max` is a negative integer or zero, `rand` returns an
/// [`ArgumentError`].
///
/// When `max` is a negative `f64`, `rand` returns an [`ArgumentError`].
///
/// When `max` is a non-finite `f64`, `rand` returns a domain error
/// [`ArgumentError`].
#[cfg(feature = "rand-method")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand-method")))]
pub fn rand(rng: &mut Random, max: Max) -> Result<Rand, ArgumentError> {
    let constraint = max;
    match constraint {
        Max::Float(max) if !max.is_finite() => {
            // NOTE: MRI returns `Errno::EDOM` exception class.
            Err(ArgumentError::domain_error())
        }
        Max::Float(max) if max < 0.0 => {
            let err = ArgumentError::with_rand_max(constraint);
            Err(err)
        }
        Max::Float(max) if max == 0.0 => {
            let number = rng.next_real();
            Ok(Rand::Float(number))
        }
        Max::Float(max) => {
            let number = rng.gen_range(0.0..max);
            Ok(Rand::Float(number))
        }
        Max::Integer(max) if max < 1 => {
            let err = ArgumentError::with_rand_max(constraint);
            Err(err)
        }
        Max::Integer(max) => {
            let number = rng.gen_range(0..max);
            Ok(Rand::Integer(number))
        }
        Max::None => {
            let number = rng.next_real();
            Ok(Rand::Float(number))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{rand, Max, Rand};
    use crate::Random;

    #[test]
    fn random_number_domain_error() {
        let mut random = Random::with_seed(33);
        assert!(matches!(rand(&mut random, Max::Float(f64::NAN)), Err(err) if err.is_domain_error()));
        assert!(matches!(rand(&mut random, Max::Float(f64::INFINITY)), Err(err) if err.is_domain_error()));
        assert!(matches!(rand(&mut random, Max::Float(f64::NEG_INFINITY)), Err(err) if err.is_domain_error()));
    }

    #[test]
    fn random_number_in_float_out_float() {
        let mut random = Random::with_seed(33);
        assert!(matches!(rand(&mut random, Max::None), Ok(Rand::Float(num)) if num < 1.0));
        assert!(matches!(
            rand(&mut random, Max::Float(0.5)),
            Ok(Rand::Float(num)) if num < 0.5
        ));
        assert!(matches!(
            rand(&mut random, Max::Float(1.0)),
            Ok(Rand::Float(num)) if num < 1.0
        ));
        assert!(matches!(
            rand(&mut random, Max::Float(9000.63)),
            Ok(Rand::Float(num)) if num < 9000.63
        ));
        assert!(matches!(
            rand(&mut random, Max::Float(0.0)),
            Ok(Rand::Float(num)) if num < 1.0
        ));
        assert!(matches!(
            rand(&mut random, Max::Float(-0.0)),
            Ok(Rand::Float(num)) if num < 1.0
        ));
    }

    #[test]
    fn random_number_in_neg_float_out_err() {
        let mut random = Random::with_seed(33);
        assert!(matches!(
            rand(&mut random, Max::Float(-1.0)), Err(err) if err.message() == "invalid argument"
        ));
    }

    #[test]
    fn random_number_in_neg_integer_out_err() {
        let mut random = Random::with_seed(33);
        assert!(matches!(rand(&mut random, Max::Integer(-1)), Err(err) if err.message() == "invalid argument"));
    }

    #[test]
    fn random_number_in_zero_integer_out_err() {
        let mut random = Random::with_seed(33);
        assert!(matches!(rand(&mut random, Max::Integer(0)), Err(err) if err.message() == "invalid argument"));
    }

    #[test]
    fn random_number_in_pos_integer_out_integer() {
        let mut random = Random::with_seed(33);
        assert!(matches!(rand(&mut random, Max::Integer(1)), Ok(Rand::Integer(0))));
        assert!(matches!(
            rand(&mut random, Max::Integer(9000)),
            Ok(Rand::Integer(ref num)) if (0..9000).contains(num)
        ));
        assert!(matches!(
            rand(&mut random, Max::Integer(i64::MAX)),
            Ok(Rand::Integer(num)) if num >= 0
        ));
    }
}
