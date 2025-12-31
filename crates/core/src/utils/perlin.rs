use crate::{Random, Vector3};

/// Perlin noise generator for creating smooth, pseudo-random gradients.
///
/// Perlin noise is a type of gradient noise commonly used in procedural texture generation,
/// terrain generation, and other applications requiring natural-looking randomness.
/// This implementation uses Ken Perlin's improved noise algorithm with trilinear interpolation.
///
/// # Examples
///
/// ```
/// use caustic_core::{utils::Perlin, Vector3, Random, random_new};
///
/// let random = random_new();
/// let perlin = Perlin::new(&*random);
///
/// // Generate noise value at a point
/// let point = Vector3::new(1.5, 2.3, 0.7);
/// let noise_value = perlin.noise(point);
/// assert!(noise_value >= -1.0 && noise_value <= 1.0);
///
/// // Generate turbulence (fractal noise)
/// let turbulence_value = perlin.turbulence(point, 7);
/// assert!(turbulence_value >= 0.0);
/// ```
#[derive(Debug)]
pub struct Perlin {
    /// Random unit vectors at lattice points for gradient noise
    rand_vec: [Vector3; Perlin::POINT_COUNT],
    /// Permutation table for x-coordinates
    perm_x: [usize; Perlin::POINT_COUNT],
    /// Permutation table for y-coordinates
    perm_y: [usize; Perlin::POINT_COUNT],
    /// Permutation table for z-coordinates
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    /// Number of lattice points in the permutation tables
    const POINT_COUNT: usize = 256;

    /// Creates a new Perlin noise generator with random gradients and permutation tables.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the `Random` trait
    ///
    /// # Returns
    ///
    /// A new `Perlin` instance initialized with random gradients and permutations
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::{utils::Perlin, Random, random_new};
    ///
    /// let random = random_new();
    /// let perlin = Perlin::new(&*random);
    /// ```
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

    /// Computes the Perlin noise value at a given 3D point.
    ///
    /// The noise function returns smooth, continuous values that vary pseudo-randomly
    /// across 3D space. The output is approximately in the range [-1, 1], though values
    /// can occasionally exceed these bounds slightly.
    ///
    /// # Arguments
    ///
    /// * `pt` - The 3D point at which to evaluate the noise function
    ///
    /// # Returns
    ///
    /// A noise value approximately in the range [-1, 1]
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::{utils::Perlin, Vector3, Random, random_new};
    ///
    /// let random = random_new();
    /// let perlin = Perlin::new(&*random);
    /// let value = perlin.noise(Vector3::new(1.0, 2.0, 3.0));
    /// ```
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

    /// Computes turbulence at a given point using fractal Brownian motion.
    ///
    /// Turbulence is created by summing multiple octaves of Perlin noise at different
    /// frequencies and amplitudes. This produces a more complex, fractal-like pattern
    /// useful for marble textures, clouds, and other natural phenomena.
    ///
    /// # Arguments
    ///
    /// * `pt` - The 3D point at which to evaluate turbulence
    /// * `depth` - The number of octaves to sum (typically 5-7 for good results)
    ///
    /// # Returns
    ///
    /// A turbulence value (always non-negative due to absolute value)
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::{utils::Perlin, Vector3, Random, random_new};
    ///
    /// let random = random_new();
    /// let perlin = Perlin::new(&*random);
    /// let turbulence = perlin.turbulence(Vector3::new(1.0, 2.0, 3.0), 7);
    /// assert!(turbulence >= 0.0);
    /// ```
    pub fn turbulence(&self, pt: Vector3, depth: u32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = pt;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        acc.abs()
    }

    /// Performs trilinear interpolation with Hermite smoothing on a 2x2x2 cube of gradient vectors.
    ///
    /// This is the core of Perlin noise, using dot products between random gradients and
    /// distance vectors to create smooth, continuous noise. The Hermite curve (3t² - 2t³)
    /// is used for smoothing to eliminate directional artifacts.
    ///
    /// # Arguments
    ///
    /// * `c` - A 2x2x2 array of gradient vectors at the corners of the unit cube
    /// * `u` - Fractional position in x-direction [0, 1]
    /// * `v` - Fractional position in y-direction [0, 1]
    /// * `w` - Fractional position in z-direction [0, 1]
    ///
    /// # Returns
    ///
    /// The interpolated noise value
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

    /// Generates a random permutation table for hashing coordinates to gradient indices.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the `Random` trait
    ///
    /// # Returns
    ///
    /// An array containing a permutation of indices 0..255
    fn generate_perm(random: &dyn Random) -> [usize; Perlin::POINT_COUNT] {
        let mut p: [usize; Perlin::POINT_COUNT] = [0; Perlin::POINT_COUNT];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }

        Perlin::permute(random, &mut p);
        p
    }

    /// Randomly permutes an array using the Fisher-Yates shuffle algorithm.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the `Random` trait
    /// * `p` - The array to permute in-place
    fn permute(random: &dyn Random, p: &mut [usize; Perlin::POINT_COUNT]) {
        for i in (1..Perlin::POINT_COUNT - 1).rev() {
            let target = random.rand_int_interval(0, i as i64) as usize;
            p.swap(i, target);
        }
    }
}
