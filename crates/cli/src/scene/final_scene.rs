use std::sync::Arc;

use rust_raytracer_core::{
    Camera, Color, RenderContext, Vector3,
    camera::CameraBuilder,
    image::ImageImage,
    material::{DiffuseLight, Lambertian, Metal, Refractive},
    object::{
        BoundingVolumeHierarchy, Box, ConstantMedium, Node, Quad, RotateY, Sphere, Translate,
    },
    texture::{ImageTexture, PerlinNoiseTexture},
};

pub fn create_final_scene(ctx: &RenderContext) -> (Arc<Camera>, Arc<dyn Node>) {
    let mut world: Vec<Arc<dyn Node>> = vec![];

    // ground
    let ground_material = Arc::new(Lambertian::new_from_color(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = ctx.random.rand_interval(1.0, 101.0);
            let z1 = z0 + w;

            world.push(Arc::new(Box::new(
                Vector3::new(x0, y0, z0),
                Vector3::new(x1, y1, z1),
                ground_material.clone(),
            )));
        }
    }

    // light
    let light_material = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));
    world.push(Arc::new(Quad::new(
        Vector3::new(123.0, 554.0, 147.0),
        Vector3::new(300.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 265.0),
        light_material,
    )));

    // moving sphere, top left
    let center1 = Vector3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new_from_color(Color::new(0.7, 0.3, 0.1)));
    let mut sphere = Sphere::new(center1, 50.0, sphere_material);
    sphere.set_direction(center2 - center1);
    world.push(Arc::new(sphere));

    // glass sphere bottom middle
    world.push(Arc::new(Sphere::new(
        Vector3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Refractive::new(1.5)),
    )));

    // metal sphere bottom right
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // blue sphere left
    let boundary = Arc::new(Sphere::new(
        Vector3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Refractive::new(1.5)),
    ));
    world.push(boundary.clone());
    world.push(Arc::new(ConstantMedium::new_from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // atmosphere everywhere
    let boundary = Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Refractive::new(1.5)),
    ));
    world.push(Arc::new(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // earth left
    let earth_image = ImageImage::load_file("assets/earth-map.jpg").unwrap();
    let earth_texture = Arc::new(ImageTexture::new(earth_image));
    let earth_material = Arc::new(Lambertian::new(earth_texture));
    world.push(Arc::new(Sphere::new(
        Vector3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));

    // noise sphere middle
    let perlin_texture = Arc::new(PerlinNoiseTexture::new(&*ctx.random, 0.2));
    world.push(Arc::new(Sphere::new(
        Vector3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    // box made of spheres middle right
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    let mut boxes2: Vec<Arc<dyn Node>> = vec![];
    for _ in 0..ns {
        boxes2.push(Arc::new(Sphere::new(
            Vector3::random_interval(&*ctx.random, 0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BoundingVolumeHierarchy::new(&boxes2)),
            15.0,
        )),
        Vector3::new(-100.0, 270.0, 395.0),
    )));

    // world
    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Camera
    // let image_width = 400;
    // let samples_per_pixel = 500;
    // let max_depth = 10;

    let image_width = 800;
    let samples_per_pixel = 5000;
    let max_depth = 40;

    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 1.0;
    camera_builder.image_width = image_width;
    camera_builder.samples_per_pixel = samples_per_pixel;
    camera_builder.max_depth = max_depth;
    camera_builder.vertical_fov = 40.0;
    camera_builder.look_from = Vector3::new(478.0, 278.0, -600.0);
    camera_builder.look_at = Vector3::new(278.0, 278.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.defocus_angle = 0.0;
    let camera = Arc::new(camera_builder.build());

    (camera, world)
}
