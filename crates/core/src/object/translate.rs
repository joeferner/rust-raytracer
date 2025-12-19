use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Node, Ray, RenderContext, Vector3, object::HitRecord,
};

#[derive(Debug)]
pub struct Translate {
    object: Arc<dyn Node>,
    offset: Vector3,
    bbox: AxisAlignedBoundingBox,
}

impl Translate {
    pub fn new(object: Arc<dyn Node>, offset: Vector3) -> Self {
        let bbox = *object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Node for Translate {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_with_time(ray.origin - self.offset, ray.direction, ray.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        let mut hit = self.object.hit(ctx, &offset_r, ray_t)?;

        // Move the intersection point forwards by the offset
        hit.pt = hit.pt + self.offset;

        Some(hit)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
