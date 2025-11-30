use core::f64;
use std::ops::Add;

/// A one-dimensional interval representing a continuous range [min, max].
///
/// An `Interval` defines a closed range of floating-point values from `min` to `max`.
/// It provides utilities for checking containment, expanding ranges, and performing
/// arithmetic operations.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Interval;
///
/// let interval = Interval::new(0.0, 10.0);
/// assert!(interval.contains(5.0));
/// assert!(!interval.contains(15.0));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    /// The minimum value of the interval (inclusive)
    pub min: f64,
    /// The maximum value of the interval (inclusive)
    pub max: f64,
}

impl Interval {
    /// An empty interval where min > max.
    ///
    /// This represents an invalid or empty range that contains no values.
    pub const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);

    /// An interval spanning all possible values from negative to positive infinity.
    pub const UNIVERSE: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);

    /// Creates a new interval with the specified minimum and maximum values.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value (inclusive)
    /// * `max` - The maximum value (inclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// let interval = Interval::new(1.0, 5.0);
    /// assert_eq!(interval.min, 1.0);
    /// assert_eq!(interval.max, 5.0);
    /// ```
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Creates a new interval that encompasses two existing intervals.
    ///
    /// The resulting interval will have the minimum of both `min` values and
    /// the maximum of both `max` values, effectively creating a bounding interval.
    ///
    /// # Arguments
    ///
    /// * `a` - First interval
    /// * `b` - Second interval
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// let a = Interval::new(0.0, 5.0);
    /// let b = Interval::new(3.0, 8.0);
    /// let combined = Interval::new_from_intervals(a, b);
    /// assert_eq!(combined.min, 0.0);
    /// assert_eq!(combined.max, 8.0);
    /// ```
    pub const fn new_from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    /// Checks if a value is contained within the interval (inclusive).
    ///
    /// Returns `true` if `min <= x <= max`.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert!(interval.contains(5.0));
    /// assert!(interval.contains(0.0));  // boundaries included
    /// assert!(interval.contains(10.0));
    /// assert!(!interval.contains(-1.0));
    /// ```
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if a value is strictly inside the interval (exclusive).
    ///
    /// Returns `true` if `min < x < max`.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// let interval = Interval::new(0.0, 10.0);
    /// assert!(interval.surrounds(5.0));
    /// assert!(!interval.surrounds(0.0));  // boundaries excluded
    /// assert!(!interval.surrounds(10.0));
    /// ```
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Creates a new interval expanded by the specified delta.
    ///
    /// The expansion is symmetric: half of `delta` is subtracted from `min`
    /// and half is added to `max`.
    ///
    /// # Arguments
    ///
    /// * `delta` - The total amount to expand the interval
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// let interval = Interval::new(5.0, 15.0);
    /// let expanded = interval.expand(4.0);
    /// assert_eq!(expanded.min, 3.0);  // 5.0 - 2.0
    /// assert_eq!(expanded.max, 17.0); // 15.0 + 2.0
    /// ```
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    /// Returns the size (width) of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// let interval = Interval::new(5.0, 15.0);
    /// assert_eq!(interval.size(), 10.0);
    /// ```
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if the interval is empty.
    ///
    /// An interval is considered empty if `max <= min`, meaning it contains no values.
    /// This occurs when the interval is invalid or represents an empty range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::Interval;
    ///
    /// // Empty interval where max < min
    /// let empty = Interval::new(5.0, 3.0);
    /// assert!(empty.is_empty());
    ///
    /// // The EMPTY constant is empty
    /// assert!(Interval::EMPTY.is_empty());
    ///
    /// // Single-point interval (max == min) is considered empty
    /// let point = Interval::new(5.0, 5.0);
    /// assert!(point.is_empty());
    ///
    /// // Valid interval is not empty
    /// let valid = Interval::new(3.0, 5.0);
    /// assert!(!valid.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.max <= self.min
    }
}

