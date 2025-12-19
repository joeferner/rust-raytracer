use core::f64;
use std::sync::Arc;

use crate::{
    Axis, AxisAlignedBoundingBox, Interval, Matrix3x3, Node, Ray, RenderContext, object::HitRecord,
};

#[derive(Debug)]
pub struct Scale {
    object: Arc<dyn Node>,
    scale_matrix: Matrix3x3,
    inverse_scale_matrix: Matrix3x3,
    bbox: AxisAlignedBoundingBox,
}

impl Scale {
    /// Creates a scaling transformation
    pub fn new(object: Arc<dyn Node>, scale_x: f64, scale_y: f64, scale_z: f64) -> Self {
        // 1. Create the scale matrix (diagonal matrix with scale factors)
        let scale_matrix = Matrix3x3::new([
            [scale_x, 0.0, 0.0],
            [0.0, scale_y, 0.0],
            [0.0, 0.0, scale_z],
        ]);

        // 2. Create the inverse scale matrix
        // The inverse of a diagonal scaling matrix is another diagonal matrix
        // with the reciprocals of the scale factors on the diagonal.
        // We must check for division by zero (or near zero).
        let inv_x = if scale_x.abs() > 1e-9 {
            1.0 / scale_x
        } else {
            f64::INFINITY
        };
        let inv_y = if scale_y.abs() > 1e-9 {
            1.0 / scale_y
        } else {
            f64::INFINITY
        };
        let inv_z = if scale_z.abs() > 1e-9 {
            1.0 / scale_z
        } else {
            f64::INFINITY
        };

        let inverse_scale_matrix =
            Matrix3x3::new([[inv_x, 0.0, 0.0], [0.0, inv_y, 0.0], [0.0, 0.0, inv_z]]);

        // 3. Compute the new bounding box
        let obj_bbox = object.bounding_box();
        let bbox = Self::compute_bounding_box(obj_bbox, scale_x, scale_y, scale_z);

        Self {
            object,
            scale_matrix,
            inverse_scale_matrix,
            bbox,
        }
    }

    fn compute_bounding_box(
        original_bbox: &AxisAlignedBoundingBox,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> AxisAlignedBoundingBox {
        // Scaling an AABB by positive scale factors simply scales the min/max of the intervals.
        // If a scale factor is negative (mirroring), the min/max might swap.
        let min_x = original_bbox.axis_interval(Axis::X).min;
        let max_x = original_bbox.axis_interval(Axis::X).max;
        let min_y = original_bbox.axis_interval(Axis::Y).min;
        let max_y = original_bbox.axis_interval(Axis::Y).max;
        let min_z = original_bbox.axis_interval(Axis::Z).min;
        let max_z = original_bbox.axis_interval(Axis::Z).max;

        // Calculate new bounds after scaling, ensuring min <= max
        let new_min_x = (min_x * scale_x).min(max_x * scale_x);
        let new_max_x = (min_x * scale_x).max(max_x * scale_x);
        let new_min_y = (min_y * scale_y).min(max_y * scale_y);
        let new_max_y = (min_y * scale_y).max(max_y * scale_y);
        let new_min_z = (min_z * scale_z).min(max_z * scale_z);
        let new_max_z = (min_z * scale_z).max(max_z * scale_z);

        // Create the new AxisAlignedBoundingBox
        AxisAlignedBoundingBox::new_from_intervals(
            Interval::new(new_min_x, new_max_x),
            Interval::new(new_min_y, new_max_y),
            Interval::new(new_min_z, new_max_z),
        )
    }
}

impl Node for Scale {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // 1. Transform the ray from world space to object space using the inverse scale matrix
        let origin = &self.inverse_scale_matrix * ray.origin;
        let direction = &self.inverse_scale_matrix * ray.direction;
        let scaled_r = Ray::new_with_time(origin, direction, ray.time);

        // 2. Determine whether an intersection exists in object space
        let mut hit = self.object.hit(ctx, &scaled_r, ray_t)?;

        // 3. Transform the intersection from object space back to world space
        // a. Hit point is transformed by the scale matrix
        hit.pt = &self.scale_matrix * hit.pt;

        // b. Normal vector transformation:
        // Normals transform by the *transpose of the inverse* of the transformation matrix.
        // For a diagonal scaling matrix M, its inverse is $M^{-1}$, and the transpose of $M^{-1}$ is
        // $(M^{-1})^T = M^{-1}$. So, we use the inverse scale matrix $M^{-1}$ for the normal.
        hit.normal = &self.inverse_scale_matrix * hit.normal;

        // Normals also need to be re-normalized after transformation
        hit.normal = hit.normal.unit();

        Some(hit)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
