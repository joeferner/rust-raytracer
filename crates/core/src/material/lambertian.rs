use core::f64;
use std::sync::Arc;

use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, ScatterResult},
    object::HitRecord,
    texture::{SolidColor, Texture},
    utils::OrthonormalBasis,
};

#[derive(Debug)]
pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn new_from_color(color: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let uvw = OrthonormalBasis::new(hit.normal);
        let scatter_direction =
            uvw.transform_to_local(Vector3::random_cosine_direction(&*ctx.random));
        let scattered = Ray::new_with_time(hit.pt, scatter_direction.unit(), r_in.time);
        let pdf = uvw.w.dot(&scattered.direction) / f64::consts::PI;

        Some(ScatterResult {
            attenuation: self.texture.value(hit.u, hit.v, hit.pt),
            scattered,
            pdf,
        })
    }

    fn scattering_pdf(
        &self,
        _ctx: &RenderContext,
        _r_in: &Ray,
        hit: &HitRecord,
        scattered: &Ray,
    ) -> f64 {
        let cos_theta = hit.normal.dot(&scattered.direction.unit());
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / f64::consts::PI
        }
    }
}
