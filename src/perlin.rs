use crate::rtweekend::{random_f64, random_i32};
use crate::vec3::Point3;

pub struct Perlin {
    randfloat: [f64; 256],

    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.0; 256];

        #[allow(clippy::needless_range_loop)]
        for i in 0..256 {
            randfloat[i] = random_f64();
        }

        let mut perm_x = [0; 256];
        let mut perm_y = [0; 256];
        let mut perm_z = [0; 256];

        Perlin::generate_perm(&mut perm_x);
        Perlin::generate_perm(&mut perm_y);
        Perlin::generate_perm(&mut perm_z);
        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32) & 255;
        let j = ((4.0 * p.y()) as i32) & 255;
        let k = ((4.0 * p.z()) as i32) & 255;

        let index = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];

        self.randfloat[index as usize]
    }

    fn generate_perm(p: &mut [i32; 256]) {
        #[allow(clippy::needless_range_loop)]
        for i in 0..256 {
            p[i] = i as i32;
        }
        Perlin::permute(p, 256);
    }

    fn permute(p: &mut [i32; 256], n: usize) {
        for i in (1..n).rev() {
            let target = random_i32(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
}
