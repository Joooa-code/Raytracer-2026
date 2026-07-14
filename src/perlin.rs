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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.randfloat[index as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
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

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_u = if i == 1 { u } else { 1.0 - u };
                    let weight_v = if j == 1 { v } else { 1.0 - v };
                    let weight_w = if k == 1 { w } else { 1.0 - w };
                    accum += weight_u * weight_v * weight_w * c[i][j][k];
                }
            }
        }
        accum
    }
}
