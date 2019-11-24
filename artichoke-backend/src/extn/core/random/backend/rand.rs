use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{self, Rng, SeedableRng};

use crate::extn::core::exception::RubyException;
use crate::extn::core::random;
use crate::types::{Float, Int};
use crate::Artichoke;

pub fn new(seed: Option<u64>) -> Box<dyn random::Rand> {
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
        if let Some(seed) = seed {
            let rng = T::seed_from_u64(seed);
            Self { rng, seed }
        } else {
            let seed = rand::random();
            let rng = T::seed_from_u64(seed);
            Self { rng, seed }
        }
    }
}

impl<T> random::Rand for Rand<T>
where
    T: 'static + Rng,
{
    fn bytes(&mut self, interp: &Artichoke, buf: &mut [u8]) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        self.rng.fill_bytes(buf);
        Ok(())
    }

    fn seed(&self, interp: &Artichoke) -> Result<u64, Box<dyn RubyException>> {
        let _ = interp;
        Ok(self.seed)
    }

    fn has_same_internal_state(&self, interp: &Artichoke, other: &dyn random::Rand) -> bool {
        let _ = interp;
        if let Ok(other) = other.downcast_ref::<Self>() {
            self.seed == other.seed
        } else {
            false
        }
    }

    fn rand_int(&mut self, interp: &Artichoke, max: Int) -> Result<Int, Box<dyn RubyException>> {
        let _ = interp;
        let between = Uniform::from(0..max);
        Ok(between.sample(&mut self.rng))
    }

    fn rand_float(
        &mut self,
        interp: &Artichoke,
        max: Option<Float>,
    ) -> Result<Float, Box<dyn RubyException>> {
        let _ = interp;
        let max = max.unwrap_or(1.0);
        let between = Uniform::from(0.0..max);
        Ok(between.sample(&mut self.rng))
    }
}
