use crate::{Color, Random, Vector3, texture::Texture, utils::Perlin};

#[derive(Debug)]
pub struct PerlinTurbulenceTexture {
    noise: Perlin,
    scale: f64,
    turbulence_depth: u32,
}

impl PerlinTurbulenceTexture {
    pub fn new(random: &dyn Random, scale: f64, turbulence_depth: u32) -> Self {
        Self {
            noise: Perlin::new(random),
            scale,
            turbulence_depth,
        }
    }
}

#[typetag::serde]
impl Texture for PerlinTurbulenceTexture {
    fn value(&self, _u: f64, _v: f64, pt: Vector3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0
                + (self.scale * pt.z + 10.0 * self.noise.turbulence(pt, self.turbulence_depth))
                    .sin())
    }
}
