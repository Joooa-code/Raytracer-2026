use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64, // the exact time for each ray
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Point3::zero(),
            dir: Vec3::zero(),
            tm: 0.0,
        }
    }

    pub fn from(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }

    #[allow(dead_code)]
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}
