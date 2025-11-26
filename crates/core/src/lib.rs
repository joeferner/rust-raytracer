pub mod axis_aligned_bounding_box;
pub mod camera;
pub mod color;
pub mod interval;
pub mod material;
pub mod object;
pub mod ray;
pub mod vector;

pub use axis_aligned_bounding_box::AxisAlignedBoundingBox;
pub use camera::Camera;
pub use color::Color;
pub use interval::Interval;
pub use ray::Ray;
pub use vector::Vector3;

pub trait Random {
    fn rand(&self) -> f64;
    fn rand_int_interval(&self, min: i64, max: i64) -> i64;
    fn rand_interval(&self, min: f64, max: f64) -> f64;
}

pub struct RenderContext<'a> {
    pub random: &'a dyn Random,
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
