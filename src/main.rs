mod color;
mod ray;
mod vec3;
use color::Color;
use image::{ImageBuffer, RgbImage};
use ray::Ray;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *center - *r.origin();
    let a = Vec3::dot(r.direction(), r.direction());
    let b = -2.0 * Vec3::dot(r.direction(), &oc);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn main() {
    let path = std::path::Path::new("output/book1/image3.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    if image_height < 1 {
        image_height = 1;
    }

    let mut img: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport vertical vector
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    // pixel delta
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_center;
            let r = Ray::from(camera_center, ray_direction);

            let pixel = img.get_pixel_mut(i as u32, j as u32);
            let pixel_color = ray_color(&r);
            let r = (255.999 * pixel_color.x()) as u8;
            let g = (255.999 * pixel_color.y()) as u8;
            let b = (255.999 * pixel_color.z()) as u8;

            *pixel = image::Rgb([r, g, b]);
        }
    }
    eprintln!("\rDone.");
    img.save(path).expect("Cannot save the image to the file");
}
