use crate::Random;
use std::ops::{Add, AddAssign, Div, Mul};

/// Represents an RGB color with floating-point components in the range [0.0, 1.0].
///
/// Each color component (red, green, blue) is stored as an `f64` to enable
/// high-precision color calculations during rendering operations.
/// Values outside the [0.0, 1.0] range are permitted during intermediate calculations,
/// but should be clamped before final output.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Color;
///
/// // Create a custom color
/// let purple = Color::new(0.5, 0.0, 0.5);
///
/// // Use predefined constants
/// let black = Color::BLACK;
/// let white = Color::WHITE;
///
/// // Perform color arithmetic
/// let mixed = purple * 0.5 + white * 0.5;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Red component (typically 0.0 to 1.0)
    pub r: f64,
    /// Green component (typically 0.0 to 1.0)
    pub g: f64,
    /// Blue component (typically 0.0 to 1.0)
    pub b: f64,
}

impl Color {
    /// Pure black color (0, 0, 0)
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

    /// Pure white color (1, 1, 1)
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    /// Creates a new color with the specified RGB components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (typically 0.0 to 1.0)
    /// * `g` - Green component (typically 0.0 to 1.0)
    /// * `b` - Blue component (typically 0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Color;
    ///
    /// let red = Color::new(1.0, 0.0, 0.0);
    /// let cyan = Color::new(0.0, 1.0, 1.0);
    /// ```
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Generates a random color with each component in the range [0.0, 1.0).
    ///
    /// # Arguments
    ///
    /// * `random` - Random number generator implementing the `Random` trait
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{Color, random_new};
    ///
    /// let mut rng = random_new();
    /// let random_color = Color::random(&*rng);
    /// ```
    pub fn random(random: &dyn Random) -> Self {
        Self {
            r: random.rand(),
            g: random.rand(),
            b: random.rand(),
        }
    }

    /// Generates a random color with each component in the specified interval.
    ///
    /// # Arguments
    ///
    /// * `random` - Random number generator implementing the `Random` trait
    /// * `from` - Minimum value for each component (inclusive)
    /// * `to` - Maximum value for each component (exclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{Color, random_new};
    ///
    /// let mut rng = random_new();
    /// // Generate a dark color (components between 0.0 and 0.3)
    /// let dark_color = Color::random_interval(&*rng, 0.0, 0.3);
    /// ```
    pub fn random_interval(random: &dyn Random, from: f64, to: f64) -> Self {
        Self {
            r: random.rand_interval(from, to),
            g: random.rand_interval(from, to),
            b: random.rand_interval(from, to),
        }
    }

    /// Converts linear color space to gamma-corrected color space and clamps to [0.0, 0.999].
    ///
    /// This applies gamma correction using a square root transformation (gamma = 2.0),
    /// which is commonly used to convert linear light intensity values to values suitable
    /// for display on standard monitors.
    ///
    /// Components are clamped to [0.0, 0.999] to ensure they map to valid pixel values
    /// (typically 0-255 for 8-bit color).
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Color;
    ///
    /// let linear = Color::new(0.25, 0.5, 1.0);
    /// let gamma = linear.linear_to_gamma();
    /// // gamma.r ≈ 0.5, gamma.g ≈ 0.707, gamma.b ≈ 0.999
    /// ```
    pub fn linear_to_gamma(&self) -> Self {
        Self {
            r: linear_to_gamma(self.r).clamp(0.0, 0.999),
            g: linear_to_gamma(self.g).clamp(0.0, 0.999),
            b: linear_to_gamma(self.b).clamp(0.0, 0.999),
        }
    }

    /// Replaces any NaN (Not a Number) components with 0.0.
    ///
    /// This is useful for handling edge cases in rendering calculations where
    /// operations like division by zero or square roots of negative numbers
    /// might produce NaN values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Color;
    /// use assert_eq_float::assert_eq_float;
    ///
    /// let invalid = Color::new(1.0, f64::NAN, 0.5);
    /// let valid = invalid.nan_to_zero();
    /// assert_eq_float!(valid.r, 1.0);
    /// assert_eq_float!(valid.g, 0.0);
    /// assert_eq_float!(valid.b, 0.5);
    /// ```
    pub fn nan_to_zero(&self) -> Color {
        Color {
            r: if self.r.is_nan() { 0.0 } else { self.r },
            g: if self.g.is_nan() { 0.0 } else { self.g },
            b: if self.b.is_nan() { 0.0 } else { self.b },
        }
    }
}

