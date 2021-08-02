use rendering::camera::{Camera, Film, CameraSettings};
use rendering::world::{Scene, Object, shapes::Sphere};
use rendering::trace::DefaultTracer;
use rendering::core::{Vector2i, Vector3f, Spectrum};
use rendering::render::{RenderContext, RenderCoord};
use rendering::sample::UniformSampler;
use rendering::material::*;

use std::sync::Arc;

fn main() {
    let res = Vector2i::new_xy(500, 500);

    let cam = Camera::new(&CameraSettings {
        target: Vector3f::new_xyz(0.0, 0.0, -1.0),
        origin: Vector3f::new_xyz(0.0, 0.0, 1.0),
        up: Vector3f::new_xyz(0.0, 1.0, 0.0),
        fov: 45.0, 
        filmsize: res,
        focus: None,
        aperture: Some(10.0),
        //aperture: None,
    });

    let brown = Arc::new(Lambertian::new(Spectrum::new_rgb(0.5, 0.3, 0.0)));
    let blue = Arc::new(Lambertian::new(Spectrum::new_rgb(0.0, 0.3, 0.7)));
    let green = Arc::new(Lambertian::new(Spectrum::new_rgb(0.0, 0.7, 0.3)));
    let metal = Arc::new(Reflectant::new(Spectrum::new_rgb(0.8, 0.8, 0.9), Some(1.0)));
    let glass = Arc::new(Dielectric::new(1.5));
    let sun = Arc::new(DiffuseLight::new_white(50.0));

    let mut scn = Scene::new();
    scn.add_objects(vec![
        Object::new(glass, Sphere::new(0.2, Vector3f::new_xyz(0.0, 0.0, -1.0))),
        Object::new(blue, Sphere::new(0.5, Vector3f::new_xyz(1.0, 0.0, -1.5))),
        Object::new(green, Sphere::new(0.3, Vector3f::new_xyz(0.5, 0.0, -2.5))),
        Object::new(brown, Sphere::new(100.0, Vector3f::new_xyz(0.0, -100.5, -1.0))),
        Object::new(metal, Sphere::new(0.2, Vector3f::new_xyz(-0.5, 0.0, -1.0))),
        Object::new(sun, Sphere::new(0.4, Vector3f::new_xyz(-1.0, 3.0, 0.0))),
    ]);

    let tracer = DefaultTracer::new(&scn, Some(50), 
                                    //Some(Box::new(SkyLight::new()))
                                    None
                                    );

    let mut sampler = UniformSampler::new();

    let ctx = RenderContext { cam: &cam, trc: &tracer };

    let mut film = Film::new(res);
    {
        let coord = RenderCoord::new(&mut film, Vector2i::new_xy(50, 50), 50);

        coord.run_threaded(&ctx, &mut sampler, 8);
    }

    let image = film.finalize_image();
    if let Err(e) = image.save("test.png") {
        println!("Failed to save {}", e);
    }

}
