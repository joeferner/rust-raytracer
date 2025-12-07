use core::f64;
use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Random, RenderContext, Vector3,
    material::Material,
    object::{HitRecord, Node},
    ray::Ray,
    utils::OrthonormalBasis,
};

/// Represents a circular disk, defined by its center, radius, and normal.
/// This will be used for the cylinder's top and bottom caps.
#[derive(Debug)]
pub struct Disc {
    center: Vector3,
    radius: f64,
    normal: Vector3, // Normal vector pointing outward from the cylinder
    pub material: Arc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
}

impl Disc {
    pub fn new(center: Vector3, radius: f64, normal: Vector3, material: Arc<dyn Material>) -> Self {
        let radius_y = if normal.y.abs() > 0.9 { 0.0 } else { radius };
        let radius_x = if normal.x.abs() > 0.9 { 0.0 } else { radius };
        let radius_z = if normal.z.abs() > 0.9 { 0.0 } else { radius };

        // Calculate a small delta for the BBox along the normal axis
        let delta = 1e-4;
        let bbox_extents = Vector3::new(
            radius_x + normal.x.abs() * delta,
            radius_y + normal.y.abs() * delta,
            radius_z + normal.z.abs() * delta,
        );

        Self {
            center,
            radius,
            normal,
            material,
            // A Disc's BBox should be calculated based on its plane orientation.
            bbox: AxisAlignedBoundingBox::new_from_points(
                center - bbox_extents,
                center + bbox_extents,
            ),
        }
    }

    /// UV mapping for a circular disk (flat cap).
    /// Maps the projection of the point onto the disk plane to [0, 1]x[0, 1].
    pub fn get_uv(pt: Vector3, center: Vector3, radius: f64) -> (f64, f64) {
        // Assume Y-aligned disk (Normal = +/-Y) for simplicity in UV calculation
        let local_pt = pt - center;

        // Map X and Z (the plane coordinates) relative to the radius
        let u = (local_pt.x / radius + 1.0) * 0.5;
        let v = (local_pt.z / radius + 1.0) * 0.5;
        (u, v)
    }

    /// Generates a random point on the disc's surface.
    /// Uses an OrthonormalBasis to transform a 2D random point into 3D space
    /// on the plane defined by the disc's normal.
    fn random_on_disc(
        random: &dyn Random,
        center: Vector3,
        radius: f64,
        normal: Vector3,
    ) -> Vector3 {
        // 1. Generate a random point in a unit square and map it to a unit disc.
        // We use random_in_unit_disc() from a common library or implement it:
        // A simple way is to use a polar coordinate approach for uniform sampling
        let r_sq = random.rand().sqrt(); // Radius from center: r in [0, 1]
        let phi = 2.0 * f64::consts::PI * random.rand(); // Angle: phi in [0, 2pi)

        let x = r_sq * phi.cos() * radius;
        let y = r_sq * phi.sin() * radius;

        // 2. Define a coordinate system for the disc's plane.
        // OrthonormalBasis helps define u and v vectors that span the plane.
        let uvw = OrthonormalBasis::new(normal);

        // 3. Convert the 2D random point (x, y) into a 3D point on the disc's plane.
        let random_local_pt = uvw.u * x + uvw.v * y;

        // 4. Translate to the disc's actual center.
        center + random_local_pt
    }
}

impl Node for Disc {
    fn hit(&self, _ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // 1. Intersect Ray with the Plane defined by the Disc
        let denominator = ray.direction.dot(&self.normal);

        // Ray is parallel to the plane.
        if denominator.abs() < 1e-8 {
            return None;
        }

        // t = (C - O) . N / (D . N)
        let t = (self.center - ray.origin).dot(&self.normal) / denominator;

        if !ray_t.contains(t) {
            return None;
        }

        // 2. Check if the intersection point is within the Disc's radius
        let pt = ray.at(t);
        let v = pt - self.center; // Vector from center to hit point

        // Check distance squared against radius squared
        if v.length_squared() > self.radius * self.radius {
            return None;
        }

        // 3. Create HitRecord
        let outward_normal = self.normal;
        let (u, v_uv) = Disc::get_uv(pt, self.center, self.radius);

        let mut rec = HitRecord {
            pt,
            normal: Vector3::ZERO,
            t,
            u,
            v: v_uv,
            front_face: false,
            material: self.material.clone(),
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }

    fn pdf_value(&self, ctx: &RenderContext, origin: &Vector3, direction: &Vector3) -> f64 {
        // 1. Check if the ray hits the disc.
        // We use a temporary Ray and an Interval to perform the hit test.
        let ray = Ray::new(*origin, *direction);
        let hit_result = self.hit(
            ctx,
            &ray,
            // Use a small epsilon for the minimum t value
            Interval::new(1e-4, f64::INFINITY),
        );

        match hit_result {
            None => 0.0, // Ray does not hit the disc
            Some(rec) => {
                // 2. Calculate the squared distance (r^2) from the origin to the hit point
                let dist_squared = rec.t * rec.t * direction.length_squared();

                // 3. Calculate the cosine of the angle between the normal (N) and the
                // outgoing ray direction (-D).
                // D is the *incident* ray direction, so the *outgoing* direction is -D.
                let cosine_abs = (rec.normal.dot(&-*direction)).abs();

                // If the disc is being viewed edge-on or if cosine_abs is very small,
                // the PDF approaches infinity, so we return 0.0 or a very small number
                if cosine_abs < 1e-8 {
                    return 0.0;
                }

                // 4. Calculate the Disc's Area
                let area = f64::consts::PI * self.radius * self.radius;

                // 5. Calculate the PDF value
                // PDF = (r^2) / (|N . D| * Area)
                dist_squared / (cosine_abs * area)
            }
        }
    }

    fn random(&self, ctx: &RenderContext, origin: &Vector3) -> Vector3 {
        // Get a random point on the disc's surface
        let target = Disc::random_on_disc(&*ctx.random, self.center, self.radius, self.normal);

        // Return the direction vector from the origin to that random point
        target - *origin
    }
}
