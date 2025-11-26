use crate::{Axis, Interval, Ray, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct AxisAlignedBoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AxisAlignedBoundingBox {
    pub fn new() -> Self {
        Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }

    /// Treat the two points a and b as extrema for the bounding box, so we don't require a
    /// particular minimum/maximum coordinate order.
    pub fn new_from_points(a: Vector3, b: Vector3) -> Self {
        Self {
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
        }
    }

    pub fn new_from_bbox(box1: AxisAlignedBoundingBox, box2: AxisAlignedBoundingBox) -> Self {
        Self {
            x: Interval::new_from_intervals(box1.x, box2.x),
            y: Interval::new_from_intervals(box1.y, box2.y),
            z: Interval::new_from_intervals(box1.z, box2.z),
        }
    }

    pub fn axis_interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

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

    /// Returns the index of the longest axis of the bounding box.
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
}

impl Default for AxisAlignedBoundingBox {
    fn default() -> Self {
        Self::new()
    }
}
