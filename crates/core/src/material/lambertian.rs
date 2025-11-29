use std::sync::Arc;

use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, ScatterResult},
    object::HitRecord,
    texture::{SolidColor, Texture},
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
        let mut scatter_direction = hit.normal + Vector3::random_unit(&*ctx.random);

        if scatter_direction.is_near_near() {
            scatter_direction = hit.normal
        }

        Some(ScatterResult {
            attenuation: self.texture.value(hit.u, hit.v, hit.pt),
            scattered: Ray::new_with_time(hit.pt, scatter_direction, r_in.time),
        })
    }
}
