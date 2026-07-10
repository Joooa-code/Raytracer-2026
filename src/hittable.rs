use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::default::Default;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
