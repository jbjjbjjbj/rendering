use pathtrace::camera::{Camera, Film};
use pathtrace::scene::Scene;
use pathtrace::trace::Tracer;
use pathtrace::scene::shapes::Sphere;
use pathtrace::core::{Vector2i, Vector3f};
use pathtrace::render::{RenderContext, RenderTask};

fn main() {
    let res = Vector2i::new_xy(500, 500);

    let cam = Camera::new(
        Vector3f::new_xyz(10.0, 0.0, 0.0),
        Vector3f::new(0.0),
        Vector3f::new_xyz(0.0, 0.1, 0.0),
        90.0, res,
        );

    let mut scn = Scene::new();
    scn.add_shape(
        Box::new(Sphere::new(4.0, Vector3f::new(0.0))),
        );

    let tracer = Tracer::new();

    let ctx = RenderContext { cam: &cam, scn: &scn, trc: &tracer };

    let mut film = Film::new(res);
    let tile = film.get_tile(&film.frame);

    let mut task = RenderTask::new(Box::new(tile), 1);
    task.render(&ctx);

    film.commit_tile(&task.tile);

    let image = film.finalize_image();
    if let Err(e) = image.save("test.png") {
        println!("Failed to save {}", e);
    }

}
