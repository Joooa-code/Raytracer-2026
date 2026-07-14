use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    // Stationary Sphere
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(static_center - rvec, static_center + rvec);
        Self {
            center: Ray::new(static_center, Vec3::zero(), 0.0),
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }
    // Moving Sphere
    pub fn new_mov(center1: Point3, center2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = Aabb::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = Aabb::from_boxes(&box1, &box2);
        Self {
            center,
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }
    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
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
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;
        rec.mat = self.mat.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
