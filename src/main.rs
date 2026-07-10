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
use material::{Lambertian, Material, Metal};
use sphere::Sphere;
use std::sync::Arc;
use vec3::Point3;

fn main() {
    let mut world = HittableList::new();
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::clone(&material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Arc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::clone(&material_right),
    )));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
