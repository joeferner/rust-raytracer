use core::f64;
use std::{any::Any, sync::Arc};

use crate::{
    AxisAlignedBoundingBox, Interval, Node, Ray, RenderContext, Vector3, material::Material,
    object::HitRecord,
};

/// A planar quadrilateral primitive defined by a corner point and two edge vectors.
///
/// The quad is defined by a corner point `q` and two edge vectors `u` and `v` that
/// span the plane. The four corners of the quad are: `q`, `q + u`, `q + v`, and `q + u + v`.
#[derive(Debug)]
pub struct Quad {
    /// Corner point of the quadrilateral
    q: Vector3,
    /// First edge vector from the corner
    u: Vector3,
    /// Second edge vector from the corner
    v: Vector3,
    /// Surface material for rendering
    material: Arc<dyn Material>,
    /// Axis-aligned bounding box containing the quad
    bbox: AxisAlignedBoundingBox,
    /// Unit normal vector perpendicular to the quad's plane
    normal: Vector3,
    /// Plane equation constant (distance from origin)
    d: f64,
    /// Precomputed vector for barycentric coordinate calculations
    w: Vector3,
    /// Surface area of the quadrilateral
    area: f64,
}

impl Quad {
    /// Creates a new quadrilateral primitive.
    ///
    /// # Arguments
    ///
    /// * `q` - Corner point of the quad
    /// * `u` - First edge vector defining one side
    /// * `v` - Second edge vector defining another side
    /// * `material` - Material for the quad's surface
    ///
    /// # Returns
    ///
    /// A new `Quad` instance with precomputed geometric properties.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let quad = Quad::new(
    ///     Vector3::new(0.0, 0.0, 0.0),
    ///     Vector3::new(1.0, 0.0, 0.0),
    ///     Vector3::new(0.0, 1.0, 0.0),
    ///     material,
    /// );
    /// ```
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
            area: n.length(),
        }
    }

    /// Computes the axis-aligned bounding box encompassing all four vertices of the quad.
    ///
    /// This method constructs the bounding box by considering both diagonals of the
    /// quadrilateral and combining them to ensure all corners are contained.
    ///
    /// # Arguments
    ///
    /// * `q` - Corner point of the quad
    /// * `u` - First edge vector
    /// * `v` - Second edge vector
    ///
    /// # Returns
    ///
    /// An `AxisAlignedBoundingBox` that contains all four vertices of the quad.
    fn calculate_bbox(q: Vector3, u: Vector3, v: Vector3) -> AxisAlignedBoundingBox {
        let bbox_diagonal1 = AxisAlignedBoundingBox::new_from_points(q, q + u + v);
        let bbox_diagonal2 = AxisAlignedBoundingBox::new_from_points(q + u, q + v);
        AxisAlignedBoundingBox::new_from_bbox(bbox_diagonal1, bbox_diagonal2)
    }

    /// Determines if the given barycentric coordinates lie within the unit square.
    ///
    /// This helper function checks whether a point (in parametric coordinates) falls
    /// inside the quad by verifying that both coordinates are within [0, 1].
    ///
    /// # Arguments
    ///
    /// * `a` - First barycentric coordinate (alpha)
    /// * `b` - Second barycentric coordinate (beta)
    ///
    /// # Returns
    ///
    /// * `Some((a, b))` if the point is interior to the quad
    /// * `None` if the point lies outside the quad boundaries
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
    /// Tests for ray-quad intersection.
    ///
    /// This method performs the following steps:
    /// 1. Checks if the ray is parallel to the quad's plane
    /// 2. Computes the intersection point with the plane
    /// 3. Determines if the intersection point lies within the quad boundaries
    /// 4. Returns hit information if all conditions are met
    ///
    /// # Arguments
    ///
    /// * `_ctx` - Rendering context (unused for basic quad intersection)
    /// * `ray` - The ray to test for intersection
    /// * `ray_t` - Valid interval for intersection parameter t
    ///
    /// # Returns
    ///
    /// * `Some(HitRecord)` containing intersection details if the ray hits the quad
    /// * `None` if there is no valid intersection
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

    /// Returns a reference to the quad's axis-aligned bounding box.
    ///
    /// # Returns
    ///
    /// A reference to the precomputed bounding box that contains the quad.
    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }

    /// Calculates the probability density function value for sampling this quad from a given origin.
    ///
    /// This is used in importance sampling for advanced rendering techniques. The PDF accounts
    /// for the solid angle subtended by the quad as seen from the origin point.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Rendering context
    /// * `origin` - Point from which the quad is being sampled
    /// * `direction` - Direction vector towards the quad
    ///
    /// # Returns
    ///
    /// The PDF value, or 0.0 if the direction doesn't intersect the quad. The PDF is
    /// computed as `distance² / (cosine * area)` where cosine is the angle between
    /// the direction and the quad's normal.
    fn pdf_value(&self, ctx: &RenderContext, origin: &Vector3, direction: &Vector3) -> f64 {
        let hit = match self.hit(
            ctx,
            &Ray::new(*origin, *direction),
            Interval::new(0.001, f64::INFINITY),
        ) {
            Some(hit) => hit,
            None => {
                return 0.0;
            }
        };

        let distance_squared = hit.t * hit.t * direction.length_squared();
        let cosine = (direction.dot(&hit.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    /// Generates a random direction from a given origin point towards a random point on the quad.
    ///
    /// This method is used for importance sampling in rendering algorithms like path tracing.
    /// It uniformly samples a point on the quad's surface and returns the direction vector
    /// from the origin to that point.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Rendering context containing the random number generator
    /// * `origin` - The point from which to generate a direction
    ///
    /// # Returns
    ///
    /// A direction vector from the origin to a randomly sampled point on the quad.
    fn random(&self, ctx: &RenderContext, origin: &Vector3) -> Vector3 {
        let p = self.q + (ctx.random.rand() * self.u) + (ctx.random.rand() * self.v);
        p - *origin
    }

    /// Returns a reference to this quad as an `Any` trait object for dynamic type checking.
    ///
    /// # Returns
    ///
    /// A reference to self as `&dyn Any`.
    fn as_any(&self) -> &dyn Any {
        self
    }
}
