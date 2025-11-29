use core::f64;
use std::{f64::consts::PI, sync::Arc};

use crate::{
    AxisAlignedBoundingBox, Interval, RenderContext, Vector3,
    material::Material,
    object::{HitRecord, Node},
    ray::Ray,
};

#[derive(Debug)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    pub material: Arc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material>) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);
        Self {
            center: Ray::new(center, Vector3::ZERO),
            radius,
            material,
            bbox: AxisAlignedBoundingBox::new_from_points(center - radius_vec, center + radius_vec),
        }
    }

    pub fn set_direction(&mut self, direction: Vector3) {
        self.center = Ray::new(self.center.origin, direction);
        self.update_bbox();
    }

    fn update_bbox(&mut self) {
        let rvec = Vector3::new(self.radius, self.radius, self.radius);
        let box1 = AxisAlignedBoundingBox::new_from_points(
            self.center.at(0.0) - rvec,
            self.center.at(0.0) + rvec,
        );
        let box2 = AxisAlignedBoundingBox::new_from_points(
            self.center.at(1.0) - rvec,
            self.center.at(1.0) + rvec,
        );
        self.bbox = AxisAlignedBoundingBox::new_from_bbox(box1, box2);
    }

    /// Converts a point on the unit sphere into UV coordinates.
    ///
    /// # Parameters
    /// - `pt`: A point on the unit sphere (radius = 1, centered at the origin).
    ///
    /// # Returns
    /// A tuple `(u, v)` where:
    /// - `u` ∈ [0, 1]: the normalized azimuth angle around the Y axis,
    ///   measured from the negative X direction.  
    /// - `v` ∈ [0, 1]: the normalized polar angle, where 0 corresponds to
    ///   `y = -1` (south pole) and 1 corresponds to `y = +1` (north pole).
    ///
    /// # Mapping Examples
    /// | Point        | (u, v)       |
    /// |--------------|--------------|
    /// | ( 1,  0,  0) | (0.50, 0.50) |
    /// | (-1,  0,  0) | (0.00, 0.50) |
    /// | ( 0,  1,  0) | (0.50, 1.00) |
    /// | ( 0, -1,  0) | (0.50, 0.00) |
    /// | ( 0,  0,  1) | (0.25, 0.50) |
    /// | ( 0,  0, -1) | (0.75, 0.50) |
    pub fn get_uv(pt: Vector3) -> (f64, f64) {
        // produces a polar angle where the south pole maps to 0 and the north
        // pole maps to 1 after normalization.
        let theta = (-pt.y).acos();

        // yields an azimuth that wraps `[0, 2π)` with `u = 0` at `(-1, 0, 0)`
        // and increasing counterclockwise when viewed from above the positive
        // Y axis.
        let phi = (-pt.z).atan2(pt.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Node for Sphere {
    fn hit(&self, _ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let pt = ray.at(t);
        let outward_normal = (pt - current_center) / self.radius;
        let (u, v) = Sphere::get_uv(outward_normal);
        let mut rec = HitRecord {
            pt,
            normal: Vector3::ZERO, // set by set_face_normal
            t,
            u,
            v,
            front_face: false,
            material: self.material.clone(),
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
