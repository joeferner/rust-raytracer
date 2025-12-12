use core::f64;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    Color, CosinePdf, Ray, RenderContext,
    material::{Material, PdfOrRay, ScatterResult},
    object::HitRecord,
    texture::{SolidColor, Texture},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[typetag::serde]
impl Material for Lambertian {
    fn scatter(&self, _ctx: &RenderContext, _r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.texture.value(hit.u, hit.v, hit.pt),
            pdf_or_ray: PdfOrRay::Pdf(Arc::new(CosinePdf::new(hit.normal))),
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
