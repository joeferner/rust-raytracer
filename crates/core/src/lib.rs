pub mod axis;
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

pub use axis::Axis;
pub use axis_aligned_bounding_box::AxisAlignedBoundingBox;
pub use camera::{Camera, CameraBuilder};
pub use color::Color;
pub use image::Image;
pub use interval::Interval;
pub use object::Node;
pub use probability_density_function::{
    CosinePdf, HittablePdf, ProbabilityDensityFunction, SpherePdf,
};
pub use random::{Random, random_new};
pub use ray::Ray;
pub use vector::Vector3;

pub struct RenderContext {
    pub random: Arc<dyn Random>,
}
