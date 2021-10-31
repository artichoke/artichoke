// This source file is derived from `rand_mt`.
//
// See https://github.com/artichoke/rand_mt/blob/aeb3274a/src/mt.rs.

use core::fmt;
use core::mem::size_of;
use core::num::Wrapping;

#[cfg(feature = "rand-traits")]
mod rand;

const N: usize = 624;
const M: usize = 397;
const ONE: Wrapping<u32> = Wrapping(1);
const MATRIX_A: Wrapping<u32> = Wrapping(0x9908_b0df);
const UPPER_MASK: Wrapping<u32> = Wrapping(0x8000_0000);
const LOWER_MASK: Wrapping<u32> = Wrapping(0x7fff_ffff);

/// The 32-bit Ruby flavor of the Mersenne Twister pseudorandom number
/// generator.
///
/// The RNG is used to implement the [`Random` class] in Ruby Core. This RNG is
/// a modified version of the [`MT19937`] random number generator. It natively
/// outputs `u32`.
///
/// **`Mt` is not suitable for cryptographic use.**
///
/// # Size
///
/// `Mt` requires approximately 2.5KB of internal state.
///
/// You may wish to store an `Mt` on the heap in a [`Box`] to make
/// it easier to embed in another struct.
///
/// [`Random` class]: https://ruby-doc.org/core-2.6.3/Random.html
/// [`MT19937`]: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/emt.html
/// [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
#[derive(Clone)]
#[cfg_attr(docsrs, doc(alias = "Mersenne"))]
#[cfg_attr(docsrs, doc(alias = "Twister"))]
#[cfg_attr(docsrs, doc(alias = "Mersenne Twister"))]
#[cfg_attr(docsrs, doc(alias = "mersenne"))]
#[cfg_attr(docsrs, doc(alias = "twister"))]
#[cfg_attr(docsrs, doc(alias = "mersenne twister"))]
#[cfg_attr(docsrs, doc(alias = "MT"))]
#[cfg_attr(docsrs, doc(alias = "MT19937"))]
#[allow(missing_copy_implementations)] // RNGs should not implement `Copy`
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mt {
    idx: usize,
    state: [Wrapping<u32>; N],
}

impl fmt::Debug for Mt {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Mt {}")
    }
}

