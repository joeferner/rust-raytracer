use std::sync::Arc;

use crate::{Color, Image, Vector3, texture::Texture};

#[derive(Debug)]
pub struct ImageTexture {
    image: Arc<dyn Image>,
}

impl ImageTexture {
    pub fn new(image: Arc<dyn Image>) -> Self {
        Self { image }
    }
}

#[typetag::serde]
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _pt: Vector3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        if let Some(color) = self.image.get_pixel(i, j) {
            color
        } else {
            Color::new(0.0, 1.0, 1.0)
        }
    }
}
