pub mod cosine;
pub mod hittable;
pub mod mixture;
pub mod sphere;

pub use cosine::CosinePdf;
pub use hittable::HittablePdf;
pub use mixture::MixturePdf;
pub use sphere::SpherePdf;

use core::f64;

use crate::{RenderContext, Vector3};

pub trait ProbabilityDensityFunction {
    fn value(&self, ctx: &RenderContext, direction: &Vector3) -> f64;
    fn generate(&self, ctx: &RenderContext) -> Vector3;
}
