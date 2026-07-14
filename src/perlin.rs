use crate::rtweekend::random_i32;
use crate::vec3::{Point3, Vec3};

pub struct Perlin {
    randvec: [Vec3; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vec3::zero(); 256];

        #[allow(clippy::needless_range_loop)]
        for i in 0..256 {
            randvec[i] = Vec3::unit_vector(&Vec3::random_range(-1.0, 1.0));
        }
        let mut perm_x = [0; 256];
        let mut perm_y = [0; 256];
        let mut perm_z = [0; 256];

        Perlin::generate_perm(&mut perm_x);
        Perlin::generate_perm(&mut perm_y);
        Perlin::generate_perm(&mut perm_z);
        Self {
            randvec,
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

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.randvec[index as usize];
                }
            }
        }
        Perlin::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        #[allow(unused_variables)]
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
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

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }
        accum
    }
}
