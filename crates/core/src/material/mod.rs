use std::fmt::Debug;

use crate::{Color, Ray, RenderContext, Vector3, object::HitRecord};

pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
pub mod refractive;

pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use refractive::Refractive;

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

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
    pub pdf: f64,
}
