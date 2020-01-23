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

impl<T> Rand<T> {
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl<T> Rand<T>
where
    T: 'static,
{
    pub fn has_same_internal_state(&self, other: &dyn backend::Rand) -> bool {
        if let Ok(other) = other.downcast_ref::<Self>() {
            // This is not quite right. It needs to take into account bytes
            // read from the PRNG.
            self.seed == other.seed
        } else {
            false
        }
    }
}

impl<T> Rand<T>
where
    T: Rng,
{
    pub fn bytes(&mut self, buf: &mut [u8]) {
        self.rng.fill_bytes(buf);
    }

    pub fn rand_int(&mut self, max: Int) -> Int {
        let between = Uniform::from(0..max);
        between.sample(&mut self.rng)
    }

    pub fn rand_float(&mut self, max: Option<Float>) -> Float {
        let max = max.unwrap_or(1.0);
        let between = Uniform::from(0.0..max);
        between.sample(&mut self.rng)
    }
}

impl<T> backend::Rand for Rand<T>
where
    T: 'static + Rng,
{
    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) {
        let _ = interp;
        self.bytes(buf);
    }

    fn seed(&self, interp: &Artichoke) -> u64 {
        let _ = interp;
        self.seed()
    }

    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn backend::Rand) -> bool {
        let _ = interp;
        self.has_same_internal_state(other)
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Int {
        let _ = interp;
        self.rand_int(max)
    }

    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Float>) -> Float {
        let _ = interp;
        self.rand_float(max)
    }
}
