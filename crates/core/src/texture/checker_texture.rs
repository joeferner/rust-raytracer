use std::sync::Arc;

use crate::texture::Texture;

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, pt: crate::Vector3) -> crate::Color {
        let x_integer = (self.inv_scale * pt.x).floor() as i64;
        let y_integer = (self.inv_scale * pt.y).floor() as i64;
        let z_integer = (self.inv_scale * pt.z).floor() as i64;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, pt)
        } else {
            self.odd.value(u, v, pt)
        }
    }
}
