use std::fmt::Debug;

use crate::{Color, Vector3};

pub mod checker_texture;
pub mod solid_color;

pub use checker_texture::CheckerTexture;
pub use solid_color::SolidColor;

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, pt: Vector3) -> Color;
}
