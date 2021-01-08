use rand_core::impls::next_u64_via_u32;
use rand_core::{Error, RngCore, SeedableRng};

use super::Mt;
use crate::random::{seed_to_key, DEFAULT_SEED_BYTES};

impl SeedableRng for Mt {
    type Seed = [u8; DEFAULT_SEED_BYTES];

    /// Reseed from four `u32`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand_core::{RngCore, SeedableRng};
    /// # use spinoso_random::Mt;
    /// // Default MT seed
    /// let seed = 5489_u128.to_le_bytes();
    /// let mut mt = Mt::from_seed(seed);
    /// assert_ne!(mt.next_u32(), mt.next_u32());
    /// ```
    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        let seed = seed_to_key(seed);
        Self::new_with_key(seed.iter().copied())
    }
}

impl RngCore for Mt {
    /// Generate next `u64` output.
    ///
    /// This function is implemented by generating two `u32`s from the RNG and
    /// shifting + masking them into a `u64` output.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand_core::RngCore;
    /// # use spinoso_random::Mt;
    /// let mut mt = Mt::with_seed(5489);
    /// assert_ne!(mt.next_u64(), mt.next_u64());
    /// ```
    #[inline]
    #[must_use]
    fn next_u64(&mut self) -> u64 {
        next_u64_via_u32(self)
    }

    /// Generate next `u32` output.
    ///
    /// `u32` is the native output of the generator. This function advances the
    /// RNG step counter by one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand_core::RngCore;
    /// # use spinoso_random::Mt;
    /// let mut mt = Mt::with_seed(5489);
    /// assert_ne!(mt.next_u32(), mt.next_u32());
    /// ```
    #[inline]
    #[must_use]
    fn next_u32(&mut self) -> u32 {
        self.next_int32()
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
    /// # use rand_core::RngCore;
    /// # use spinoso_random::Mt;
    /// let mut mt = Mt::with_seed(5489);
    /// let mut buf = [0; 32];
    /// mt.fill_bytes(&mut buf);
    /// assert_ne!([0; 32], buf);
    /// let mut buf = [0; 31];
    /// mt.fill_bytes(&mut buf);
    /// assert_ne!([0; 31], buf);
    /// ```
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        Self::fill_bytes(self, dest)
    }

    /// Fill a buffer with bytes generated from the RNG.
    ///
    /// This method generates random `u32`s (the native output unit of the RNG)
    /// until `dest` is filled.
    ///
    /// This method may discard some output bits if `dest.len()` is not a
    /// multiple of 4.
    ///
    /// `try_fill_bytes` is implemented with [`fill_bytes`] and is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rand_core::{Error, RngCore};
    /// # use spinoso_random::Mt;
    /// # fn example() -> Result<(), Error> {
    /// let mut mt = Mt::with_seed(5489);
    /// let mut buf = [0; 32];
    /// mt.try_fill_bytes(&mut buf)?;
    /// assert_ne!([0; 32], buf);
    /// let mut buf = [0; 31];
    /// mt.try_fill_bytes(&mut buf)?;
    /// assert_ne!([0; 31], buf);
    /// # Ok(())
    /// # }
    /// # example().unwrap()
    /// ```
    ///
    /// # Errors
    ///
    /// This method never returns an error. It is equivalent to calling the
    /// infallible [`fill_bytes`] method.
    ///
    /// [`fill_bytes`]: Mt::fill_bytes
    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Self::fill_bytes(self, dest);
        Ok(())
    }
}
