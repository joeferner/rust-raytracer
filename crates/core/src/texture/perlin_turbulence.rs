use crate::{Color, Random, Vector3, texture::Texture, utils::Perlin};

#[derive(Debug)]
pub struct PerlinTurbulenceTexture {
    noise: Perlin,
    depth: u32,
}

impl PerlinTurbulenceTexture {
    pub fn new(random: &dyn Random, depth: u32) -> Self {
        Self {
            noise: Perlin::new(random),
            depth,
        }
    }
}

impl Texture for PerlinTurbulenceTexture {
    fn value(&self, _u: f64, _v: f64, pt: Vector3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.turbulence(pt, self.depth)
    }
}
