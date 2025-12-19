use core::f64;
use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Node, Ray, RenderContext, Vector3,
    material::Material,
    object::{Disc, Group, HitRecord},
};

#[derive(Debug)]
pub struct ConeFrustum {
    pub object_node: Group,
}

impl ConeFrustum {
    /// Creates a closed cylinder (or frustum/cone) with its base centered at `base`.
    ///
    /// The frustum spans from `base.y` to `base.y + height`.
    pub fn new(
        base: Vector3,
        height: f64,
        top_radius: f64,
        bottom_radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        // Y-coordinates for the caps
        let y_base = base.y; // Bottom Y-coordinate
        let y_top = base.y + height; // Top Y-coordinate

        let mut nodes: Vec<Arc<dyn Node>> = Vec::new();

        // --- 1. Top Cap (Disc) ---
        // Normal points UP (+Y)
        if top_radius > 1e-4 {
            let top_center = Vector3::new(base.x, y_top, base.z);
            let top_normal = Vector3::new(0.0, 1.0, 0.0);
            let top_disc = Disc::new(top_center, top_radius, top_normal, material.clone());
            nodes.push(Arc::new(top_disc));
        }

        // --- 2. Bottom Cap (Disc) ---
        // Normal points DOWN (-Y)
        if bottom_radius > 1e-4 {
            let bottom_center = Vector3::new(base.x, y_base, base.z);
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
        // base is used as the reference point for the frustum wall
        let side_wall = ConeFrustumWall::new(
            base,
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

impl Node for ConeFrustum {
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

#[derive(Debug)]
struct ConeFrustumWall {
    base: Vector3,
    height: f64,
    r0: f64, // Bottom radius
    r1: f64, // Top radius
    pub material: Arc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
}

impl ConeFrustumWall {
    /// Creates a frustum wall section.
    ///
    /// The frustum is centered at (base.x, base.y + height/2, base.z).
    /// The bottom cap is centered at (base.x, base.y, base.z).
    pub fn new(
        base: Vector3,
        height: f64,
        r1: f64, // top radius
        r0: f64, // bottom radius
        material: Arc<dyn Material>,
    ) -> Self {
        // Assume min radius is 0 for bounding box calculation
        let max_radius = f64::max(r0, r1);

        // Bounding Box Calculation
        // Frustum spans from base.y to base.y + height
        let min_p = Vector3::new(
            base.x - max_radius,
            base.y, // Min Y is the base Y
            base.z - max_radius,
        );
        let max_p = Vector3::new(
            base.x + max_radius,
            base.y + height, // Max Y is base Y + height
            base.z + max_radius,
        );

        Self {
            base,
            height,
            r0,
            r1,
            material,
            bbox: AxisAlignedBoundingBox::new_from_points(min_p, max_p),
        }
    }

    /// Converts a point on the frustum's wall into UV coordinates.
    /// Maps azimuth (angle around Y) to U, and height (Y-coordinate) to V.
    pub fn get_uv(pt: Vector3, base_y: f64, height: f64) -> (f64, f64) {
        // Calculate U (azimuth)
        // atan2(z, x) gives angle in [-pi, pi]. Add PI to get [0, 2pi].
        // Normalize to [0, 1].
        let phi = pt.z.atan2(pt.x);
        let u = (phi + f64::consts::PI) / (2.0 * f64::consts::PI);

        // Calculate V (height)
        // Normalize the Y coordinate relative to the base
        let local_y = pt.y - base_y;
        let v = local_y / height; // (local_y in [0, h]) -> (v in [0, 1])

        // Clamp V to ensure it stays in [0, 1] due to potential floating point errors
        let v = v.clamp(0.0, 1.0);

        (u, v)
    }
}

impl Node for ConeFrustumWall {
    fn hit(&self, _ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let h = self.height;
        let base = self.base; // Get the full base vector
        let base_y = base.y;
        let r0 = self.r0;
        let r1 = self.r1;

        // --- Translate the ray origin into the object's local space ---
        let local_origin = ray.origin - base;

        // Ray origin and direction components for a Y-aligned frustum centered at (0, base.y, 0) in local space.
        let ox = local_origin.x;
        let oy = local_origin.y;
        let oz = local_origin.z;

        // Ray direction components are unchanged by translation
        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let dz = ray.direction.z;

        // Map y-coordinates relative to the bottom of the frustum (y_local = 0 to h)
        // Since local_origin.y is already relative to base.y, y_local is simply oy
        let y_local = oy;
        let dy_local = dy;

        // Frustum wall equation setup
        let dr = r1 - r0;
        let k = dr / h; // slope of radius as a function of local height y

        // Quadratic equation: A*t^2 + B*t + C = 0
        // A = dx^2 + dz^2 - k^2 * dy^2
        let a = dx * dx + dz * dz - k * k * dy_local * dy_local;

        // B = 2 * [ ox*dx + oz*dz - k^2 * y_local*dy - k*r0*dy ]
        let b = 2.0 * (ox * dx + oz * dz - k * k * y_local * dy_local - k * r0 * dy_local);

        // C = ox^2 + oz^2 - (r0 + k*y_local)^2
        let c = ox * ox + oz * oz - (r0 * r0 + 2.0 * k * r0 * y_local + k * k * y_local * y_local);

        // ... Handle A=0, discriminant check ...
        if a.abs() < 1e-8 {
            return None;
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Solve for the nearest root (t) that is in range (ray_t)
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

        // Check height bounds. The frustum spans from base_y to base_y + h
        let y_min = base_y;
        let y_max = base_y + h;

        // Check the hit point's Y coordinate against the global bounds
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

        // Calculate the hit point's local Y coordinate (y' in [0, h])
        let hit_y_local = pt.y - base_y;

        // Calculate the radius at the hit point
        let hit_radius = r0 + k * hit_y_local;

        // The normal calculation must also be done relative to the central axis (base.x, base.z).
        let pt_local = pt - base;

        // Normal N is proportional to (X_local, k * R(y), Z_local)
        let outward_normal = Vector3::new(
            pt_local.x,
            k * hit_radius, // This component accounts for the cone/frustum slope
            pt_local.z,
        )
        .unit();

        // UV calculation still uses the global hit point's Y and Z/X relative to the base.
        // The azimuth calculation is based on the local X and Z:
        let (u, v) = ConeFrustumWall::get_uv(pt_local, 0.0, h); // We pass 0.0 as base_y because pt_local is already relative to the base.

        let mut rec = HitRecord {
            pt, // Store global hit point
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

    fn pdf_value(&self, _ctx: &RenderContext, _origin: &Vector3, _direction: &Vector3) -> f64 {
        let h = self.height;
        let r0 = self.r0;
        let r1 = self.r1;

        // Slant height L
        let dr = r1 - r0;
        let l = (h * h + dr * dr).sqrt();

        // Lateral Surface Area A = pi * (r0 + r1) * L
        let area = f64::consts::PI * (r0 + r1) * l;

        // PDF is 1 / Area
        if area > 1e-8 {
            1.0 / area
        } else {
            0.0 // Handle zero-area case (e.g., a line segment)
        }
    }

    fn random(&self, ctx: &RenderContext, _origin: &Vector3) -> Vector3 {
        let r0 = self.r0;
        let r1 = self.r1;
        let h = self.height;

        let u1 = ctx.random.rand_interval(0.0, 1.0);
        let u2 = ctx.random.rand_interval(0.0, 1.0);

        // 1. Azimuthal Angle (u1)
        let phi = 2.0 * f64::consts::PI * u1;

        // 2. Uniformly Sampled Radius (R_rand) (u2)
        // R_rand^2 = r0^2 + u2 * (r1^2 - r0^2)
        let r_rand_sq = r0 * r0 + u2 * (r1 * r1 - r0 * r0);
        let r_rand = r_rand_sq.sqrt();

        // Handle the degenerate cylinder case (r1 == r0)
        let y_local = if (r1 - r0).abs() < 1e-8 {
            // Cylinder: y_local is uniform
            h * u2
        } else {
            // Frustum: Calculate local height y' from R_rand
            // y' = h / (r1 - r0) * (R_rand - r0)
            h / (r1 - r0) * (r_rand - r0)
        };

        // 3. Construct Local Point (P_local)
        let p_local = Vector3::new(r_rand * phi.cos(), y_local, r_rand * phi.sin());

        // 4. Translate back to Global Space
        p_local + self.base
    }
}
