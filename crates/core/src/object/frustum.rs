use core::f64;
use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Node, Ray, RenderContext, Vector3,
    material::Material,
    object::{Disc, Group, HitRecord},
};

#[derive(Debug)]
struct FrustumWall {
    center_y: f64, // The Y-coordinate of the center of the cylinder's height
    height: f64,
    r0: f64, // Bottom radius
    r1: f64, // Top radius
    pub material: Arc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
}

impl FrustumWall {
    pub fn new(
        center: Vector3,
        height: f64,
        r1: f64, // top radius
        r0: f64, // bottom radius
        material: Arc<dyn Material>,
    ) -> Self {
        // Assume min radius is 0 for bounding box calculation
        let max_radius = f64::max(r0, r1);
        let half_height = height / 2.0;

        // Bounding Box Calculation
        let min_p = Vector3::new(
            center.x - max_radius,
            center.y - half_height,
            center.z - max_radius,
        );
        let max_p = Vector3::new(
            center.x + max_radius,
            center.y + half_height,
            center.z + max_radius,
        );

        Self {
            center_y: center.y,
            height,
            r0,
            r1,
            material,
            bbox: AxisAlignedBoundingBox::new_from_points(min_p, max_p),
        }
    }

    /// Converts a point on the frustum's wall into UV coordinates.
    /// Maps azimuth (angle around Y) to U, and height (Y-coordinate) to V.
    pub fn get_uv(pt: Vector3, center_y: f64, height: f64) -> (f64, f64) {
        // Calculate U (azimuth)
        // atan2(z, x) gives angle in [-pi, pi]. Add PI to get [0, 2pi].
        // Normalize to [0, 1].
        let phi = pt.z.atan2(pt.x);
        let u = (phi + f64::consts::PI) / (2.0 * f64::consts::PI);

        // Calculate V (height)
        // Normalize the Y coordinate relative to the height
        let local_y = pt.y - center_y;
        let v = (local_y / height) + 0.5; // (local_y in [-h/2, h/2]) -> (v in [0, 1])

        // Clamp V to ensure it stays in [0, 1] due to potential floating point errors
        let v = v.clamp(0.0, 1.0);

        (u, v)
    }
}

