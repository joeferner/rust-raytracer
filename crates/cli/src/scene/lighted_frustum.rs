use std::sync::Arc;

use rust_raytracer_core::{
    CameraBuilder, Color, RenderContext, Vector3,
    material::{DiffuseLight, Lambertian},
    object::{BoundingVolumeHierarchy, Frustum, Node, Quad, Sphere},
    texture::PerlinTurbulenceTexture,
};

use crate::scene::SceneData;

pub fn create_lighted_frustum_scene(ctx: &RenderContext) -> SceneData {
    // Material
    let perlin_texture = Arc::new(PerlinTurbulenceTexture::new(&*ctx.random, 4.0, 7));
    let perlin_material = Arc::new(Lambertian::new(perlin_texture));

    let diffuse_light_white = Arc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    let diffuse_light_blue = Arc::new(DiffuseLight::new_from_color(Color::new(0.0, 0.0, 2.0)));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));

    world.push(Arc::new(Frustum::new(
        Vector3::new(0.0, 1.5, 0.0),
        2.0,
        1.0,
        2.0,
        perlin_material,
    )));

    world.push(Arc::new(Quad::new(
        Vector3::new(3.0, 1.0, -2.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
        diffuse_light_white.clone(),
    )));

    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light_blue,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 16.0 / 9.0;
    camera_builder.image_width = 400;
    camera_builder.samples_per_pixel = 50;
    camera_builder.max_depth = 50;
    camera_builder.defocus_angle = 0.0;
    camera_builder.vertical_fov = 20.0;
    camera_builder.look_from = Vector3::new(26.0, 6.0, 6.0);
    camera_builder.look_at = Vector3::new(0.0, 2.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    camera_builder.background = Color::new(0.0, 0.0, 0.0);
    let camera = Arc::new(camera_builder.build());

    SceneData {
        camera,
        world,
        lights: None,
    }
}
