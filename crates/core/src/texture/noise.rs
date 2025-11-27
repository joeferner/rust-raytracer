use crate::{Color, Random, Vector3, texture::Texture};

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(random: &dyn Random) -> Self {
        Self {
            noise: Perlin::new(random),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, pt: Vector3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(pt)
    }
}

#[derive(Debug)]
pub struct Perlin {
    rand_float: [f64; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(random: &dyn Random) -> Self {
        let mut rand_float: [f64; Perlin::POINT_COUNT] = [0.0; Perlin::POINT_COUNT];
        for item in rand_float.iter_mut() {
            *item = random.rand();
        }

        let perm_x = Perlin::generate_perm(random);
        let perm_y = Perlin::generate_perm(random);
        let perm_z = Perlin::generate_perm(random);

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, pt: Vector3) -> f64 {
        let u = pt.x - pt.x.floor();
        let v = pt.y - pt.y.floor();
        let w = pt.z - pt.z.floor();

        // Hermitian Smoothing
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = pt.x.floor() as isize;
        let j = pt.y.floor() as isize;
        let k = pt.z.floor() as isize;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.rand_float[i];
                }
            }
        }

        Perlin::trilinear_interpolation(c, u, v, w)
    }

    fn trilinear_interpolation(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;
        for (i, item) in c.iter().enumerate() {
            for (j, item) in item.iter().enumerate() {
                for (k, item) in item.iter().enumerate() {
                    acc += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * item;
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
