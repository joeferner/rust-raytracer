pub mod cosine_pdf;
pub mod hit_table_pdf;
pub mod sphere_pdf;

pub use cosine_pdf::CosinePdf;
pub use hit_table_pdf::HitTablePdf;
pub use sphere_pdf::SpherePdf;

use core::f64;

use crate::{RenderContext, Vector3};

pub trait ProbabilityDensityFunction {
    fn value(&self, ctx: &RenderContext, direction: &Vector3) -> f64;
    fn generate(&self, ctx: &RenderContext) -> Vector3;
}
