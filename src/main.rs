mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Material};
use sphere::Sphere;
use std::f64::consts::PI;
use std::sync::Arc;
use vec3::Point3;

fn main() {
    let mut world = HittableList::new();
    let r = (PI / 4.0).cos();
    let material_left: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        Arc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        Arc::clone(&material_right),
    )));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 90.0;
    cam.render(&world);
}
