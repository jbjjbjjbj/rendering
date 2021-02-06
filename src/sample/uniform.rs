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
        Self {
            r: Pcg32::seed_from_u64(1),
            d: Uniform::from(0.0..1.0),
        }
    }
}

impl Sampler for UniformSampler {
    fn get_sample(&mut self) -> Float {
        self.d.sample(&mut self.r)
    }
}
