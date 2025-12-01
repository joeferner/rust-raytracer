use std::ops::Add;

use crate::{Axis, Interval, Ray, Vector3};

/// An axis-aligned bounding box (AABB) in 3D space.
///
/// An AABB is defined by three intervals along the x, y, and z axes. It represents
/// the smallest box aligned with the coordinate axes that can contain a given object
/// or set of objects. AABBs are commonly used for efficient intersection testing and
/// spatial partitioning.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::{AxisAlignedBoundingBox, Vector3};
///
/// // Create a bounding box from two corner points
/// let min_point = Vector3::new(0.0, 0.0, 0.0);
/// let max_point = Vector3::new(1.0, 1.0, 1.0);
/// let bbox = AxisAlignedBoundingBox::new_from_points(min_point, max_point);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AxisAlignedBoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AxisAlignedBoundingBox {
    /// Creates an empty AABB with all intervals set to empty.
    ///
    /// The resulting bounding box is padded to ensure a minimum size along each axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::AxisAlignedBoundingBox;
    ///
    /// let bbox = AxisAlignedBoundingBox::new();
    /// ```
    pub fn new() -> Self {
        AxisAlignedBoundingBox::pad_to_minimums(Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        })
    }

    /// Creates an AABB from two corner points.
    ///
    /// The points `a` and `b` are treated as opposite corners of the bounding box.
    /// The order doesn't matter - the constructor will determine the minimum and
    /// maximum extents automatically.
    ///
    /// # Arguments
    ///
    /// * `a` - First corner point
    /// * `b` - Opposite corner point
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{AxisAlignedBoundingBox, Vector3};
    ///
    /// let bbox = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(2.0, 2.0, 2.0)
    /// );
    /// ```
    pub fn new_from_points(a: Vector3, b: Vector3) -> Self {
        AxisAlignedBoundingBox::pad_to_minimums(Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        })
    }

    /// Creates an AABB that encloses two existing bounding boxes.
    ///
    /// The resulting AABB is the smallest box that completely contains both
    /// input boxes.
    ///
    /// # Arguments
    ///
    /// * `box1` - First bounding box
    /// * `box2` - Second bounding box
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{AxisAlignedBoundingBox, Vector3};
    ///
    /// let bbox1 = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1.0, 1.0, 1.0)
    /// );
    /// let bbox2 = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.5, 0.5, 0.5),
    ///     Vector3::new(2.0, 2.0, 2.0)
    /// );
    /// let combined = AxisAlignedBoundingBox::new_from_bbox(bbox1, bbox2);
    /// ```
    pub fn new_from_bbox(box1: AxisAlignedBoundingBox, box2: AxisAlignedBoundingBox) -> Self {
        AxisAlignedBoundingBox::pad_to_minimums(Self {
            x: Interval::new_from_intervals(box1.x, box2.x),
            y: Interval::new_from_intervals(box1.y, box2.y),
            z: Interval::new_from_intervals(box1.z, box2.z),
        })
    }

    /// Returns the interval for the specified axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis to query (X, Y, or Z)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{AxisAlignedBoundingBox, Axis, Vector3};
    ///
    /// let bbox = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1.0, 2.0, 3.0)
    /// );
    /// let x_interval = bbox.axis_interval(Axis::X);
    /// ```
    pub fn axis_interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    /// Tests whether a ray intersects this bounding box within the given interval.
    ///
    /// This uses the slab method for ray-box intersection testing, which is efficient
    /// and numerically stable. The method checks if the ray intersects the box within
    /// the parameter range specified by `ray_t`.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection
    /// * `ray_t` - The valid parameter interval for the ray (typically representing time/distance)
    ///
    /// # Returns
    ///
    /// `true` if the ray intersects the box within the specified interval, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{AxisAlignedBoundingBox, Ray, Vector3, Interval};
    ///
    /// let bbox = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1.0, 1.0, 1.0)
    /// );
    /// let ray = Ray::new(Vector3::new(-1.0, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0));
    /// let hits = bbox.hit(&ray, Interval::new(0.0, f64::INFINITY));
    /// assert!(hits);
    /// ```
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let ray_orig = ray.origin;
        let ray_dir = ray.direction;
        let mut ray_t = ray_t;

        for axis in Axis::iter() {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.axis_value(axis);

            let t0 = (ax.min - ray_orig.axis_value(axis)) * adinv;
            let t1 = (ax.max - ray_orig.axis_value(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    /// Returns the axis along which the bounding box is longest.
    ///
    /// This is useful for spatial partitioning algorithms like BVH construction,
    /// where splitting along the longest axis often produces better results.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{AxisAlignedBoundingBox, Axis, Vector3};
    ///
    /// let bbox = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(5.0, 2.0, 3.0)
    /// );
    /// assert_eq!(bbox.longest_axis(), Axis::X);
    /// ```
    pub fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                Axis::X
            } else {
                Axis::Z
            }
        } else if self.y.size() > self.z.size() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    /// Adjusts the AABB to ensure no dimension is narrower than a minimum threshold.
    ///
    /// This prevents degenerate bounding boxes (like infinitely thin planes) from
    /// causing numerical issues in intersection tests. If any dimension is smaller
    /// than `delta` (0.0001), it's expanded symmetrically.
    ///
    /// # Arguments
    ///
    /// * `aabb` - The bounding box to pad
    ///
    /// # Returns
    ///
    /// A new AABB with minimum dimensions enforced
    fn pad_to_minimums(mut aabb: AxisAlignedBoundingBox) -> AxisAlignedBoundingBox {
        let delta = 0.0001;
        if aabb.x.size() < delta {
            aabb.x = aabb.x.expand(delta);
        }
        if aabb.y.size() < delta {
            aabb.y = aabb.y.expand(delta);
        }
        if aabb.z.size() < delta {
            aabb.z = aabb.z.expand(delta);
        }
        aabb
    }
}

impl Add<Vector3> for AxisAlignedBoundingBox {
    type Output = Self;

    /// Translates the bounding box by a vector offset.
    ///
    /// This shifts the entire bounding box in space without changing its size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_raytracer_core::{AxisAlignedBoundingBox, Vector3};
    ///
    /// let bbox = AxisAlignedBoundingBox::new_from_points(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1.0, 1.0, 1.0)
    /// );
    /// let offset = Vector3::new(5.0, 5.0, 5.0);
    /// let translated = bbox + offset;
    /// ```
    fn add(self, rhs: Vector3) -> Self::Output {
        AxisAlignedBoundingBox {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Default for AxisAlignedBoundingBox {
    fn default() -> Self {
        Self::new()
    }
}
