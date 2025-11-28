use std::sync::Arc;

use rust_raytracer_core::{
    Camera, Color, Node, RenderContext, Vector3,
    camera::CameraBuilder,
    material::{DiffuseLight, Lambertian},
    object::{BoundingVolumeHierarchy, Box, Quad},
};

pub fn create_cornell_box_scene(_ctx: &RenderContext) -> (Arc<Camera>, Arc<dyn Node>) {
    let red_material = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white_material = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green_material = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let light_material = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Quad::new(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green_material,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red_material,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light_material,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white_material.clone(),
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white_material.clone(),
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white_material.clone(),
    )));

    world.push(Arc::new(Box::new(
        Vector3::new(130.0, 0.0, 65.0),
        Vector3::new(295.0, 165.0, 230.0),
        white_material.clone(),
    )));

    world.push(Arc::new(Box::new(
        Vector3::new(265.0, 0.0, 295.0),
        Vector3::new(430.0, 330.0, 460.0),
        white_material,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 1.0;
    camera_builder.image_width = 400;
    camera_builder.samples_per_pixel = 50;
    camera_builder.max_depth = 10;
    camera_builder.vertical_fov = 40.0;
    camera_builder.look_from = Vector3::new(278.0, 278.0, -800.0);
    camera_builder.look_at = Vector3::new(278.0, 278.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    let camera = Arc::new(camera_builder.build());

    (camera, world)
}
