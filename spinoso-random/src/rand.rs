use core::fmt;
use rand_::Rng;

use crate::{ArgumentError, Random};

/// A range constraint for generating random numbers.
///
/// This enum is an input to the [`rand`] function. See its documentation for
/// more details.
// TODO: Add range variants
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
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

impl fmt::Display for Max {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(max) => write!(f, "{}", max),
            Self::Integer(max) => write!(f, "{}", max),
            Self::None => f.write_str("Infinity"),
        }
    }
}

/// A generated random number.
///
/// This enum is returned by the [`rand`] function. See its documentation for
/// more details.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub enum Rand {
    /// A random float.
    ///
    /// A random float is returned from [`rand`] when given [`Max::Float`] or
    /// [`Max::None`] max constraint.
    Float(f64),
    /// A random integer.
    ///
    /// A random integer is returned from [`rand`] when given [`Max::Integer`]
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
/// This function does not yet support range constaints. When support is added,
/// when `max` is a `Range`, `rand` will return a random number where
/// range.member?(number) == true.
///
/// # Examples
///
/// Generate floats from `(0.0..1.0)`:
///
/// ```
/// # use spinoso_random::{rand, ArgumentError, InitializeError, Max, Rand, Random};
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
/// # use spinoso_random::{rand, ArgumentError, InitializeError, Max, Rand, Random};
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
#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
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
            let number = rng.gen_range(0.0, max);
            Ok(Rand::Float(number))
        }
        Max::Integer(max) if max < 1 => {
            let err = ArgumentError::with_rand_max(constraint);
            Err(err)
        }
        Max::Integer(max) => {
            let number = rng.gen_range(0, max);
            Ok(Rand::Integer(number))
        }
        Max::None => {
            let number = rng.next_real();
            Ok(Rand::Float(number))
        }
    }
}
