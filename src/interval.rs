use crate::rtweekend::INFINITY;
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    #[allow(dead_code)]
    pub const fn size(&self) -> f64 {
        self.max - self.min
    }
    #[allow(dead_code)]
    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn empty() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
    #[allow(dead_code)]
    pub fn universe() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    #[allow(dead_code)]
    pub fn expand(&self, delta: f64) -> Interval {
        let padding: f64 = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}
