use std::sync::Arc;

use crate::object::{HitRecord, Node};

pub struct Group {
    nodes: Vec<Arc<dyn Node>>,
}

impl Group {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn push(&mut self, node: Arc<dyn Node>) {
        self.nodes.push(node);
    }
}

impl Node for Group {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_t = ray_tmax;
        let mut closest_hit: Option<HitRecord> = None;

        for node in &self.nodes {
            if let Some(hit) = node.hit(ray, ray_tmin, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
