use std::sync::Arc;

use rust_raytracer_core::{
    Camera, Color, RenderContext, Vector3,
    camera::CameraBuilder,
    material::{Lambertian, Metal, Refractive},
    object::{BoundingVolumeHierarchy, Node, Sphere},
    texture::{CheckerTexture, SolidColor},
};

pub fn get_scene_three_spheres(_ctx: &RenderContext) -> (Arc<Camera>, Arc<dyn Node>) {
    let material_ground = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
        0.32,
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ))));
    let material_center = Arc::new(Lambertian::new_from_color(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Refractive::new(1.5));
    let material_bubble = Arc::new(Refractive::new(1.0 / 1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 16.0 / 9.0;
    camera_builder.image_width = 300;
    camera_builder.samples_per_pixel = 100;
    camera_builder.max_depth = 50;
    camera_builder.defocus_angle = 0.6;
    camera_builder.focus_distance = 1.0;
    let camera = Arc::new(camera_builder.build());

    (camera, world)
}
