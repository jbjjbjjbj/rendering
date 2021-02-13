use rendering::camera::{Camera, Film, CameraSettings};
use rendering::world::{Scene, Object, shapes::Sphere};
use rendering::trace::DefaultTracer;
use rendering::core::{Vector2i, Vector3f, Spectrum};
use rendering::render::{RenderContext, RenderCoord};
use rendering::sample::UniformSampler;
use rendering::material::{Reflectant, Lambertian};

use std::sync::Arc;

fn main() {
    let res = Vector2i::new_xy(500, 500);

    let cam = Camera::new(&CameraSettings {
        target: Vector3f::new_xyz(0.0, 0.0, -1.0),
        origin: Vector3f::new_xyz(1.7, 0.0, 0.3),
        up: Vector3f::new_xyz(0.0, 1.0, 0.0),
        fov: 40.0, 
        filmsize: res,
        focus: None,
        aperture: Some(20.0),
    });

    let brown = Arc::new(Lambertian::new(Spectrum::new_rgb(0.5, 0.3, 0.0)));
    let blue = Arc::new(Lambertian::new(Spectrum::new_rgb(0.0, 0.3, 0.7)));
    let metal = Arc::new(Reflectant::new(Spectrum::new_rgb(0.75, 0.75, 0.75), None));

    let mut scn = Scene::new();
    scn.add_objects(vec![
        Object::new(metal, Box::new(Sphere::new(0.5, Vector3f::new_xyz(0.0, 0.0, -1.0)))),
        Object::new(blue, Box::new(Sphere::new(0.5, Vector3f::new_xyz(1.0, 0.0, -1.0)))),
        Object::new(brown, Box::new(Sphere::new(100.0, Vector3f::new_xyz(0.0, -100.5, -1.0)))),
    ]);

    let tracer = DefaultTracer::new(&scn, Some(50));

    let mut sampler = UniformSampler::new();

    let ctx = RenderContext { cam: &cam, trc: &tracer };

    let mut film = Film::new(res);
    {
        let coord = RenderCoord::new(&mut film, Vector2i::new_xy(32, 32), 300);

        coord.run_threaded(&ctx, &mut sampler, 4);
    }

    let image = film.finalize_image();
    if let Err(e) = image.save("test.png") {
        println!("Failed to save {}", e);
    }

}
