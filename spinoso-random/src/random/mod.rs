use core::fmt;
use core::mem::size_of;

use crate::{InitializeError, NewSeedError};

#[cfg(feature = "rand-traits")]
mod rand;
pub mod ruby;

use ruby::Mt;

const DEFAULT_SEED_CNT: usize = 4;
const DEFAULT_SEED_BYTES: usize = size_of::<u32>() * DEFAULT_SEED_CNT;

const DEFAULT_SEED: u32 = 5489_u32;

/// Random provides an interface to Ruby's pseudo-random number generator, or
/// PRNG.
///
/// The PRNG produces a deterministic sequence of bits which approximate true
/// randomness. The sequence may be represented by integers, floats, or binary
/// strings.
///
/// The generator may be initialized with either a system-generated or
/// user-supplied seed value.
///
/// PRNGs are currently implemented as a modified Mersenne Twister with a period
/// of 2**19937-1.
///
/// This RNG reproduces the same random bytes and floats as MRI. It may differ
/// when returning elements confined to a distribution.
///
/// # Examples
///
/// Create an RNG with a random seed:
///
/// ```
/// # use spinoso_random::{Error, Random};
/// # fn example() -> Result<(), Error> {
/// let mut random = Random::new()?;
/// let next = random.next_int32();
/// # Ok(())
/// # }
/// # example().unwrap();
/// ```
///
/// Create a RNG with a fixed seed:
///
/// ```
/// # use spinoso_random::Random;
/// let seed = 5489_u32;
/// let mut random = Random::with_seed(seed);
/// let rand = random.next_int32();
///
/// let seed = [627457_u32, 697550, 16438, 41926];
/// let mut random = Random::with_array_seed(seed);
/// let rand = random.next_int32();
/// ```
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Random {
    mt: Mt,
    seed: [u32; 4],
}

impl Default for Random {
    #[inline]
    fn default() -> Self {
        if let Ok(random) = Random::new() {
            random
        } else {
            Random::with_seed(DEFAULT_SEED)
        }
    }
}

impl fmt::Debug for Random {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Random {}")
    }
}

impl From<u32> for Random {
    #[inline]
    fn from(seed: u32) -> Self {
        Self::with_seed(seed)
    }
}

impl From<[u32; DEFAULT_SEED_CNT]> for Random {
    #[inline]
    fn from(seed: [u32; DEFAULT_SEED_CNT]) -> Self {
        Self::with_array_seed(seed)
    }
}

impl From<[u8; DEFAULT_SEED_BYTES]> for Random {
    #[inline]
    fn from(seed: [u8; DEFAULT_SEED_BYTES]) -> Self {
        Self::with_byte_array_seed(seed)
    }
}

impl Random {
    /// Create a new Mersenne Twister random number generator with a randomly
    /// generated seed.
    ///
    /// This method initializes the Mersenne Twister random number generator
    /// with a seed derived from a cryptographically secure source of
    /// randomness.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::{Error, Random};
    /// # fn example() -> Result<(), Error> {
    /// let mut random = Random::new()?;
    /// let next = random.next_int32();
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If the randomness feature provided by the platform is not present or
    /// failed to completely generate a seed, an error is returned. This error
    /// should be raised as a [Ruby `RuntimeError`].
    ///
    /// [Ruby `RuntimeError`]: https://ruby-doc.org/core-2.6.3/RuntimeError.html
    #[inline]
    pub fn new() -> Result<Self, InitializeError> {
        if let Ok(seed) = new_seed() {
            let mt = Mt::new_with_key(seed.iter().copied());
            Ok(Self { mt, seed })
        } else {
            Err(InitializeError::new())
        }
    }

