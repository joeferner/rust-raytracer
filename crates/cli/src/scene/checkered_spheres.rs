use std::sync::Arc;

use rust_raytracer_core::{
    CameraBuilder, Color, RenderContext, Vector3,
    material::Lambertian,
    object::{BoundingVolumeHierarchy, Node, Sphere},
    texture::{CheckerTexture, SolidColor},
};

use crate::scene::SceneResult;

pub fn create_checkered_spheres_scene(_ctx: &RenderContext) -> SceneResult {
    let checker = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
        0.32,
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ))));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 10.0, 0.0),
        10.0,
        checker,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 16.0 / 9.0;
    camera_builder.image_width = 300;
    camera_builder.samples_per_pixel = 10;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 20.0;
    camera_builder.look_from = Vector3::new(13.0, 2.0, 3.0);
    camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    camera_builder.background = Color::new(0.7, 0.8, 1.0);
    let camera = Arc::new(camera_builder.build());

    SceneResult {
        camera,
        world,
        lights: None,
    }
}
