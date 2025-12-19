use core::f64;
use std::sync::Arc;

use crate::{
    Axis, AxisAlignedBoundingBox, Interval, Matrix3x3, Node, Ray, RenderContext, Vector3,
    object::HitRecord,
};

#[derive(Debug)]
pub struct Rotate {
    object: Arc<dyn Node>,
    rotation_matrix: Matrix3x3,
    inverse_rotation_matrix: Matrix3x3,
    bbox: AxisAlignedBoundingBox,
}

impl Rotate {
    /// Creates a rotation around an arbitrary axis
    pub fn new(object: Arc<dyn Node>, axis: Vector3, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // Normalize the axis
        let axis = axis.unit();
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        // Rodrigues' rotation formula to build rotation matrix
        let one_minus_cos = 1.0 - cos_theta;

        let rotation_matrix = Matrix3x3::new([
            [
                cos_theta + x * x * one_minus_cos,
                x * y * one_minus_cos - z * sin_theta,
                x * z * one_minus_cos + y * sin_theta,
            ],
            [
                y * x * one_minus_cos + z * sin_theta,
                cos_theta + y * y * one_minus_cos,
                y * z * one_minus_cos - x * sin_theta,
            ],
            [
                z * x * one_minus_cos - y * sin_theta,
                z * y * one_minus_cos + x * sin_theta,
                cos_theta + z * z * one_minus_cos,
            ],
        ]);

        // The inverse rotation is just the transpose for rotation matrices
        let inverse_rotation_matrix = Matrix3x3::new([
            [
                rotation_matrix[0][0],
                rotation_matrix[1][0],
                rotation_matrix[2][0],
            ],
            [
                rotation_matrix[0][1],
                rotation_matrix[1][1],
                rotation_matrix[2][1],
            ],
            [
                rotation_matrix[0][2],
                rotation_matrix[1][2],
                rotation_matrix[2][2],
            ],
        ]);

        let obj_bbox = object.bounding_box();
        let bbox = Self::compute_bounding_box(obj_bbox, &rotation_matrix);

        Self {
            object,
            rotation_matrix,
            inverse_rotation_matrix,
            bbox,
        }
    }

    /// Helper function to rotate around the X axis
    pub fn rotate_x(object: Arc<dyn Node>, angle: f64) -> Self {
        Self::new(object, Vector3::new(1.0, 0.0, 0.0), angle)
    }

    /// Helper function to rotate around the Y axis
    pub fn rotate_y(object: Arc<dyn Node>, angle: f64) -> Self {
        Self::new(object, Vector3::new(0.0, 1.0, 0.0), angle)
    }

    /// Helper function to rotate around the Z axis
    pub fn rotate_z(object: Arc<dyn Node>, angle: f64) -> Self {
        Self::new(object, Vector3::new(0.0, 0.0, 1.0), angle)
    }

    fn compute_bounding_box(
        original_bbox: &AxisAlignedBoundingBox,
        rotation_matrix: &Matrix3x3,
    ) -> AxisAlignedBoundingBox {
        let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    let x = i_f * original_bbox.axis_interval(Axis::X).max
                        + (1.0 - i_f) * original_bbox.axis_interval(Axis::X).min;
                    let y = j_f * original_bbox.axis_interval(Axis::Y).max
                        + (1.0 - j_f) * original_bbox.axis_interval(Axis::Y).min;
                    let z = k_f * original_bbox.axis_interval(Axis::Z).max
                        + (1.0 - k_f) * original_bbox.axis_interval(Axis::Z).min;

                    let corner = Vector3::new(x, y, z);
                    let rotated = rotation_matrix * corner;

                    for axis in Axis::iter() {
                        *min.axis_value_mut(axis) =
                            min.axis_value(axis).min(rotated.axis_value(axis));
                        *max.axis_value_mut(axis) =
                            max.axis_value(axis).max(rotated.axis_value(axis));
                    }
                }
            }
        }

        AxisAlignedBoundingBox::new_from_points(min, max)
    }
}

impl Node for Rotate {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space using inverse rotation
        let origin = &self.inverse_rotation_matrix * ray.origin;
        let direction = &self.inverse_rotation_matrix * ray.direction;
        let rotated_r = Ray::new_with_time(origin, direction, ray.time);

        // Determine whether an intersection exists in object space
        let mut hit = self.object.hit(ctx, &rotated_r, ray_t)?;

        // Transform the intersection from object space back to world space
        hit.pt = &self.rotation_matrix * hit.pt;
        hit.normal = &self.rotation_matrix * hit.normal;

        Some(hit)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