    /// Create a new random number generator using the given seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Random;
    /// let seed = 33;
    /// let mut random = Random::with_seed(seed);
    /// let rand = random.next_int32();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_seed(seed: u32) -> Self {
        let mt = Mt::with_seed(seed);
        let seed = u128::from(seed).to_le_bytes();
        let seed = seed_to_key(seed);
        Self { mt, seed }
    }

    /// Create a new random number generator using the given seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Random;
    /// let seed = [1_u32, 2, 3, 4];
    /// let mut random = Random::with_array_seed(seed);
    /// let rand = random.next_int32();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_array_seed(seed: [u32; DEFAULT_SEED_CNT]) -> Self {
        let mt = Mt::new_with_key(seed.iter().copied());
        Self { mt, seed }
    }

    /// Create a new random number generator using the given seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Random;
    /// let seed = [1_u32, 2, 3, 4];
    /// let mut random = Random::with_array_seed(seed);
    /// let rand = random.next_int32();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_byte_array_seed(seed: [u8; DEFAULT_SEED_BYTES]) -> Self {
        let seed = seed_to_key(seed);
        let mt = Mt::new_with_key(seed.iter().copied());
        Self { mt, seed }
    }

    /// Generate next `u32` output.
    ///
    /// Generates a random number on `(0..=0xffffffff)`-interval.
    ///
    /// `u32` is the native output of the generator. This function advances the
    /// RNG step counter by one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::{Error, Random};
    /// # fn example() -> Result<(), Error> {
    /// let mut random = Random::new()?;
    /// assert_ne!(random.next_int32(), random.next_int32());
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn next_int32(&mut self) -> u32 {
        self.mt.next_int32()
    }

    /// Generate next `f64` output.
    ///
    /// Generates a random number on [0,1) with 53-bit resolution.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::{Error, Random};
    /// # fn example() -> Result<(), Error> {
    /// let mut random = Random::new()?;
    /// assert_ne!(random.next_real(), random.next_real());
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn next_real(&mut self) -> f64 {
        self.mt.next_real()
    }

    /// Fill a buffer with bytes generated from the RNG.
    ///
    /// This method generates random `u32`s (the native output unit of the RNG)
    /// until `dest` is filled.
    ///
    /// This method may discard some output bits if `dest.len()` is not a
    /// multiple of 4.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::{Error, Random};
    /// # fn example() -> Result<(), Error> {
    /// let mut random = Random::new()?;
    /// let mut buf = [0; 32];
    /// random.fill_bytes(&mut buf);
    /// assert_ne!([0; 32], buf);
    /// let mut buf = [0; 31];
    /// random.fill_bytes(&mut buf);
    /// assert_ne!([0; 31], buf);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.mt.fill_bytes(dest);
    }

    /// Returns the seed value used to initialize the generator.
    ///
    /// This may be used to initialize another generator with the same state at
    /// a later time, causing it to produce the same sequence of numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Random;
    /// let seed = [1_u32, 2, 3, 4];
    /// let random = Random::with_array_seed(seed);
    /// assert_eq!(random.seed(), seed);
    /// ```
    #[inline]
    #[must_use]
    pub const fn seed(&self) -> [u32; 4] {
        self.seed
    }
}

#[inline]
#[must_use]
#[allow(dead_code)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn int_pair_to_real_inclusive(a: u32, b: u32) -> f64 {
    const MANTISSA_DIGITS: i32 = 53;
    const M: u128 = 1 << MANTISSA_DIGITS | 1;
    let x = (u128::from(a) << 32) | u128::from(b);
    let r = ((x * M) >> 64) as u64 as f64;
    libm::ldexp(r, -MANTISSA_DIGITS)
}

/// Convert a byte array into a reseeding key of `u32`s.
#[inline]
#[must_use]
pub fn seed_to_key(seed: [u8; DEFAULT_SEED_BYTES]) -> [u32; DEFAULT_SEED_CNT] {
    let mut key = [0_u32; DEFAULT_SEED_CNT];
    let iter = key.iter_mut().zip(seed.chunks_exact(size_of::<u32>()));

    let mut bytes = [0; size_of::<u32>()];
    for (cell, chunk) in iter {
        bytes.copy_from_slice(chunk);
        *cell = u32::from_le_bytes(bytes);
    }
    key
}

/// Read a new [`Random`] seed, using platform-provided randomness.
///
/// # Examples
///
/// ```
/// # use spinoso_random::{Error, Random};
/// # fn example() -> Result<(), Error> {
/// let seed = spinoso_random::new_seed()?;
/// # Ok(())
/// # }
/// example().unwrap();
/// ```
///
/// # Errors
///
/// If the randomness feature provided by the platform is not present or failed
/// to completely generate a seed, an error is returned. This error should be
/// raised as a [Ruby `RuntimeError`].
///
/// [Ruby `RuntimeError`]: https://ruby-doc.org/core-2.6.3/RuntimeError.html
#[inline]
pub fn new_seed() -> Result<[u32; DEFAULT_SEED_CNT], NewSeedError> {
    let mut seed = [0; DEFAULT_SEED_BYTES];
    if getrandom::getrandom(&mut seed).is_err() {
        return Err(NewSeedError::new());
    }
    let seed = seed_to_key(seed);
    Ok(seed)
}

#[cfg(test)]
mod tests {
    use std::format;

    use super::Random;

    #[test]
    fn fmt_debug_does_not_leak_seed() {
        let random = Random::with_seed(874);
        let debug = format!("{:?}", random);
        assert!(!debug.contains("894"));
        assert_eq!(debug, "Random {}");

        let random = Random::with_seed(123_456);
        let debug = format!("{:?}", random);
        assert!(!debug.contains("123456"));
        assert_eq!(debug, "Random {}");
    }
}
