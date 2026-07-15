use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    #[allow(dead_code)]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut bbox = Self { x, y, z };
        bbox.pad_to_minimums();
        bbox
    }
    // empty
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }
    pub fn from_points(a: Point3, b: Point3) -> Self {
        let mut bbox = Self {
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())),

            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())),

            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        };
        bbox.pad_to_minimums();
        bbox
    }
    pub fn from_boxes(box0: &Aabb, box1: &Aabb) -> Self {
        Self {
            x: Interval::from_intervals(&box0.x, &box1.x),
            y: Interval::from_intervals(&box0.y, &box1.y),
            z: Interval::from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];
            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}
