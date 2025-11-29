use crate::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: Vector3, direction: Vector3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (t * self.direction)
    }
}
