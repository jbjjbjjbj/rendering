use crate::core::{Ray, Bound3f, Vector3f};
use crate::world::shapes::{Plane, Rect};
use crate::world::{Instancable, Intersection, Hittable, DynHittable};
use crate::world::container::HittableList;

pub struct BoxShp {
    sides: HittableList,
}

impl BoxShp {
    pub fn new(size: Vector3f) -> Self {
        let mut cont = HittableList::new();
        cont.add(Rect::new_with_offset(size.x, size.y, size.z / 2.0, Plane::XY));
        cont.add(Rect::new_with_offset(size.x, size.y, -size.z / 2.0, Plane::XY));
        cont.add(Rect::new_with_offset(size.x, size.z, size.y / 2.0, Plane::XZ));
        cont.add(Rect::new_with_offset(size.x, size.z, -size.y / 2.0, Plane::XZ));
        cont.add(Rect::new_with_offset(size.y, size.z, size.x / 2.0, Plane::YZ));
        cont.add(Rect::new_with_offset(size.y, size.z, -size.x / 2.0, Plane::YZ));

        Self {sides: cont}
    }
}

impl Into<DynHittable> for BoxShp {
    fn into(self) -> DynHittable {
        DynHittable::new(Box::new(self))
    }
}

impl Hittable for BoxShp {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.sides.intersect(ray)
    }

    fn bounding_box(&self) -> Bound3f {
        self.sides.bounding_box()
    }
}

impl Instancable for BoxShp {}
