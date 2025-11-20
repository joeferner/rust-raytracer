use std::fmt::{Debug, Display};

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    #[test]
    fn test_length_squared() {
        let v = Vector3 {
            x: 3.0,
            y: 4.0,
            z: 12.0,
        };
        assert!(approx_eq(v.length_squared(), 169.0)); // 3^2 + 4^2 + 12^2 = 169
    }

    #[test]
    fn test_length() {
        let v = Vector3 {
            x: 3.0,
            y: 4.0,
            z: 12.0,
        };
        assert!(approx_eq(v.length(), 13.0)); // sqrt(169)
    }

    #[test]
    fn test_dot_product() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vector3 {
            x: 4.0,
            y: -5.0,
            z: 6.0,
        };

        let dot = a.dot(&b); // 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert!(approx_eq(dot, 12.0));
    }

    #[test]
    fn test_cross_product() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let c = a.cross(&b);

        assert!(approx_eq(c.x, -3.0)); // 2*6 - 3*5 = -3
        assert!(approx_eq(c.y, 6.0)); // 3*4 - 1*6 = 6
        assert!(approx_eq(c.z, -3.0)); // 1*5 - 2*4 = -3
    }

    #[test]
    fn test_cross_product_perpendicular() {
        // Cross product of parallel vectors = zero vector
        let a = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let b = Vector3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };

        let c = a.cross(&b);

        assert!(approx_eq(c.x, 0.0));
        assert!(approx_eq(c.y, 0.0));
        assert!(approx_eq(c.z, 0.0));
    }

    #[test]
    fn test_cross_product_orientation() {
        // Standard basis: i × j = k
        let i = Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let j = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let k = i.cross(&j);

        assert!(approx_eq(k.x, 0.0));
        assert!(approx_eq(k.y, 0.0));
        assert!(approx_eq(k.z, 1.0));
    }
}