impl Mt {
    /// Create a new Mersenne Twister random number generator using the given
    /// seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Mt;
    /// let seed = 123_456_789_u32;
    /// let mut mt = Mt::with_seed(seed);
    /// let rand = mt.next_int32();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_seed(seed: u32) -> Self {
        let mut mt = Self {
            idx: 0,
            state: [Wrapping(0); N],
        };
        mt.reseed(seed);
        mt
    }

    /// Create a new Mersenne Twister random number generator using the given
    /// key.
    ///
    /// Key can have any length.
    ///
    /// This function is used when initializing an `Mt` with more than 32 bits,
    /// such as when initializing with a `Bignum` seed.
    ///
    /// MRI initializes the global SipHash seed with 4 `u32`s of entropy.
    #[inline]
    #[must_use]
    pub fn new_with_key<I>(key: I) -> Self
    where
        I: IntoIterator<Item = u32>,
        I::IntoIter: Clone,
    {
        let mut mt = Self {
            idx: 0,
            state: [Wrapping(0); N],
        };
        mt.reseed_with_key(key);
        mt
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
    /// # use spinoso_random::Mt;
    /// let mut mt = Mt::with_seed(5489);
    /// assert_ne!(mt.next_int32(), mt.next_int32());
    /// ```
    #[inline]
    #[must_use]
    pub fn next_int32(&mut self) -> u32 {
        // Failing this check indicates that, somehow, the structure
        // was not initialized.
        debug_assert!(self.idx != 0);
        if self.idx >= N {
            next_state(self);
        }
        let Wrapping(x) = self.state[self.idx];
        self.idx += 1;
        temper(x)
    }

    /// Generate next `f64` output.
    ///
    /// Generates a random number on [0,1) with 53-bit resolution.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Mt;
    /// let mut mt = Mt::with_seed(5489);
    /// assert_ne!(mt.next_real(), mt.next_real());
    /// ```
    #[inline]
    #[must_use]
    pub fn next_real(&mut self) -> f64 {
        let a = self.next_int32();
        let b = self.next_int32();
        int_pair_to_real_exclusive(a, b)
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
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        const CHUNK: usize = size_of::<u32>();
        let mut left = dest;
        while left.len() >= CHUNK {
            let (next, remainder) = left.split_at_mut(CHUNK);
            left = remainder;
            let chunk: [u8; CHUNK] = self.next_int32().to_le_bytes();
            next.copy_from_slice(&chunk);
        }
        let n = left.len();
        if n > 0 {
            let chunk: [u8; CHUNK] = self.next_int32().to_le_bytes();
            left.copy_from_slice(&chunk[..n]);
        }
    }

    /// Reseed a Mersenne Twister from a single `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_random::Mt;
    /// let mut mt = Mt::with_seed(5489_u32);
    /// let first = mt.next_int32();
    /// mt.fill_bytes(&mut [0; 512]);
    /// mt.reseed(5489_u32);
    /// assert_eq!(first, mt.next_int32());
    /// ```
    #[inline]
    #[allow(clippy::cast_possible_truncation)] // N is less than u32::MAX
    pub fn reseed(&mut self, seed: u32) {
        let Self { state, idx } = self;
        *idx = N;
        state[0] = Wrapping(seed);

        for i in 1..N {
            state[i] = Wrapping(1_812_433_253) * (state[i - 1] ^ (state[i - 1] >> 30)) + Wrapping(i as u32);
        }
    }

    /// Reseed a Mersenne Twister from an iterator of `u32`s.
    ///
    /// Key can have any length.
    #[inline]
    #[allow(clippy::cast_possible_truncation)] // N is less than u32::MAX
    pub fn reseed_with_key<I>(&mut self, key: I)
    where
        I: IntoIterator<Item = u32>,
        I::IntoIter: Clone,
    {
        self.reseed(19_650_218_u32);

        let Self { state, .. } = self;
        let mut i = 1_usize;

        for (j, piece) in key.into_iter().enumerate().cycle().take(N) {
            state[i] = (state[i] ^ ((state[i - 1] ^ (state[i - 1] >> 30)) * Wrapping(1_664_525)))
                + Wrapping(piece)
                + Wrapping(j as u32);
            i += 1;
            if i >= N {
                state[0] = state[N - 1];
                i = 1;
            }
        }
        for _ in 0..N - 1 {
            state[i] =
                (state[i] ^ ((state[i - 1] ^ (state[i - 1] >> 30)) * Wrapping(1_566_083_941))) - Wrapping(i as u32);
            i += 1;
            if i >= N {
                state[0] = state[N - 1];
                i = 1;
            }
        }
        state[0] = Wrapping(1 << 31);
    }
}

#[inline]
fn next_state(rng: &mut Mt) {
    let Mt { state, idx } = rng;

    for i in 0..N - M {
        state[i] = state[i + M] ^ twist(state[i], state[i + 1]);
    }
    for i in N - M..N - 1 {
        state[i] = state[i + M - N] ^ twist(state[i], state[i + 1]);
    }
    state[N - 1] = state[M - 1] ^ twist(state[N - 1], state[0]);
    *idx = 0;
}

#[inline]
fn mixbits(u: Wrapping<u32>, v: Wrapping<u32>) -> Wrapping<u32> {
    (u & UPPER_MASK) | (v & LOWER_MASK)
}

#[inline]
fn twist(u: Wrapping<u32>, v: Wrapping<u32>) -> Wrapping<u32> {
    let left = mixbits(u, v) >> 1;
    let right = if v & ONE == ONE { MATRIX_A } else { Wrapping(0) };
    left ^ right
}

#[inline]
fn int_pair_to_real_exclusive(mut a: u32, mut b: u32) -> f64 {
    a >>= 5;
    b >>= 6;
    let a = f64::from(a);
    let b = f64::from(b);
    (a * 67_108_864.0 + b) * (1.0 / 9_007_199_254_740_992.0)
}

#[inline]
const fn temper(mut x: u32) -> u32 {
    x ^= x >> 11;
    x ^= (x << 7) & 0x9d2c_5680;
    x ^= (x << 15) & 0xefc6_0000;
    x ^= x >> 18;
    x
}

#[cfg(test)]
mod tests {
    use core::iter;
    use std::format;

    use super::Mt;

    #[test]
    fn fmt_debug_does_not_leak_seed() {
        let mt = Mt::with_seed(874);
        let debug = format!("{:?}", mt);
        assert!(!debug.contains("894"));

        let mt = Mt::with_seed(123_456);
        let debug = format!("{:?}", mt);
        assert!(!debug.contains("123456"));
        assert_eq!(debug, "Mt {}");
    }

    #[test]
    fn seed_with_empty_iter_returns() {
        let _mt = Mt::new_with_key(iter::empty());
    }
}
