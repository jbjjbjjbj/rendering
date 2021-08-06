use rendering::camera::{Camera, Film, CameraSettings};
use rendering::world::{Scene, Object, shapes::{Rect, Plane, Sphere, BoxShp}, Instancable};
use rendering::trace::DefaultTracer;
use rendering::core::{Vector2i, Vector3f, Spectrum};
use rendering::render::{RenderContext, RenderCoord};
use rendering::sample::UniformSampler;
use rendering::material::*;

use std::sync::Arc;

fn main() {
    let res = Vector2i::new_xy(500, 500);

    let cam = Camera::new(&CameraSettings {
        target: Vector3f::new_xyz(278.0, 278.0, 0.0),
        origin: Vector3f::new_xyz(278.0, 278.0, -800.0),
        up: Vector3f::new_xyz(0.0, 1.0, 0.0),
        fov: 40.0, 
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
        Object::new(green.clone(), Rect::new(555.0, 555.0, Plane::YZ).translate(555.0, 277.5, 277.5)),
        Object::new(red.clone(), Rect::new(555.0, 555.0, Plane::YZ).translate(0.0, 277.5, 277.5)),
        Object::new(white.clone(), Rect::new(555.0, 555.0, Plane::XZ).translate(277.5, 0.0, 277.5)),
        Object::new(white.clone(), Rect::new(555.0, 555.0, Plane::XZ).translate(277.5, 555.0, 277.5)),
        Object::new(white.clone(), Rect::new(555.0, 555.0, Plane::XY).translate(277.5, 277.5, 555.0)),
        Object::new(sun, Rect::new(130.0, 105.0, Plane::XZ).translate(278.0, 554.0, 279.5)),
        Object::new(white.clone(), BoxShp::new(Vector3f::new(165.0)).translate(212.5, 82.5, 147.5)),
        Object::new(white.clone(), BoxShp::new(Vector3f::new_xyz(165.0, 330.0, 165.0)).translate(347.5, 165.0, 377.5)),
    ]);

    let tracer = DefaultTracer::new(&scn, Some(5), 
                                    //Some(Box::new(SkyLight::new()))
                                    None
                                    );

    let mut sampler = UniformSampler::new();

    let ctx = RenderContext { cam: &cam, trc: &tracer };

    let mut film = Film::new(res);
    {
        let coord = RenderCoord::new(&mut film, Vector2i::new_xy(50, 50), 50);

        coord.run_threaded(&ctx, &mut sampler, 8);
        //coord.work(&ctx, &mut sampler);
    }

    let image = film.finalize_image();
    if let Err(e) = image.save("test.png") {
        println!("Failed to save {}", e);
    }

}