/// Implements addition of a scalar value to an interval.
///
/// Adding a scalar shifts both bounds of the interval by the same amount.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::Interval;
///
/// let interval = Interval::new(0.0, 10.0);
/// let shifted = interval + 5.0;
/// assert_eq!(shifted.min, 5.0);
/// assert_eq!(shifted.max, 15.0);
/// ```
impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interval = Interval::new(1.0, 5.0);
        assert_eq!(interval.min, 1.0);
        assert_eq!(interval.max, 5.0);
    }

    #[test]
    fn test_empty_constant() {
        let empty = Interval::EMPTY;
        assert_eq!(empty.min, f64::INFINITY);
        assert_eq!(empty.max, -f64::INFINITY);
        assert!(!empty.contains(0.0));
        assert!(!empty.contains(f64::INFINITY));
    }

    #[test]
    fn test_universe_constant() {
        let universe = Interval::UNIVERSE;
        assert_eq!(universe.min, -f64::INFINITY);
        assert_eq!(universe.max, f64::INFINITY);
        assert!(universe.contains(0.0));
        assert!(universe.contains(1e100));
        assert!(universe.contains(-1e100));
    }

    #[test]
    fn test_new_from_intervals() {
        let a = Interval::new(0.0, 5.0);
        let b = Interval::new(3.0, 8.0);
        let combined = Interval::new_from_intervals(a, b);

        assert_eq!(combined.min, 0.0);
        assert_eq!(combined.max, 8.0);
    }

    #[test]
    fn test_new_from_intervals_disjoint() {
        let a = Interval::new(0.0, 2.0);
        let b = Interval::new(5.0, 8.0);
        let combined = Interval::new_from_intervals(a, b);

        assert_eq!(combined.min, 0.0);
        assert_eq!(combined.max, 8.0);
    }

    #[test]
    fn test_contains_inside() {
        let interval = Interval::new(0.0, 10.0);
        assert!(interval.contains(5.0));
    }

    #[test]
    fn test_contains_boundaries() {
        let interval = Interval::new(0.0, 10.0);
        assert!(interval.contains(0.0));
        assert!(interval.contains(10.0));
    }

    #[test]
    fn test_contains_outside() {
        let interval = Interval::new(0.0, 10.0);
        assert!(!interval.contains(-1.0));
        assert!(!interval.contains(11.0));
    }

    #[test]
    fn test_surrounds_inside() {
        let interval = Interval::new(0.0, 10.0);
        assert!(interval.surrounds(5.0));
    }

    #[test]
    fn test_surrounds_boundaries() {
        let interval = Interval::new(0.0, 10.0);
        assert!(!interval.surrounds(0.0));
        assert!(!interval.surrounds(10.0));
    }

    #[test]
    fn test_surrounds_outside() {
        let interval = Interval::new(0.0, 10.0);
        assert!(!interval.surrounds(-1.0));
        assert!(!interval.surrounds(11.0));
    }

    #[test]
    fn test_expand() {
        let interval = Interval::new(5.0, 15.0);
        let expanded = interval.expand(4.0);

        assert_eq!(expanded.min, 3.0);
        assert_eq!(expanded.max, 17.0);
    }

    #[test]
    fn test_expand_negative() {
        let interval = Interval::new(5.0, 15.0);
        let contracted = interval.expand(-4.0);

        assert_eq!(contracted.min, 7.0);
        assert_eq!(contracted.max, 13.0);
    }

    #[test]
    fn test_expand_zero() {
        let interval = Interval::new(5.0, 15.0);
        let same = interval.expand(0.0);

        assert_eq!(same.min, 5.0);
        assert_eq!(same.max, 15.0);
    }

    #[test]
    fn test_size() {
        let interval = Interval::new(5.0, 15.0);
        assert_eq!(interval.size(), 10.0);
    }

    #[test]
    fn test_size_zero() {
        let interval = Interval::new(5.0, 5.0);
        assert_eq!(interval.size(), 0.0);
    }

    #[test]
    fn test_size_negative() {
        let interval = Interval::new(10.0, 5.0);
        assert_eq!(interval.size(), -5.0);
    }

    #[test]
    fn test_add_positive() {
        let interval = Interval::new(0.0, 10.0);
        let shifted = interval + 5.0;

        assert_eq!(shifted.min, 5.0);
        assert_eq!(shifted.max, 15.0);
    }

    #[test]
    fn test_add_negative() {
        let interval = Interval::new(10.0, 20.0);
        let shifted = interval + (-5.0);

        assert_eq!(shifted.min, 5.0);
        assert_eq!(shifted.max, 15.0);
    }

    #[test]
    fn test_add_zero() {
        let interval = Interval::new(5.0, 15.0);
        let same = interval + 0.0;

        assert_eq!(same.min, 5.0);
        assert_eq!(same.max, 15.0);
    }

    #[test]
    fn test_clone() {
        let original = Interval::new(1.0, 5.0);
        let cloned = original.clone();

        assert_eq!(cloned.min, original.min);
        assert_eq!(cloned.max, original.max);
    }

    #[test]
    fn test_copy() {
        let original = Interval::new(1.0, 5.0);
        let copied = original;

        assert_eq!(copied.min, 1.0);
        assert_eq!(copied.max, 5.0);
    }
}
