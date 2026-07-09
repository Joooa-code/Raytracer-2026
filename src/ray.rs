use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            orig: Point3::zero(),
            dir: Vec3::zero(),
        }
    }

    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    #[allow(dead_code)]
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
