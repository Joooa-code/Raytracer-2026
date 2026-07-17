use crate::aabb::Aabb;
use crate::hittable::*;
use crate::interval::Interval;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;

pub struct Triangle {
    pub v0: Point3,
    pub v1: Point3,
    pub v2: Point3,
    pub normal: Vec3,
    pub bbox: Aabb,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: Arc<dyn Material>) -> Self {
        let normal = Vec3::unit_vector(&Vec3::cross(&(v1 - v0), &(v2 - v0)));
        let box1 = Aabb::from_points(v0, v1);
        let box2 = Aabb::from_points(v1, v2);
        let bbox = Aabb::from_boxes(&box1, &box2);
        Self {
            v0,
            v1,
            v2,
            normal,
            mat,
            bbox,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = Vec3::cross(r.direction(), &edge2);
        let a = Vec3::dot(&edge1, &h);

        if a.abs() < 1e-8 {
            return false; // Ray is parallel to the triangle
        }

        let f = 1.0 / a;
        let s = *r.origin() - self.v0;
        let u = f * Vec3::dot(&s, &h);

        if u < 0.0 || u > 1.0 {
            return false; // Intersection is outside the triangle
        }

        let q = Vec3::cross(&s, &edge1);
        let v = f * Vec3::dot(r.direction(), &q);

        if v < 0.0 || u + v > 1.0 {
            return false; // Intersection is outside the triangle
        }

        let t = f * Vec3::dot(&edge2, &q);

        if !ray_t.contains(t) {
            return false; // Intersection is outside the valid range
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);
        rec.u = u;
        rec.v = v;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
