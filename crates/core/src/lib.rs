pub mod axis_aligned_bounding_box;
pub mod camera;
pub mod color;
pub mod image;
pub mod interval;
pub mod material;
pub mod object;
pub mod probability_density_function;
pub mod random;
pub mod ray;
pub mod texture;
pub mod utils;
pub mod vector;

use std::sync::Arc;

pub use axis_aligned_bounding_box::AxisAlignedBoundingBox;
pub use camera::Camera;
pub use color::Color;
pub use image::Image;
pub use interval::Interval;
pub use object::Node;
pub use probability_density_function::{
    CosinePdf, HitTablePdf, ProbabilityDensityFunction, SpherePdf,
};
pub use random::{Random, random_new};
pub use ray::Ray;
pub use vector::Vector3;

pub struct RenderContext {
    pub random: Arc<dyn Random>,
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn iter() -> impl Iterator<Item = Axis> {
        static AXIS: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
        AXIS.iter().copied() // .copied() is used to iterate over values, not references
    }
}
