use std::sync::Arc;

use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, ScatterResult},
    object::HitRecord,
    texture::{SolidColor, Texture},
};

#[derive(Debug)]
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn new_from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.texture.value(hit.u, hit.v, hit.pt),
            scattered: Ray::new_with_time(hit.pt, Vector3::random_unit(&*ctx.random), r_in.time),
        })
    }
}
