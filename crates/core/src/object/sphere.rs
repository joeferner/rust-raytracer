use std::sync::Arc;

use crate::{
    AxisAlignedBoundingBox, Interval, Vector3,
    material::Material,
    object::{HitRecord, Node},
    ray::Ray,
};

#[derive(Debug)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    pub material: Arc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material>) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);
        Self {
            center: Ray::new(center, Vector3::ZERO),
            radius,
            material,
            bbox: AxisAlignedBoundingBox::new_from_points(center - radius_vec, center + radius_vec),
        }
    }

    pub fn set_direction(&mut self, direction: Vector3) {
        self.center = Ray::new(self.center.origin, direction);
        self.update_bbox();
    }

    fn update_bbox(&mut self) {
        let rvec = Vector3::new(self.radius, self.radius, self.radius);
        let box1 = AxisAlignedBoundingBox::new_from_points(
            self.center.at(0.0) - rvec,
            self.center.at(0.0) + rvec,
        );
        let box2 = AxisAlignedBoundingBox::new_from_points(
            self.center.at(1.0) - rvec,
            self.center.at(1.0) + rvec,
        );
        self.bbox = AxisAlignedBoundingBox::new_from_bbox(box1, box2);
    }
}

impl Node for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let pt = ray.at(t);
        let outward_normal = (pt - current_center) / self.radius;
        let mut rec = HitRecord {
            pt,
            normal: Vector3::ZERO, // set by set_face_normal
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: self.material.clone(),
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
