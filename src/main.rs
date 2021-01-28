use pathtrace::camera::filter::Filter;
use pathtrace::camera::film::Film;
use pathtrace::vector::{Vector2i, Vector2f};
use pathtrace::bound::Bound2i;

use std::rc::Rc;

fn main() {

    let filter = Rc::new(Filter::new_box(Vector2f::new(2.5)));

    let film = Film::new(Vector2i::new(100), filter.clone());
    let tile = film.get_tile(&Bound2i::new_xyxy(10, 10, 100, 100));

    println!("Yo {}", tile.bounds.min.x);
}
