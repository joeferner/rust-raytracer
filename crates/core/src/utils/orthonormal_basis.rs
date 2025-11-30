use crate::Vector3;

pub struct OrthonormalBasis {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl OrthonormalBasis {
    pub fn new(normal: Vector3) -> Self {
        let w = normal.unit();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).unit();
        let u = w.cross(&v);

        Self { u, v, w }
    }

    /// Transform from basis coordinates to local space.
    pub fn transform_to_local(&self, v: Vector3) -> Vector3 {
        (v.x * self.u) + (v.y * self.v) + (v.z * self.w)
    }
}
