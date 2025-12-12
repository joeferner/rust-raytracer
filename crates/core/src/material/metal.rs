use serde::{Deserialize, Serialize};

use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, PdfOrRay, ScatterResult},
    object::HitRecord,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

#[typetag::serde]
impl Material for Metal {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = r_in.direction.reflect(hit.normal);
        let reflected = reflected.unit() + (self.fuzz * Vector3::random_unit(&*ctx.random));

        Some(ScatterResult {
            attenuation: self.albedo,
            pdf_or_ray: PdfOrRay::Ray(Ray::new_with_time(hit.pt, reflected, r_in.time)),
        })
    }
}
