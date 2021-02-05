use crate::Float;
use crate::core::Vector2f;

mod uniform;

pub use uniform::UniformSampler;

pub trait Sampler {
    fn get_sample(&mut self) -> Float;
    fn get_sample_2d(&mut self) -> Vector2f;
}
