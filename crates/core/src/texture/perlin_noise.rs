use crate::{Color, Random, Vector3, texture::Texture, utils::Perlin};

#[derive(Debug)]
pub struct PerlinNoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl PerlinNoiseTexture {
    pub fn new(random: &dyn Random, scale: f64) -> Self {
        Self {
            noise: Perlin::new(random),
            scale,
        }
    }
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _u: f64, _v: f64, pt: Vector3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * pt))
    }
}
