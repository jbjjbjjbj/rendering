use crate::Float;
use super::Sampler;

use rand::prelude::*;
use rand::distributions::Uniform;
use rand_pcg::Pcg32;

#[derive(Clone)]
pub struct UniformSampler {
    r: Pcg32,
    d: Uniform<Float>,
}

impl UniformSampler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for UniformSampler {
    fn default() -> Self {
        Self {
            r: Pcg32::seed_from_u64(1),
            d: Uniform::new(0.0, 1.0),
        }
    }
}

impl Sampler for UniformSampler {
    fn get_sample(&mut self) -> Float {
        self.d.sample(&mut self.r)
    }

    fn clone_and_seed(&mut self) -> Box<dyn Sampler + Send> {
        let mut n = self.clone();
        n.r = Pcg32::seed_from_u64(self.r.next_u64());
        Box::new(n)
    }
}
