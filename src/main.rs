use pathtrace::camera::film::Film;
use pathtrace::vector::{Vector2i};
use pathtrace::bound::Bound2i;


fn main() {

    let film = Film::new(Vector2i::new(100));
    let tile = film.get_tile(&Bound2i::new_xyxy(10, 10, 100, 100));

    //println!("Yo {}", tile.bounds.min.x);
}
