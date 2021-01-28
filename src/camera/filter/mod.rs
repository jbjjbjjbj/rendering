use crate::vector::Vector2f;
use crate::Float;

trait Eval {
    fn eval(&self, point: &Vector2f) -> Float;
}

pub struct Filter {
    eval: Box<dyn Eval>,
    pub radius: Vector2f,
}

struct BoxFilter {}

// The same a no filter, and can give aliasing in final image
impl Eval for BoxFilter {
    fn eval(&self, _: &Vector2f) -> Float {
        1.0
    }
}

impl Eval for Filter {
    fn eval(&self, point: &Vector2f) -> Float {
        self.eval.eval(point)
    }
}

impl Filter {
    pub fn new_box(radius: Vector2f) -> Filter {
        Filter { radius: radius, eval: Box::new(BoxFilter {}) }
    }
}
