use std::sync::Arc;

use caustic_core::{
    CameraBuilder, Color, RenderContext, Vector3, image::ImageImage, material::Lambertian,
    object::Sphere, texture::ImageTexture,
};

use crate::scene::SceneData;

pub fn create_earth_scene(_ctx: &RenderContext) -> SceneData {
    let image = ImageImage::load_file("assets/earth-map.jpg").unwrap();
    let earth_texture = Arc::new(ImageTexture::new(image));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 16.0 / 9.0;
    camera_builder.image_width = 300;
    camera_builder.samples_per_pixel = 10;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 20.0;
    camera_builder.look_from = Vector3::new(0.0, 0.0, 12.0);
    camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    camera_builder.background = Color::new(0.7, 0.8, 1.0);
    let camera = Arc::new(camera_builder.build());

    SceneData {
        camera,
        world: globe,
        lights: None,
    }
}
