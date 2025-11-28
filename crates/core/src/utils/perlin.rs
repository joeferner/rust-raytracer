use crate::{Random, Vector3};

#[derive(Debug)]
pub struct Perlin {
    rand_vec: [Vector3; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(random: &dyn Random) -> Self {
        let mut rand_vec: [Vector3; Perlin::POINT_COUNT] = [Vector3::ZERO; Perlin::POINT_COUNT];
        for item in rand_vec.iter_mut() {
            *item = Vector3::random_unit(random);
        }

        let perm_x = Perlin::generate_perm(random);
        let perm_y = Perlin::generate_perm(random);
        let perm_z = Perlin::generate_perm(random);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, pt: Vector3) -> f64 {
        let u = pt.x - pt.x.floor();
        let v = pt.y - pt.y.floor();
        let w = pt.z - pt.z.floor();

        let i = pt.x.floor() as isize;
        let j = pt.y.floor() as isize;
        let k = pt.z.floor() as isize;

        let mut c: [[[Vector3; 2]; 2]; 2] = [[[Vector3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[i];
                }
            }
        }

        Perlin::trilinear_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, pt: Vector3, depth: u32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = pt;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        return acc.abs();
    }

    fn trilinear_interpolation(c: [[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;
        for (i, item) in c.iter().enumerate() {
            for (j, item) in item.iter().enumerate() {
                for (k, item) in item.iter().enumerate() {
                    let weight_v = Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * item.dot(&weight_v);
                }
            }
        }
        acc
    }

    fn generate_perm(random: &dyn Random) -> [usize; Perlin::POINT_COUNT] {
        let mut p: [usize; Perlin::POINT_COUNT] = [0; Perlin::POINT_COUNT];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }

        Perlin::permute(random, &mut p);
        p
    }

    fn permute(random: &dyn Random, p: &mut [usize; Perlin::POINT_COUNT]) {
        for i in (1..Perlin::POINT_COUNT - 1).rev() {
            let target = random.rand_int_interval(0, i as i64) as usize;
            p.swap(i, target);
        }
    }
}
