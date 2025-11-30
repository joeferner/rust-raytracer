use core::f64;

use crate::{ProbabilityDensityFunction, RenderContext, Vector3, utils::OrthonormalBasis};

pub struct CosinePdf {
    uvw: OrthonormalBasis,
}

impl CosinePdf {
    pub fn new(w: Vector3) -> Self {
        Self {
            uvw: OrthonormalBasis::new(w),
        }
    }
}

impl ProbabilityDensityFunction for CosinePdf {
    fn value(&self, _ctx: &RenderContext, direction: &Vector3) -> f64 {
        let cosine_theta = direction.unit().dot(&self.uvw.w);
        (cosine_theta / f64::consts::PI).max(0.0)
    }

    fn generate(&self, ctx: &RenderContext) -> Vector3 {
        self.uvw
            .transform_to_local(Vector3::random_cosine_direction(&*ctx.random))
    }
}
