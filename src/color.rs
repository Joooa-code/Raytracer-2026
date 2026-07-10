use crate::vec3::Vec3;
use image::Rgb;
use crate::interval::Interval;
pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> Rgb<u8> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.000,0.999);
    let red = (256.0 * intensity.clamp(r)) as u8;
    let green = (256.0 * intensity.clamp(g)) as u8;
    let blue = (256.0 * intensity.clamp(b)) as u8;

    Rgb([red, green, blue])
}
