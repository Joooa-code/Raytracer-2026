mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod image;
mod interval;
mod material;
mod model;
mod perlin;
mod quad;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod triangle;
mod vec3;
use bvh::BVHNode;
use camera::Camera;
use color::Color;
use constant_medium::ConstantMedium;
use hittable::*;
use hittable_list::HittableList;
use material::*;
use model::load;
use quad::Quad;
use rtweekend::{random_f64, random_f64_range};
use sphere::Sphere;
use std::sync::Arc;
use texture::*;
use vec3::{Point3, Vec3};

fn bouncing_spheres() {
    let mut world = HittableList::default();
    let ground_material = Arc::new(Lambertian::new_color(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new_color(albedo));
                    let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_mov(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    let object_len = world.objects.len();
    let world: Arc<dyn Hittable + Send + Sync> =
        Arc::new(BVHNode::new(&mut world.objects, 0, object_len));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::new_color(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    let material = Arc::new(Lambertian::new(checker.clone()));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let object_len = world.objects.len();
    let world: Arc<dyn Hittable + Send + Sync> =
        Arc::new(BVHNode::new(&mut world.objects, 0, object_len));
    cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::zero(), 2.0, earth_surface));
    let world = HittableList::new(globe);
    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);
    cam.render(&world);
}

fn perlin_spheres() {
    let mut world = HittableList::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    let m = Arc::new(Lambertian::new(pertext));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        m.clone(),
    )));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, m)));
    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);
    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::default();

    let left_red = Arc::new(Lambertian::new_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);
    cam.render(&world);
}

pub fn simple_light() {
    let mut world = HittableList::default();
    // Perlin noise texture
    let pertext = Arc::new(NoiseTexture::new(4.0));
    // ground sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    // small sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    // light
    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::zero();
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 554.0, 227.0),
        Vec3::new(130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1: Arc<dyn Hittable> = Arc::new(Quad::box3d(
        &Point3::zero(),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1: Arc<dyn Hittable> = Arc::new(RotateY::new(box1, 15.0));
    let box1: Arc<dyn Hittable> = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2: Arc<dyn Hittable> = Arc::new(Quad::box3d(
        &Point3::zero(),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2: Arc<dyn Hittable> = Arc::new(RotateY::new(box2, -18.0));
    let box2: Arc<dyn Hittable> = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 64;
    cam.max_depth = 50;
    cam.background = Color::zero();
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);
    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::default();
    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // box1
    let box1: Arc<dyn Hittable> = Arc::new(Quad::box3d(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));

    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    // box2
    let box2: Arc<dyn Hittable> = Arc::new(Quad::box3d(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::new_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::new_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::zero();
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);
    cam.render(&world);
}
pub fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut boxes1 = HittableList::default();

    let ground = Arc::new(Lambertian::new_color(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Quad::box3d(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::default();

    // BVH
    let object_len = boxes1.objects.len();
    let bvh1: Arc<dyn Hittable> = Arc::new(BVHNode::new(&mut boxes1.objects, 0, object_len));
    world.add(bvh1);

    // light
    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    // moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_mov(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    // glass sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    // metal sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // blue volume
    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // global fog
    let boundary = Arc::new(Sphere::new(
        Point3::zero(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // earth
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_material = Arc::new(Lambertian::new(earth_texture));

    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));

    // noise sphere
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    // 1000 small spheres
    let mut boxes2 = HittableList::default();

    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));

    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    let object_len = boxes2.objects.len();
    let bvh2: Arc<dyn Hittable + Send + Sync> =
        Arc::new(BVHNode::new(&mut boxes2.objects, 0, object_len));
    let rotated = Arc::new(RotateY::new(bvh2, 15.0));
    let translated = Arc::new(Translate::new(rotated, Vec3::new(-100.0, 270.0, 395.0)));
    world.add(translated);

    // camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::zero();
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);

    cam.render(&world);
}
fn model() {
    let material = Arc::new(Lambertian::new_color(Color::new(0.7, 0.1, 0.1)));
    let mut world = load("models/Heart.obj", material);
    let light_material = Arc::new(DiffuseLight::new_color(Color::new(8.0, 8.0, 8.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new(60.0, 60.0, -5.0),
        40.0,
        light_material,
    )));
    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(BVHNode::from_list(&mut world.objects));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 1500;
    cam.max_depth = 60;
    cam.background = Color::new(0.05, 0.05, 0.05);
    let center = Point3::new(174.7, 130.06, -1.75);
    cam.lookat = center;
    cam.lookfrom = Point3::new(54.7, 120.06, -165.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.vfov = 60.0;
    cam.render(&world);
}
fn main() {
    let scene = 10;

    match scene {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        10 => model(),
        _ => final_scene(400, 250, 4),
    }
}
