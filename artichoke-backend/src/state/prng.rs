use rand::rngs::SmallRng;

use crate::extn::core::random::backend::rand::Rand;
use crate::extn::core::random::backend::RandType;
use crate::types::{Float, Int};

#[derive(Debug)]
pub struct Prng {
    random: Rand<SmallRng>,
}

impl Prng {
    #[must_use]
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            random: Rand::new(seed),
        }
    }

    #[must_use]
    pub fn seed(&self) -> u64 {
        self.random.seed()
    }

    pub fn reseed(&mut self, new_seed: Option<u64>) {
        self.random = Rand::new(new_seed);
    }

    pub fn has_same_internal_state(&self, other: &dyn RandType) -> bool {
        self.random.has_same_internal_state(other)
    }

    pub fn bytes(&mut self, buf: &mut [u8]) {
        self.random.bytes(buf);
    }

    pub fn rand_int(&mut self, max: Int) -> Int {
        self.random.rand_int(max)
    }

    pub fn rand_float(&mut self, max: Option<Float>) -> Float {
        self.random.rand_float(max)
    }
}

impl Default for Prng {
    fn default() -> Self {
        Self::new(None)
    }
}
