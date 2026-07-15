use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let n = Vec3::cross(&u, &v);
        let normal = Vec3::unit_vector(&n);
        let d = Vec3::dot(&normal, &q);
        let w = n / Vec3::dot(&n, &n);
        let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
        let bbox = Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2);
        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
        }
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(&self.normal, r.direction());
        // ray parallel to plane
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.d - Vec3::dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));
        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
