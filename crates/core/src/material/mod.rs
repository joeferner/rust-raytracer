use std::{fmt::Debug, sync::Arc};

use crate::{Color, ProbabilityDensityFunction, Ray, RenderContext, Vector3, object::HitRecord};

pub mod dielectric;
pub mod diffuse_light;
pub mod empty;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use empty::EmptyMaterial;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

#[typetag::serde(tag = "type")]
pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult>;

    fn emitted(&self, _r_in: &Ray, _hit: &HitRecord, _u: f64, _v: f64, _pt: Vector3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(
        &self,
        _ctx: &RenderContext,
        _r_in: &Ray,
        _hit: &HitRecord,
        _scattered: &Ray,
    ) -> f64 {
        0.0
    }
}

pub enum PdfOrRay {
    Pdf(Arc<dyn ProbabilityDensityFunction>),
    Ray(Ray),
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub pdf_or_ray: PdfOrRay,
}
