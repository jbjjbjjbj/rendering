use pathtrace::camera::{Camera, Film, CameraSettings};
use pathtrace::scene::{Scene, Object};
use pathtrace::trace::DefaultTracer;
use pathtrace::scene::shapes::Sphere;
use pathtrace::core::{Vector2i, Vector3f, Spectrum};
use pathtrace::render::{RenderContext, RenderTask};
use pathtrace::sample::UniformSampler;
use pathtrace::material::Lambertian;

use std::rc::Rc;

fn main() {
    let res = Vector2i::new_xy(500, 500);

    let cam = Camera::new(&CameraSettings {
        target: Vector3f::new_xyz(0.0, 0.0, -1.0),
        origin: Vector3f::new_xyz(0.0, 0.0, 0.0),
        up: Vector3f::new_xyz(0.0, 1.0, 0.0),
        fov: 90.0, 
        screensize: res,
    });

    let brown = Rc::new(Lambertian::new(Spectrum::new_rgb(0.5, 0.3, 0.0)));
    let blue = Rc::new(Lambertian::new(Spectrum::new_rgb(0.0, 0.3, 0.7)));

    let mut scn = Scene::new();
    scn.add_objects(vec![
        Object::new(blue.clone(), Box::new(Sphere::new(0.5, Vector3f::new_xyz(0.0, 0.0, -1.0)))),
        Object::new(brown.clone(), Box::new(Sphere::new(100.0, Vector3f::new_xyz(0.0, -100.5, -1.0)))),
    ]);

    let tracer = DefaultTracer::new(&scn, None);

    let mut sampler = UniformSampler::new();

    let ctx = RenderContext { cam: &cam, trc: &tracer };

    let mut film = Film::new(res);
    let tile = film.get_tile(&film.frame);

    let mut task = RenderTask::new(Box::new(tile), 10);
    task.render(&ctx, &mut sampler);

    film.commit_tile(&task.tile);

    let image = film.finalize_image();
    if let Err(e) = image.save("test.png") {
        println!("Failed to save {}", e);
    }

}
