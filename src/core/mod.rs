//! Contains a collection of core modules used by other modules
//!
//! Also creates a shortcut for some common types

mod vector2;
mod vector3;
mod bound2;
mod bound3;
mod spectrum;
mod ray;

pub use vector2::{Vector2i, Vector2f};
pub use vector3::Vector3f;
pub use bound2::{Bound2i, Bound2f};
pub use bound3::Bound3f;
pub use spectrum::Spectrum;
pub use ray::Ray;

use crate::Number;

pub fn min<T: Number> (a: T, b: T) -> T {
    if b < a {
        return b;
    }
    a
}

pub fn max<T: Number> (a: T, b: T) -> T {
    if b > a {
        return b;
    }
    a
}

