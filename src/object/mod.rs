use crate::{ray::Ray, vector::Vector3};

pub mod sphere;

pub use sphere::Sphere;

pub struct HitRecord {
    pub pt: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
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

pub trait Node {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
