//! Contains a collection of core modules used by other modules
//!
//! Also creates a shortcut for some common types

pub mod vector2;
pub mod vector3;
pub mod bound;
pub mod spectrum;
pub mod hittable;
mod ray;

pub use vector2::{Vector2i, Vector2f};
pub use vector3::Vector3f;
pub use bound::{Bound2i, Bound2f};
pub use spectrum::Spectrum;
pub use ray::Ray;
pub use hittable::{Hittable, Intersection};
