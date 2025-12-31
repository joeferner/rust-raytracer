use std::sync::Arc;

use caustic_core::{
    CameraBuilder, Color, Node, RenderContext, Vector3,
    material::Lambertian,
    object::{BoundingVolumeHierarchy, Quad},
};

use crate::scene::SceneData;

pub fn create_quads_scene(_ctx: &RenderContext) -> SceneData {
    // Materials
    let left_red = Arc::new(Lambertian::new_from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.8, 0.8)));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Quad::new(
        Vector3::new(-3.0, -2.0, 5.0),
        Vector3::new(0.0, 0.0, -4.0),
        Vector3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(-2.0, -2.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(3.0, -2.0, 1.0),
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(-2.0, 3.0, 1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(-2.0, -3.0, 5.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 1.0;
    camera_builder.image_width = 400;
    camera_builder.samples_per_pixel = 10;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 80.0;
    camera_builder.look_from = Vector3::new(0.0, 0.0, 9.0);
    camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    camera_builder.background = Color::new(0.7, 0.8, 1.0);
    let camera = Arc::new(camera_builder.build());

    SceneData {
        camera,
        world,
        lights: None,
    }
}
