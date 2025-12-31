use std::sync::Arc;

use caustic_core::{
    CameraBuilder, Color, Node, RenderContext, Vector3,
    material::Lambertian,
    object::{BoundingVolumeHierarchy, Sphere},
    texture::{PerlinNoiseTexture, PerlinTurbulenceTexture},
};

use crate::scene::SceneData;

pub fn create_perlin_spheres_scene(ctx: &RenderContext) -> SceneData {
    let texture_perlin_noise = Arc::new(PerlinNoiseTexture::new(&*ctx.random, 4.0));
    let material_perlin_noise = Arc::new(Lambertian::new(texture_perlin_noise));

    let texture_perlin_turbulence = Arc::new(PerlinTurbulenceTexture::new(&*ctx.random, 4.0, 7));
    let material_perlin_turbulence = Arc::new(Lambertian::new(texture_perlin_turbulence));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_perlin_noise.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 2.0, -2.0),
        2.0,
        material_perlin_noise,
    )));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 2.0, 2.0),
        2.0,
        material_perlin_turbulence,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 16.0 / 9.0;
    camera_builder.image_width = 400;
    camera_builder.samples_per_pixel = 10;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 20.0;
    camera_builder.look_from = Vector3::new(15.0, 2.0, 3.0);
    camera_builder.look_at = Vector3::new(0.0, 1.5, 0.0);
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
