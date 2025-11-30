use std::sync::Arc;

use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, ScatterResult},
    object::HitRecord,
    texture::{SolidColor, Texture},
};

#[derive(Debug)]
pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn new_from_color(emit: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ctx: &RenderContext,
        _r_in: &Ray,
        _hit: &HitRecord,
    ) -> Option<ScatterResult> {
        None
    }

    fn emitted(&self, _r_in: &Ray, hit: &HitRecord, u: f64, v: f64, pt: Vector3) -> Color {
        if hit.front_face {
            self.texture.value(u, v, pt)
        } else {
            Color::BLACK
        }
    }
}
