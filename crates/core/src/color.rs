use std::ops::{Add, AddAssign, Div, Mul};

use crate::Random;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn random(random: &dyn Random) -> Self {
        Self {
            r: random.rand(),
            g: random.rand(),
            b: random.rand(),
        }
    }

    pub fn random_interval(random: &dyn Random, from: f64, to: f64) -> Self {
        Self {
            r: random.rand_interval(from, to),
            g: random.rand_interval(from, to),
            b: random.rand_interval(from, to),
        }
    }

    pub fn linear_to_gamma(&self) -> Self {
        Self {
            r: linear_to_gamma(self.r).clamp(0.0, 0.999),
            g: linear_to_gamma(self.g).clamp(0.0, 0.999),
            b: linear_to_gamma(self.b).clamp(0.0, 0.999),
        }
    }

    pub fn nan_to_zero(&self) -> Color {
        Color {
            r: if self.r.is_nan() { 0.0 } else { self.r },
            g: if self.g.is_nan() { 0.0 } else { self.g },
            b: if self.b.is_nan() { 0.0 } else { self.b },
        }
    }
}

fn linear_to_gamma(v: f64) -> f64 {
    if v > 0.0 { v.sqrt() } else { 0.0 }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, v: Color) -> Color {
        Color {
            r: self * v.r,
            g: self * v.g,
            b: self * v.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
    }
}
