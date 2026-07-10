use rand::Rng;
use std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
// generate [0.0, 1.0)
pub fn random_f64() -> f64 {
    rand::rng().random::<f64>()
}
#[allow(dead_code)]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
