use core::f64;
use std::sync::Arc;

use crate::{Axis, AxisAlignedBoundingBox, Interval, Node, Ray, Vector3, object::HitRecord};

pub struct RotateY {
    object: Arc<dyn Node>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AxisAlignedBoundingBox,
}

impl RotateY {
    pub fn new(object: Arc<dyn Node>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i * bbox.axis_interval(Axis::X).max
                        + (1.0 - i) * bbox.axis_interval(Axis::X).min;
                    let y = j * bbox.axis_interval(Axis::Y).max
                        + (1.0 - j) * bbox.axis_interval(Axis::Y).min;
                    let z = k * bbox.axis_interval(Axis::Z).max
                        + (1.0 - k) * bbox.axis_interval(Axis::Z).min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vector3::new(new_x, y, new_z);

                    for axis in Axis::iter() {
                        *min.axis_value_mut(axis) =
                            min.axis_value(axis).min(tester.axis_value(axis));
                        *max.axis_value_mut(axis) =
                            max.axis_value(axis).max(tester.axis_value(axis));
                    }
                }
            }
        }

        let bbox = AxisAlignedBoundingBox::new_from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Node for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.
        let rotated_r = {
            let origin = Vector3::new(
                (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
                ray.origin.y,
                (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z),
            );

            let direction = Vector3::new(
                (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
                ray.direction.y,
                (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z),
            );

            let mut rotated_r = Ray::new(origin, direction);
            rotated_r.time = ray.time;
            rotated_r
        };

        // Determine whether an intersection exists in object space (and if so, where).
        let mut hit = self.object.hit(&rotated_r, ray_t)?;

        // Transform the intersection from object space back to world space.
        hit.pt = Vector3::new(
            (self.cos_theta * hit.pt.x) + (self.sin_theta * hit.pt.z),
            hit.pt.y,
            (-self.sin_theta * hit.pt.x) + (self.cos_theta * hit.pt.z),
        );

        hit.normal = Vector3::new(
            (self.cos_theta * hit.normal.x) + (self.sin_theta * hit.normal.z),
            hit.normal.y,
            (-self.sin_theta * hit.normal.x) + (self.cos_theta * hit.normal.z),
        );

        Some(hit)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
