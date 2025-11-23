use crate::{
    Color, Ray, RenderContext,
    material::{Material, ScatterResult},
    object::HitRecord,
};

#[derive(Debug)]
pub struct Refractive {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    refraction_index: f64,
}

impl Refractive {
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

impl Material for Refractive {
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
            scattered: Ray::new(hit.pt, direction),
        })
    }
}
