use core::f64;
use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Color, Interval, Node, Ray, RenderContext, Vector3,
    material::{Isotropic, Material},
    object::HitRecord,
    texture::Texture,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Node>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_from_texture(
        boundary: Arc<dyn Node>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_texture(texture)),
        }
    }

    pub fn new_from_color(boundary: Arc<dyn Node>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_color(albedo)),
        }
    }
}

impl Node for ConstantMedium {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit1 = self.boundary.hit(ctx, ray, Interval::UNIVERSE)?;
        let mut hit2 =
            self.boundary
                .hit(ctx, ray, Interval::new(hit1.t + 0.0001, f64::INFINITY))?;

        if hit1.t < ray_t.min {
            hit1.t = ray_t.min;
        }
        if hit2.t > ray_t.max {
            hit2.t = ray_t.max;
        }

        if hit1.t >= hit2.t {
            return None;
        }

        if hit1.t < 0.0 {
            hit1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.neg_inv_density * ctx.random.rand().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;
        Some(HitRecord {
            pt: ray.at(t),
            normal: Vector3::new(1.0, 0.0, 0.0), // arbitrary
            t,
            u: 0.0,
            v: 0.0,
            front_face: true, // also arbitrary
            material: self.phase_function.clone(),
        })
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        self.boundary.bounding_box()
    }
}
