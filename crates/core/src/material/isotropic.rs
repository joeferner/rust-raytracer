use core::f64;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    Color, Ray, RenderContext, SpherePdf,
    material::{Material, PdfOrRay, ScatterResult},
    object::HitRecord,
    texture::{SolidColor, Texture},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[typetag::serde]
impl Material for Isotropic {
    fn scatter(&self, _ctx: &RenderContext, _r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.texture.value(hit.u, hit.v, hit.pt),
            pdf_or_ray: PdfOrRay::Pdf(Arc::new(SpherePdf::new())),
        })
    }

    fn scattering_pdf(
        &self,
        _ctx: &RenderContext,
        _r_in: &Ray,
        _hit: &HitRecord,
        _scattered: &Ray,
    ) -> f64 {
        1.0 / (4.0 / f64::consts::PI)
    }
}
