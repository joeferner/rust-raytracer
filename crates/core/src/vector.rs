use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::RenderContext;

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn random(ctx: &RenderContext) -> Self {
        Vector3::new(ctx.random.rand(), ctx.random.rand(), ctx.random.rand())
    }

    pub fn random_interval(ctx: &RenderContext, min: f64, max: f64) -> Self {
        Vector3::new(
            ctx.random.rand_interval(min, max),
            ctx.random.rand_interval(min, max),
            ctx.random.rand_interval(min, max),
        )
    }

    pub fn random_unit(ctx: &RenderContext) -> Self {
        loop {
            let p = Self::random_interval(ctx, -1.0, 1.0);
            let len_sq = p.length_squared();
            if 1e-160 < len_sq && len_sq <= 1.0 {
                return p / len_sq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(ctx: &RenderContext, normal: Vector3) -> Self {
        let on_unit_sphere = Self::random_unit(ctx);
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

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

    pub fn unit(&self) -> Vector3 {
        *self / self.length()
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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