/// Converts a linear color component to gamma-corrected space.
///
/// Uses square root (gamma = 2.0) for the transformation. Negative values
/// are clamped to 0.0.
fn linear_to_gamma(v: f64) -> f64 {
    if v > 0.0 { v.sqrt() } else { 0.0 }
}

// Operator Implementations

/// Multiplies each color component by a scalar value.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Color;
/// use assert_eq_float::assert_eq_float;
///
/// let color = Color::new(0.8, 0.4, 0.2);
/// let dimmed = color * 0.5;
/// assert_eq_float!(dimmed.r, 0.4);
/// ```
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

/// Multiplies two colors component-wise.
///
/// This is useful for modulating one color by another, such as applying
/// a surface color to incident light.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Color;
///
/// let surface = Color::new(0.8, 0.0, 0.0); // Red surface
/// let light = Color::new(1.0, 1.0, 1.0);   // White light
/// let result = surface * light;
/// // Result is (0.8, 0.0, 0.0) - red light reflected
/// ```
impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

/// Multiplies a scalar by a color (scalar * color).
///
/// This enables writing expressions like `0.5 * color` in addition to `color * 0.5`.
impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, v: Color) -> Color {
        Color {
            r: self * v.r,
            g: self * v.g,
            b: self * v.b,
        }
    }
}

/// Divides each color component by a scalar value.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Color;
/// use assert_eq_float::assert_eq_float;
///
/// let color = Color::new(0.8, 0.4, 0.2);
/// let averaged = color / 2.0;
/// assert_eq_float!(averaged.r, 0.4);
/// ```
impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

/// Adds two colors component-wise.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Color;
/// use assert_eq_float::assert_eq_float;
///
/// let red = Color::new(1.0, 0.0, 0.0);
/// let green = Color::new(0.0, 1.0, 0.0);
/// let yellow = red + green;
/// assert_eq_float!(yellow.r, 1.0);
/// assert_eq_float!(yellow.g, 1.0);
/// assert_eq_float!(yellow.b, 0.0);
/// ```
impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Color) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

