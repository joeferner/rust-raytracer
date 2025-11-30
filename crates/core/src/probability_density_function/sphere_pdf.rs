use core::f64;

use crate::{ProbabilityDensityFunction, RenderContext, Vector3};

pub struct SpherePdf {}

impl SpherePdf {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SpherePdf {
    fn default() -> Self {
        SpherePdf::new()
    }
}

impl ProbabilityDensityFunction for SpherePdf {
    fn value(&self, _ctx: &RenderContext, _direction: &Vector3) -> f64 {
        1.0 / (4.0 * f64::consts::PI)
    }

    fn generate(&self, ctx: &RenderContext) -> Vector3 {
        Vector3::random_unit(&*ctx.random)
    }
}
