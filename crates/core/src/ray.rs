use crate::vector::Vector3;

/// Represents a ray in 3D space with an origin point, direction vector, and time.
///
/// A ray is defined by the parametric equation: P(t) = origin + t * direction,
/// where t is a scalar parameter. The time field is used for motion blur.
///
/// # Examples
///
/// ```
/// use caustic_core::{Ray,Vector3};
///
/// // Create a ray starting at the origin, pointing along the x-axis
/// let ray = Ray::new(
///     Vector3::new(0.0, 0.0, 0.0),
///     Vector3::new(1.0, 0.0, 0.0)
/// );
///
/// // Get a point along the ray at t=2.0
/// let point = ray.at(2.0);
/// assert_eq!(point, Vector3::new(2.0, 0.0, 0.0));
/// ```
#[derive(Debug)]
pub struct Ray {
    /// The starting point of the ray
    pub origin: Vector3,

    /// The direction vector of the ray (not necessarily unit length)
    pub direction: Vector3,

    /// The time at which this ray exists (for motion blur)
    pub time: f64,
}

impl Ray {
    /// Creates a new ray with the given origin and direction.
    ///
    /// The time is set to 0.0 by default.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the ray
    /// * `direction` - The direction vector of the ray
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::{Ray,Vector3};
    ///
    /// let ray = Ray::new(
    ///     Vector3::new(1.0, 2.0, 3.0),
    ///     Vector3::new(0.0, 1.0, 0.0)
    /// );
    /// assert_eq!(ray.time, 0.0);
    /// ```
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    /// Creates a new ray with the given origin, direction, and time.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the ray
    /// * `direction` - The direction vector of the ray
    /// * `time` - The time at which this ray exists
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::{Ray,Vector3};
    ///
    /// let ray = Ray::new_with_time(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1.0, 0.0, 0.0),
    ///     0.5
    /// );
    /// assert_eq!(ray.time, 0.5);
    /// ```
    pub fn new_with_time(origin: Vector3, direction: Vector3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    /// Returns the point along the ray at parameter t.
    ///
    /// Computes P(t) = origin + t * direction.
    ///
    /// # Arguments
    ///
    /// * `t` - The parameter value along the ray
    ///
    /// # Examples
    ///
    /// ```
    /// use caustic_core::{Ray,Vector3};
    ///
    /// let ray = Ray::new(
    ///     Vector3::new(1.0, 0.0, 0.0),
    ///     Vector3::new(0.0, 1.0, 0.0)
    /// );
    ///
    /// let point = ray.at(3.0);
    /// assert_eq!(point, Vector3::new(1.0, 3.0, 0.0));
    /// ```
    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (t * self.direction)
    }
}
