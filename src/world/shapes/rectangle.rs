use crate::{NEAR_ZERO, Float};
use crate::core::{Ray, Vector3f, Bound3f};
use crate::world::{Hittable, DynHittable, Intersection, Instancable};

const OUT_XY: Vector3f = Vector3f { x: 0.0, y: 0.0, z: 1.0 };

pub enum Plane {
    XY,
}

pub struct Rect {
    // Size of rectangle in first dimension
    d1: Float,
    // Size in second
    d2: Float,
    // Offset of plane plane. Should only be used when composing other types
    offset: Float,

    plane: Plane,
}

impl Rect {
    pub fn new(width: Float, height: Float, plane: Plane) -> Self {
        Rect {d1: width, d2: height, offset: 0.0, plane}
    }

    pub fn new_with_offset(d1: Float, d2: Float, offset: Float, plane: Plane) -> Self {
        Rect {
            d1, d2,
            offset,
            plane
        }
    }
}

impl Hittable for Rect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let t = (self.offset-ray.origin.z) / ray.direction.z;
        if t <= NEAR_ZERO {
            return None;
        }

        let poi = ray.at(t);
        // Check if at is inside rectangle on plane
        if poi.x.abs() > (self.d1/2.0) || poi.y.abs() > (self.d2/2.0) {
            // No collision
            return None;
        }

        Some(Intersection::new(OUT_XY, poi, ray, t))
    }

    fn bounding_box(&self) -> Bound3f {
        // A rectangle has no area, but we will give it some for its bounding box
        Bound3f::new(
            Vector3f::new_xyz(-self.d1, -self.d2, self.offset - 0.0001),
            Vector3f::new_xyz(self.d1, self.d2, self.offset + 0.0001)
            )
    }
}

impl Into<DynHittable> for Rect {
    fn into(self) -> DynHittable {
        DynHittable::new(Box::new(self))
    }
}

impl Instancable for Rect {}

impl Plane {
    pub fn switch(&self, v: Vector3f) -> Vector3f {
        match self {
            Plane::XY => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xy_rect() {
        let rect = Rect::new_xy(5.0, 5.0);
        let ray = Ray {
            origin: Vector3f::new_xyz(0.0, 1.0, 1.5),
            direction: Vector3f::new_xyz(0.0, 0.0, -1.0),
        };

        let t = rect.intersect(&ray).unwrap().t;
        assert!((t - 1.5).abs() < 0.01);
    }

    #[test]
    fn xy_rect_offset() {
        let rect = Rect::new_with_offset(5.0, 5.0, -1.0, Plane::XY);
        let ray = Ray {
            origin: Vector3f::new_xyz(0.0, 1.0, 1.5),
            direction: Vector3f::new_xyz(0.0, 0.0, -1.0),
        };

        let t = rect.intersect(&ray).unwrap().t;
        assert!((t - 2.5).abs() < 0.01);
    }
}
