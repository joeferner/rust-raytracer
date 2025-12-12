use std::{cmp::Ordering, sync::Arc};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    Axis, AxisAlignedBoundingBox, Interval, Ray, RenderContext,
    object::{Group, HitRecord, Node},
};

pub struct BoundingVolumeHierarchy {
    left: Arc<dyn Node>,
    right: Arc<dyn Node>,
    bbox: AxisAlignedBoundingBox,
}

impl BoundingVolumeHierarchy {
    pub fn new(nodes: &[Arc<dyn Node>]) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = AxisAlignedBoundingBox::new();
        for obj in nodes {
            bbox = AxisAlignedBoundingBox::new_from_bbox(bbox, *obj.bounding_box());
        }

        let (left, right) = if nodes.is_empty() {
            let left: Arc<dyn Node> = Arc::new(Group::new());
            let right: Arc<dyn Node> = Arc::new(Group::new());
            (left, right)
        } else if nodes.len() == 1 {
            (nodes[0].clone(), nodes[0].clone())
        } else if nodes.len() == 2 {
            (nodes[0].clone(), nodes[1].clone())
        } else {
            let axis = bbox.longest_axis();

            let mut nodes = nodes.to_vec();
            nodes.sort_by(|a, b| Self::bbox_compare(a, b, axis));

            let mid = nodes.len() / 2;
            let left: Arc<dyn Node> = Arc::new(BoundingVolumeHierarchy::new(&nodes[..mid]));
            let right: Arc<dyn Node> = Arc::new(BoundingVolumeHierarchy::new(&nodes[mid..]));
            (left, right)
        };

        let bbox =
            AxisAlignedBoundingBox::new_from_bbox(*left.bounding_box(), *right.bounding_box());
        Self { left, right, bbox }
    }

    fn bbox_compare(a: &Arc<dyn Node>, b: &Arc<dyn Node>, axis: Axis) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
}

#[typetag::serde]
impl Node for BoundingVolumeHierarchy {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ctx, ray, ray_t);

        // check to see if right is closer
        let mut t = ray_t.max;
        if let Some(hit_left) = &hit_left {
            t = hit_left.t;
        }
        let hit_right = self.right.hit(ctx, ray, Interval::new(ray_t.min, t));
        if hit_right.is_some() {
            return hit_right;
        }

        hit_left
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}

#[derive(Serialize, Deserialize)]
struct BoundingVolumeHierarchySerde {
    left: Arc<dyn Node>,
    right: Arc<dyn Node>,
}

impl Serialize for BoundingVolumeHierarchy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let builder = BoundingVolumeHierarchySerde {
            left: self.left.clone(),
            right: self.right.clone(),
        };

        builder.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BoundingVolumeHierarchy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serde = BoundingVolumeHierarchySerde::deserialize(deserializer)?;
        Ok(BoundingVolumeHierarchy::new(&[serde.left, serde.right]))
    }
}
