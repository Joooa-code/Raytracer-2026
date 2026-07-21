use crate::color;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, degrees_to_radians, random_f64};
use crate::vec3::{Point3, Vec3};
use image::{ImageBuffer, RgbImage};
use std::f64::consts::PI;
use std::sync::Arc;
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene
    pub background: Color,
    pub vfov: f64, // Vertical view angle (field of view)
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus
    image_height: usize,
    pixel_samples_scale: f64,
    sqrt_spp: usize,     // Square root of number of samples per pixel
    recip_sqrt_spp: f64, // 1 / sqrt_spp
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            max_depth: 10,
            background: Color::zero(),
            vfov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            samples_per_pixel: 10,
            sqrt_spp: 3,
            recip_sqrt_spp: 1.0 / 3.0,
            pixel_samples_scale: 0.0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }
}
impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as usize;
        self.pixel_samples_scale = 1.0 / ((self.sqrt_spp * self.sqrt_spp) as f64);
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;
        self.center = self.lookfrom;
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.w = Vec3::unit_vector(&(self.lookfrom - self.lookat));
        self.u = Vec3::unit_vector(&Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * (-self.v);
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &Arc<dyn Hittable + Send + Sync>) {
        let path = std::path::Path::new("output/book3/image4.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        self.initialize();
        let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, si, sj);
                        pixel_color += self.ray_color(&r, self.max_depth, world.as_ref());
                    }
                }
                pixel_color *= self.pixel_samples_scale;
                let pixel = img.get_pixel_mut(i as u32, j as u32);
                *pixel = color::write_color(&pixel_color);
            }
        }
        eprintln!("\rDone.");
        img.save(path).expect("Cannot save the image to the file");
    }

    fn sample_square_stratified(&self, s_i: usize, s_j: usize) -> Vec3 {
        let px = ((s_i as f64 + random_f64()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_f64()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }

    #[allow(dead_code)]
    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn get_ray(&self, i: usize, j: usize, s_i: usize, s_j: usize) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_time = random_f64();
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction, ray_time)
    }
    fn ray_color(&self, r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::zero();
        }
        let mut rec = HitRecord::default();
        if !world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }
        let mut scattered = Ray::default();
        let mut attenuation = Color::zero();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }
        let scattering_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);
        let pdf_value = 1.0 / (2.0 * PI);
        let color_from_scatter =
            (attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world))
                / pdf_value;

        color_from_emission + color_from_scatter
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}
