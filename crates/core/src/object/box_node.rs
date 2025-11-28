use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Node, Ray, Vector3,
    material::Material,
    object::{Group, HitRecord, Quad},
};

pub struct Box {
    group: Group,
}

impl Box {
    pub fn new(a: Vector3, b: Vector3, material: Arc<dyn Material>) -> Self {
        let mut group = Group::new();

        let min = Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);

        // front
        group.push(Arc::new(Quad::new(
            Vector3::new(min.x, min.y, max.z),
            dx,
            dy,
            material.clone(),
        )));

        // right
        group.push(Arc::new(Quad::new(
            Vector3::new(max.x, min.y, max.z),
            -dz,
            dy,
            material.clone(),
        )));

        // back
        group.push(Arc::new(Quad::new(
            Vector3::new(max.x, min.y, min.z),
            -dx,
            dy,
            material.clone(),
        )));

        // left
        group.push(Arc::new(Quad::new(
            Vector3::new(min.x, min.y, min.z),
            dz,
            dy,
            material.clone(),
        )));

        // top
        group.push(Arc::new(Quad::new(
            Vector3::new(min.x, max.y, max.z),
            dx,
            -dz,
            material.clone(),
        )));

        // bottom
        group.push(Arc::new(Quad::new(
            Vector3::new(min.x, min.y, min.z),
            dx,
            dz,
            material,
        )));

        Self { group }
    }
}

impl Node for Box {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.group.hit(ray, ray_t)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        self.group.bounding_box()
    }
}
