use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    // Stationary Sphere
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::from(static_center, Vec3::new(0.0, 0.0, 0.0), 0.0),
            radius: radius.max(0.0),
            mat,
        }
    }
    // Moving Sphere
    pub fn new_mov(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::from(center1, center2 - center1, 0.0),
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
