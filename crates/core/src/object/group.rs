use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Ray, RenderContext,
    object::{HitRecord, Node},
};

pub struct Group {
    nodes: Vec<Arc<dyn Node>>,
    bbox: AxisAlignedBoundingBox,
}

impl Group {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            bbox: AxisAlignedBoundingBox::new(),
        }
    }

    pub fn from_list(nodes: &[Arc<dyn Node>]) -> Self {
        let mut results = Self::new();
        for node in nodes {
            results.push(node.clone());
        }
        results
    }

    pub fn push(&mut self, node: Arc<dyn Node>) {
        let node_bbox = *node.bounding_box();
        self.nodes.push(node);
        self.bbox = AxisAlignedBoundingBox::new_from_bbox(self.bbox, node_bbox);
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Group {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;

        for node in &self.nodes {
            if let Some(hit) = node.hit(ctx, ray, ray_t) {
                ray_t.max = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
