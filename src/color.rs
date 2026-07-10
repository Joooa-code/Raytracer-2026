use crate::interval::Interval;
use crate::vec3::Vec3;
use image::Rgb;
pub type Color = Vec3;

impl Color {
    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        0.0
    }
}

pub fn write_color(pixel_color: &Color) -> Rgb<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = Color::linear_to_gamma(r);
    g = Color::linear_to_gamma(g);
    b = Color::linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let red = (256.0 * intensity.clamp(r)) as u8;
    let green = (256.0 * intensity.clamp(g)) as u8;
    let blue = (256.0 * intensity.clamp(b)) as u8;

    Rgb([red, green, blue])
}
