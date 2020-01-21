use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{self, Rng, SeedableRng};

use crate::extn::core::random::backend;
use crate::extn::prelude::*;

#[must_use]
pub fn new(seed: Option<u64>) -> Box<dyn backend::Rand> {
    Box::new(Rand::<SmallRng>::new(seed))
}

#[derive(Debug, Clone)]
pub struct Rand<T> {
    rng: T,
    seed: u64,
}

impl<T> Rand<T>
where
    T: SeedableRng,
{
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(rand::random);
        let rng = T::seed_from_u64(seed);
        Self { rng, seed }
    }
}

impl<T> backend::Rand for Rand<T>
where
    T: 'static + Rng,
{
    fn bytes(&mut self, interp: &Artichoke, buf: &mut [u8]) -> Result<(), Exception> {
        let _ = interp;
        self.rng.fill_bytes(buf);
        Ok(())
    }

    fn seed(&self, interp: &Artichoke) -> Result<u64, Exception> {
        let _ = interp;
        Ok(self.seed)
    }

    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn backend::Rand) -> bool {
        let _ = interp;
        if let Ok(other) = other.downcast_ref::<Self>() {
            // This is not quite right. It needs to take into account bytes
            // read from the PRNG.
            self.seed == other.seed
        } else {
            false
        }
    }

    fn rand_int(&mut self, interp: &Artichoke, max: Int) -> Result<Int, Exception> {
        let _ = interp;
        let between = Uniform::from(0..max);
        Ok(between.sample(&mut self.rng))
    }

    fn rand_float(&mut self, interp: &Artichoke, max: Option<Float>) -> Result<Float, Exception> {
        let _ = interp;
        let max = max.unwrap_or(1.0);
        let between = Uniform::from(0.0..max);
        Ok(between.sample(&mut self.rng))
    }
}
