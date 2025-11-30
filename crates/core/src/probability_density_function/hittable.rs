use std::sync::Arc;

use crate::{Node, ProbabilityDensityFunction, RenderContext, Vector3};

pub struct HittablePdf {
    objects: Arc<dyn Node>,
    origin: Vector3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Node>, origin: Vector3) -> Self {
        Self { objects, origin }
    }
}

impl ProbabilityDensityFunction for HittablePdf {
    fn value(&self, ctx: &RenderContext, direction: &Vector3) -> f64 {
        self.objects.pdf_value(ctx, &self.origin, direction)
    }

    fn generate(&self, ctx: &RenderContext) -> Vector3 {
        self.objects.random(ctx, &self.origin)
    }
}
