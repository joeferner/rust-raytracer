use core::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Axis, Random};

/// A 3-dimensional vector with x, y, and z components.
///
/// This struct is commonly used for representing points or directions in 3D space.
///
/// # Examples
///
/// ```
/// use caustic_core::Vector3;
///
/// let v = Vector3::new(1.0, 2.0, 3.0);
/// let length = v.length();
/// let unit_vector = v.unit();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// A constant zero vector (0, 0, 0).
    pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);

    /// Creates a new Vector3 with the given x, y, and z components.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let v = Vector3::new(1.0, 2.0, 3.0);
    /// assert_eq_float!(v.x, 1.0);
    /// assert_eq_float!(v.y, 2.0);
    /// assert_eq_float!(v.z, 3.0);
    /// ```
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    /// Generates a random vector with components in the range [0, 1).
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    pub fn random(random: &dyn Random) -> Self {
        Vector3::new(random.rand(), random.rand(), random.rand())
    }

    /// Generates a random vector with components in the specified range [min, max).
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    /// * `min` - The minimum value for each component, inclusive.
    /// * `max` - The maximum value for each component, exclusive.
    pub fn random_interval(random: &dyn Random, min: f64, max: f64) -> Self {
        Vector3::new(
            random.rand_interval(min, max),
            random.rand_interval(min, max),
            random.rand_interval(min, max),
        )
    }

    /// Generates a random unit vector (length = 1) with uniform distribution
    /// on the unit sphere.
    ///
    /// Uses rejection sampling to ensure uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    pub fn random_unit(random: &dyn Random) -> Self {
        loop {
            let p = Self::random_interval(random, -1.0, 1.0);
            let len_sq = p.length_squared();
            if 1e-160 < len_sq && len_sq <= 1.0 {
                return p / len_sq.sqrt();
            }
        }
    }

    /// Generates a random vector on the hemisphere oriented around the given normal.
    ///
    /// The returned vector will be in the same hemisphere as the normal vector,
    /// useful for sampling light directions.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    /// * `normal` - The normal vector defining the hemisphere orientation.
    pub fn random_on_hemisphere(random: &dyn Random, normal: Vector3) -> Self {
        let on_unit_sphere = Self::random_unit(random);
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Generates a random point within the unit disk (circle of radius 1) in the XY plane.
    ///
    /// Uses rejection sampling. The z component is always 0.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    pub fn random_in_unit_disk(random: &dyn Random) -> Vector3 {
        loop {
            let pt = Vector3::new(
                random.rand_interval(-1.0, 1.0),
                random.rand_interval(-1.0, 1.0),
                0.0,
            );
            if pt.length_squared() < 1.0 {
                return pt;
            }
        }
    }

    /// Generates a random direction using cosine-weighted hemisphere sampling.
    ///
    /// This is useful for importance sampling, where the probability density
    /// is proportional to the cosine of the angle from the normal.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    pub fn random_cosine_direction(random: &dyn Random) -> Vector3 {
        let r1 = random.rand();
        let r2 = random.rand();

        let phi = 2.0 * f64::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        let z = (1.0 - r2).sqrt();

        Vector3::new(x, y, z)
    }

    /// Returns a vector to a random point in the [-0.5, -0.5] to [+0.5, +0.5] unit square.
    ///
    /// The z component is always 0. Useful for antialiasing and depth of field effects.
    ///
    /// # Arguments
    ///
    /// * `random` - A random number generator implementing the Random trait.
    pub fn sample_square(random: &dyn Random) -> Vector3 {
        Vector3::new(random.rand() - 0.5, random.rand() - 0.5, 0.0)
    }

    /// Returns the length (magnitude) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let v = Vector3::new(3.0, 4.0, 0.0);
    /// assert_eq_float!(v.length(), 5.0);
    /// ```
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the squared length of the vector.
    ///
    /// This is more efficient than `length()` when you only need to compare
    /// lengths or when the actual length value isn't needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let v = Vector3::new(3.0, 4.0, 0.0);
    /// assert_eq_float!(v.length_squared(), 25.0);
    /// ```
    pub fn length_squared(&self) -> f64 {
        let x_squared = self.x * self.x;
        let y_squared = self.y * self.y;
        let z_squared = self.z * self.z;
        x_squared + y_squared + z_squared
    }

    /// Computes the dot product (scalar product) of this vector with another.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let v1 = Vector3::new(1.0, 2.0, 3.0);
    /// let v2 = Vector3::new(4.0, 5.0, 6.0);
    /// assert_eq_float!(v1.dot(&v2), 32.0);
    /// ```
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product (vector product) of this vector with another.
    ///
    /// The resulting vector is perpendicular to both input vectors.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let v1 = Vector3::new(1.0, 0.0, 0.0);
    /// let v2 = Vector3::new(0.0, 1.0, 0.0);
    /// let result = v1.cross(&v2);
    /// assert_eq_float!(result.z, 1.0);
    /// ```
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the unit vector (normalized vector) in the same direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    ///
    /// let v = Vector3::new(3.0, 4.0, 0.0);
    /// let unit = v.unit();
    /// assert!((unit.length() - 1.0).abs() < 1e-10);
    /// ```
    pub fn unit(&self) -> Vector3 {
        *self / self.length()
    }

    /// Returns true if the vector is close to zero in all dimensions.
    ///
    /// Uses a threshold of 1e-8 for each component.
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    ///
    /// let v1 = Vector3::new(1e-9, 1e-9, 1e-9);
    /// assert!(v1.is_near_zero());
    ///
    /// let v2 = Vector3::new(0.1, 0.0, 0.0);
    /// assert!(!v2.is_near_zero());
    /// ```
    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    /// Reflects this vector about the given normal vector.
    ///
    /// # Arguments
    ///
    /// * `n` - The normal vector to reflect about (should be normalized).
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::Vector3;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let v = Vector3::new(1.0, -1.0, 0.0);
    /// let n = Vector3::new(0.0, 1.0, 0.0);
    /// let reflected = v.reflect(n);
    /// assert_eq_float!(reflected.y, 1.0);
    /// ```
    pub fn reflect(&self, n: Vector3) -> Vector3 {
        *self - 2.0 * self.dot(&n) * n
    }

    /// Refracts this vector through a surface with the given normal and refractive index ratio.
    ///
    /// # Arguments
    ///
    /// * `n` - The normal vector of the surface (should be normalized).
    /// * `etai_over_etat` - The ratio of refractive indices (incident/transmitted).
    pub fn refract(&self, n: Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = (-(*self)).dot(&n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
        r_out_perp + r_out_parallel
    }

    /// Returns the value of the component along the specified axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis (X, Y, or Z) to retrieve.
    pub fn axis_value(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    /// Returns a mutable reference to the component along the specified axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis (X, Y, or Z) to retrieve.
    pub fn axis_value_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
