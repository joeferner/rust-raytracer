use std::fmt::Debug;

use crate::{Color, Vector3};

pub mod checker_texture;
pub mod image_texture;
pub mod perlin_noise;
pub mod perlin_turbulence;
pub mod solid_color;

pub use checker_texture::CheckerTexture;
pub use image_texture::ImageTexture;
pub use perlin_noise::PerlinNoiseTexture;
pub use perlin_turbulence::PerlinTurbulenceTexture;
pub use solid_color::SolidColor;

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, pt: Vector3) -> Color;
}
