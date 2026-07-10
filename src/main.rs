mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
