use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Node, Ray, RenderContext, Vector3, material::Material,
    object::HitRecord,
};

#[derive(Debug)]
pub struct Quad {
    q: Vector3,
    u: Vector3,
    v: Vector3,
    material: Arc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
    normal: Vector3,
    d: f64,
    w: Vector3,
}

impl Quad {
    pub fn new(q: Vector3, u: Vector3, v: Vector3, material: Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        Self {
            q,
            u,
            v,
            material,
            bbox: Quad::calculate_bbox(q, u, v),
            normal,
            d,
            w,
        }
    }

    /// Compute the bounding box of all four vertices.
    fn calculate_bbox(q: Vector3, u: Vector3, v: Vector3) -> AxisAlignedBoundingBox {
        let bbox_diagonal1 = AxisAlignedBoundingBox::new_from_points(q, q + u + v);
        let bbox_diagonal2 = AxisAlignedBoundingBox::new_from_points(q + u, q + v);
        AxisAlignedBoundingBox::new_from_bbox(bbox_diagonal1, bbox_diagonal2)
    }

    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }

        Some((a, b))
    }
}

impl Node for Quad {
    fn hit(&self, _ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(&ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hit_pt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_pt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_pt_vector));

        let (u, v) = match Quad::is_interior(alpha, beta) {
            None => {
                return None;
            }
            Some(v) => v,
        };

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        let mut hit = HitRecord {
            pt: intersection,
            normal: Vector3::ZERO,
            t,
            u,
            v,
            front_face: false,
            material: self.material.clone(),
        };
        hit.set_face_normal(ray, self.normal);
        Some(hit)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