/// Adds a color to this color in place.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Color;
/// use assert_eq_float::assert_eq_float;
///
/// let mut color = Color::new(0.5, 0.2, 0.1);
/// color += Color::new(0.3, 0.4, 0.5);
/// assert_eq_float!(color.r, 0.8);
/// ```
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.r - other.r).abs() < EPSILON
            && (self.g - other.g).abs() < EPSILON
            && (self.b - other.b).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use assert_eq_float::assert_eq_float;

    use crate::random::test::MockRandom;

    use super::*;

    #[test]
    fn test_new() {
        let color = Color::new(0.5, 0.7, 0.9);
        assert_eq_float!(color.r, 0.5);
        assert_eq_float!(color.g, 0.7);
        assert_eq_float!(color.b, 0.9);
    }

    #[test]
    fn test_constants() {
        assert_eq_float!(Color::BLACK.r, 0.0);
        assert_eq_float!(Color::BLACK.g, 0.0);
        assert_eq_float!(Color::BLACK.b, 0.0);

        assert_eq_float!(Color::WHITE.r, 1.0);
        assert_eq_float!(Color::WHITE.g, 1.0);
        assert_eq_float!(Color::WHITE.b, 1.0);
    }

    #[test]
    fn test_random() {
        let rng = MockRandom::new(vec![0.3, 0.5, 0.7]);
        let color = Color::random(&rng);
        assert_eq_float!(color.r, 0.3);
        assert_eq_float!(color.g, 0.5);
        assert_eq_float!(color.b, 0.7);
    }

    #[test]
    fn test_random_interval() {
        let rng = MockRandom::new(vec![0.0, 0.5, 1.0]);
        let color = Color::random_interval(&rng, 10.0, 20.0);
        assert_eq_float!(color.r, 10.0); // 10.0 + 10.0 * 0.0
        assert_eq_float!(color.g, 15.0); // 10.0 + 10.0 * 0.5
        assert_eq_float!(color.b, 20.0); // 10.0 + 10.0 * 1.0
    }

    #[test]
    fn test_linear_to_gamma() {
        let linear = Color::new(0.25, 0.0, 1.0);
        let gamma = linear.linear_to_gamma();
        assert!((gamma.r - 0.5).abs() < 1e-10);
        assert_eq_float!(gamma.g, 0.0);
        assert_eq_float!(gamma.b, 0.999); // Clamped
    }

    #[test]
    fn test_linear_to_gamma_negative() {
        let linear = Color::new(-0.5, 0.0, 0.0);
        let gamma = linear.linear_to_gamma();
        assert_eq_float!(gamma.r, 0.0); // Negative clamped to 0
    }

    #[test]
    fn test_nan_to_zero() {
        let color = Color::new(0.5, f64::NAN, 0.8);
        let cleaned = color.nan_to_zero();
        assert_eq_float!(cleaned.r, 0.5);
        assert_eq_float!(cleaned.g, 0.0);
        assert_eq_float!(cleaned.b, 0.8);
    }

    #[test]
    fn test_nan_to_zero_all_valid() {
        let color = Color::new(0.5, 0.6, 0.7);
        let cleaned = color.nan_to_zero();
        assert_eq_float!(cleaned.r, 0.5);
        assert_eq_float!(cleaned.g, 0.6);
        assert_eq_float!(cleaned.b, 0.7);
    }

    #[test]
    fn test_mul_scalar() {
        let color = Color::new(0.8, 0.4, 0.2);
        let result = color * 0.5;
        assert_eq_float!(result.r, 0.4);
        assert_eq_float!(result.g, 0.2);
        assert_eq_float!(result.b, 0.1);
    }

    #[test]
    fn test_scalar_mul_color() {
        let color = Color::new(0.8, 0.4, 0.2);
        let result = 0.5 * color;
        assert_eq_float!(result.r, 0.4);
        assert_eq_float!(result.g, 0.2);
        assert_eq_float!(result.b, 0.1);
    }

    #[test]
    fn test_mul_color() {
        let c1 = Color::new(0.8, 0.5, 0.2);
        let c2 = Color::new(0.5, 0.4, 1.0);
        let result = c1 * c2;
        assert_eq_float!(result.r, 0.4);
        assert_eq_float!(result.g, 0.2);
        assert_eq_float!(result.b, 0.2);
    }

    #[test]
    fn test_div_scalar() {
        let color = Color::new(0.8, 0.4, 0.2);
        let result = color / 2.0;
        assert_eq_float!(result.r, 0.4);
        assert_eq_float!(result.g, 0.2);
        assert_eq_float!(result.b, 0.1);
    }

    #[test]
    fn test_add() {
        let c1 = Color::new(0.3, 0.2, 0.1);
        let c2 = Color::new(0.5, 0.4, 0.3);
        let result = c1 + c2;
        assert_eq_float!(result.r, 0.8);
        assert_eq_float!(result.g, 0.6);
        assert_eq_float!(result.b, 0.4);
    }

    #[test]
    fn test_add_assign() {
        let mut color = Color::new(0.3, 0.2, 0.1);
        color += Color::new(0.5, 0.4, 0.3);
        assert_eq_float!(color.r, 0.8);
        assert_eq_float!(color.g, 0.6);
        assert_eq_float!(color.b, 0.4);
    }

    #[test]
    fn test_complex_expression() {
        // Test a realistic color blending calculation
        let surface = Color::new(0.8, 0.2, 0.1);
        let light = Color::new(1.0, 0.9, 0.8);
        let ambient = Color::new(0.1, 0.1, 0.15);

        let result = surface * light * 0.8 + ambient * 0.2;

        assert!((result.r - 0.66).abs() < 1e-10);
        assert!((result.g - 0.164).abs() < 1e-10);
        assert!((result.b - 0.094).abs() < 1e-10);
    }

    #[test]
    fn test_clone_and_copy() {
        let c1 = Color::new(0.5, 0.6, 0.7);
        let c2 = c1; // Copy
        let c3 = c1.clone(); // Clone

        assert_eq_float!(c1.r, c2.r);
        assert_eq_float!(c1.r, c3.r);
    }
}
