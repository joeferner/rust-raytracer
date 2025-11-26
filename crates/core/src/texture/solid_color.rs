use crate::{Color, texture::Texture};

#[derive(Debug)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _pt: crate::Vector3) -> crate::Color {
        self.albedo
    }
}
