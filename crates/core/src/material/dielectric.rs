use core::f64;

use serde::{Deserialize, Serialize};

use crate::{
    Color, Ray, RenderContext,
    material::{Material, PdfOrRay, ScatterResult},
    object::HitRecord,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// Use Schlick's approximation for reflectance.
    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[typetag::serde]
impl Material for Dielectric {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > ctx.random.rand() {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, ri)
        };

        Some(ScatterResult {
            attenuation: Color::WHITE,
            pdf_or_ray: PdfOrRay::Ray(Ray::new_with_time(hit.pt, direction, r_in.time)),
        })
    }
}
