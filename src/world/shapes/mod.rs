mod sphere;
mod rectangle;
mod r#box;

pub use sphere::Sphere;
pub use rectangle::{Rect, Plane};
pub use r#box::BoxShp;

use crate::world::{Hittable, Intersection};
use crate::core::{Bound3f, Ray};
