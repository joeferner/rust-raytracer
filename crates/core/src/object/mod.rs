use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, RenderContext, material::Material, ray::Ray, vector::Vector3,
};

pub mod bounding_volume_hierarchy;
pub mod box_node;
pub mod constant_medium;
pub mod group;
pub mod quad;
pub mod rotate_y;
pub mod sphere;
pub mod translate;

pub use bounding_volume_hierarchy::BoundingVolumeHierarchy;
pub use box_node::Box;
pub use constant_medium::ConstantMedium;
pub use group::Group;
pub use quad::Quad;
pub use rotate_y::RotateY;
pub use sphere::Sphere;
pub use translate::Translate;

pub struct HitRecord {
    pub pt: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Node: Send + Sync {
    fn hit(&self, ctx: &RenderContext, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AxisAlignedBoundingBox;
}
