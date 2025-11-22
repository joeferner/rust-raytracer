pub mod camera;
pub mod color;
pub mod interval;
pub mod object;
pub mod ray;
pub mod vector;

pub use camera::Camera;
pub use color::Color;
pub use interval::Interval;
pub use ray::Ray;
pub use vector::Vector3;

pub trait Random {
    fn rand(&self) -> f64;
    fn rand_interval(&self, min: f64, max: f64) -> f64;
}

pub struct RenderContext<'a> {
    pub random: &'a dyn Random,
}
