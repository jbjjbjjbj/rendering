use rendering::camera::{Camera, Film, CameraSettings};
use rendering::world::{Scene, Object, shapes::{Rect, Plane, Sphere}, Instancable};
use rendering::trace::DefaultTracer;
use rendering::core::{Vector2i, Vector3f, Spectrum};
use rendering::render::{RenderContext, RenderCoord};
use rendering::sample::UniformSampler;
use rendering::material::*;

use std::sync::Arc;

fn main() {
    let res = Vector2i::new_xy(500, 500);

    let cam = Camera::new(&CameraSettings {
        target: Vector3f::new_xyz(0.0, 0.0, 0.0),
        origin: Vector3f::new_xyz(0.0, 0.0, 5.0),
        up: Vector3f::new_xyz(0.0, 1.0, 0.0),
        fov: 85.0, 
        filmsize: res,
        focus: None,
        aperture: None,
    });

    let red = Arc::new(Lambertian::new(Spectrum::new_rgb(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Spectrum::new_rgb(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Spectrum::new_rgb(0.12, 0.45, 0.15)));
    let metal = Arc::new(Reflectant::new(Spectrum::new_rgb(0.8, 0.8, 0.9), Some(1.0)));
    let mirror = Arc::new(Reflectant::new(Spectrum::new_rgb(1.0, 1.0, 1.0), None));
    let glass = Arc::new(Dielectric::new(1.5));
    let sun = Arc::new(DiffuseLight::new_white(15.0));

    let mut scn = Scene::new();
    scn.add_objects(vec![
        Object::new(white.clone(), Rect::new_with_offset(10.0, 10.0, -5.0, Plane::XY)),
        Object::new(white.clone(), Rect::new_with_offset(10.0, 10.0, 5.0, Plane::XY)),
        Object::new(white.clone(), Rect::new_with_offset(10.0, 10.0, -5.0, Plane::XZ)),
        Object::new(white.clone(), Rect::new_with_offset(10.0, 10.0, 5.0, Plane::XZ)),
        Object::new(red.clone(), Rect::new_with_offset(10.0, 10.0, -5.0, Plane::YZ)),
        Object::new(green.clone(), Rect::new_with_offset(10.0, 10.0, 5.0, Plane::YZ)),
        Object::new(sun, Rect::new(1.4, 1.0, Plane::XZ).translate(0.0, 4.99, -2.5)),
    ]);

    let tracer = DefaultTracer::new(&scn, Some(5), 
                                    Some(Box::new(SkyLight::new()))
                                    //None
                                    );

    let mut sampler = UniformSampler::new();

    let ctx = RenderContext { cam: &cam, trc: &tracer };

    let mut film = Film::new(res);
    {
        let coord = RenderCoord::new(&mut film, Vector2i::new_xy(50, 50), 500);

        coord.run_threaded(&ctx, &mut sampler, 8);
        //coord.work(&ctx, &mut sampler);
    }

    let image = film.finalize_image();
    if let Err(e) = image.save("test.png") {
        println!("Failed to save {}", e);
    }

}