impl Node for FrustumWall {
    fn hit(&self, _ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let h = self.height;
        let half_h = h / 2.0;
        let c_y = self.center_y; // center Y-coordinate
        let r0 = self.r0;
        let r1 = self.r1;

        // Ray origin and direction components for a Y-aligned frustum.
        // We assume the frustum is centered at (0, c_y, 0) for the math,
        // which means the ray is translated to a local coordinate system.
        // For simplicity, we assume center.x = center.z = 0 in new().
        let ox = ray.origin.x;
        let oy = ray.origin.y;
        let oz = ray.origin.z;
        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let dz = ray.direction.z;

        // Map y-coordinates relative to the bottom of the frustum (y_local = 0 to h)
        let y_local = oy - (c_y - half_h);
        let dy_local = dy;

        // Frustum wall equation derivation (from bottom radius r0 at y=0 to r1 at y=h)
        // R(y) = r0 + (r1 - r0) * (y / h)
        let dr = r1 - r0;
        let k = dr / h; // slope of radius as a function of local height y

        // Quadratic equation: A*t^2 + B*t + C = 0
        // A = dx^2 + dz^2 - k^2 * dy^2
        let a = dx * dx + dz * dz - k * k * dy_local * dy_local;

        // B = 2 * [ ox*dx + oz*dz - k^2 * y_local*dy - k*r0*dy ]
        let b = 2.0 * (ox * dx + oz * dz - k * k * y_local * dy_local - k * r0 * dy_local);

        // C = ox^2 + oz^2 - (r0 + k*y_local)^2
        let c = ox * ox + oz * oz - (r0 * r0 + 2.0 * k * r0 * y_local + k * k * y_local * y_local);

        // Handle case where ray is parallel to the cylinder axis (A=0)
        // and ray is not inside (B=0).
        if a.abs() < 1e-8 {
            return None; // Parallel, or ray passes through the central axis (handled by C)
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Solve for the nearest root (t) that is in range (ray_t) and on the surface (0 < y < h)
        let mut t = (-b - sqrt_discriminant) / (2.0 * a);

        // Check first root
        if !ray_t.contains(t) {
            t = (-b + sqrt_discriminant) / (2.0 * a);
            if !ray_t.contains(t) {
                return None;
            }
        }

        let pt = ray.at(t);
        let hit_y = pt.y;

        // Check height bounds
        let y_min = c_y - half_h;
        let y_max = c_y + half_h;
        if hit_y < y_min || hit_y > y_max {
            // Check the other root if the first one was out of bounds
            let t_other = (-b + sqrt_discriminant) / (2.0 * a);

            if t_other == t {
                return None; // Only one solution, and it was out of bounds
            }

            if !ray_t.contains(t_other) {
                return None;
            }

            let pt_other = ray.at(t_other);
            if pt_other.y < y_min || pt_other.y > y_max {
                return None;
            }

            // The second root is the valid one
            t = t_other;
        }

        let pt = ray.at(t);
        let hit_y_local = pt.y - (c_y - half_h); // y' in [0, h]

        // Calculate the radius at the hit point
        let hit_radius = r0 + k * hit_y_local;

        // Calculate the outward normal
        // Normal N is proportional to (X, k * R(y), Z)
        let outward_normal = Vector3::new(
            pt.x,
            k * hit_radius, // This component accounts for the cone/frustum slope
            pt.z,
        )
        .unit();

        let (u, v) = FrustumWall::get_uv(pt, c_y, h);

        let mut rec = HitRecord {
            pt,
            normal: Vector3::ZERO,
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

    // You would need to implement pdf_value and random here based on your ray tracer's logic.
    // For a side wall, sampling is usually proportional to its surface area.
    fn pdf_value(&self, _ctx: &RenderContext, _origin: &Vector3, _direction: &Vector3) -> f64 {
        0.0 // Simplified: only caps are sampled, or a more complex sampling is needed
    }

    fn random(&self, _ctx: &RenderContext, _origin: &Vector3) -> Vector3 {
        Vector3::ZERO // Simplified: no surface sampling for the side wall
    }
}

pub struct Frustum {
    pub object_node: Group,
}

impl Frustum {
    /// Creates a closed cylinder (or frustum/cone) centered around the Y-axis.
    ///
    /// The cylinder spans from `center.y - height / 2` to `center.y + height / 2`.
    pub fn new(
        center: Vector3,
        height: f64,
        top_radius: f64,
        bottom_radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let half_height = height / 2.0;

        // Y-coordinates for the caps
        let y_top = center.y + half_height;
        let y_bottom = center.y - half_height;

        let mut nodes: Vec<Arc<dyn Node>> = Vec::new();

        // --- 1. Top Cap (Disc) ---
        // Normal points UP (+Y)
        if top_radius > 1e-4 {
            let top_center = Vector3::new(center.x, y_top, center.z);
            let top_normal = Vector3::new(0.0, 1.0, 0.0);
            let top_disc = Disc::new(top_center, top_radius, top_normal, material.clone());
            nodes.push(Arc::new(top_disc));
        }

        // --- 2. Bottom Cap (Disc) ---
        // Normal points DOWN (-Y)
        if bottom_radius > 1e-4 {
            let bottom_center = Vector3::new(center.x, y_bottom, center.z);
            let bottom_normal = Vector3::new(0.0, -1.0, 0.0);
            let bottom_disc = Disc::new(
                bottom_center,
                bottom_radius,
                bottom_normal,
                material.clone(),
            );
            nodes.push(Arc::new(bottom_disc));
        }

        // --- 3. Side Wall ---
        let side_center = center;
        let side_wall = FrustumWall::new(
            side_center,
            height,
            top_radius,    // r1
            bottom_radius, // r0
            material.clone(),
        );
        nodes.push(Arc::new(side_wall));

        Self {
            object_node: Group::from_list(&nodes),
        }
    }
}

impl Node for Frustum {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.object_node.hit(ctx, ray, ray_t)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        self.object_node.bounding_box()
    }

    fn pdf_value(&self, ctx: &RenderContext, origin: &Vector3, direction: &Vector3) -> f64 {
        self.object_node.pdf_value(ctx, origin, direction)
    }

    fn random(&self, ctx: &RenderContext, origin: &Vector3) -> Vector3 {
        self.object_node.random(ctx, origin)
    }
}
