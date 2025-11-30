use std::sync::Arc;

use rust_raytracer_core::{
    CameraBuilder, Color, RenderContext, Vector3,
    material::{Dielectric, Lambertian, Metal},
    object::{BoundingVolumeHierarchy, Group, Node, Sphere},
};

use crate::scene::SceneResult;

pub fn create_random_spheres_scene(ctx: &RenderContext) -> SceneResult {
    let mut world: Vec<Arc<dyn Node>> = vec![];

    let ground_material = Arc::new(Lambertian::new_from_color(Color::new(0.5, 0.5, 0.5)));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = ctx.random.rand();
            let center = Vector3::new(
                a as f64 + 0.9 * ctx.random.rand(),
                0.2,
                b as f64 + 0.9 * ctx.random.rand(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&*ctx.random) * Color::random(&*ctx.random);
                    let sphere_material = Arc::new(Lambertian::new_from_color(albedo));
                    let mut sphere = Sphere::new(center, 0.2, sphere_material);
                    sphere.set_direction(Vector3::new(
                        0.0,
                        ctx.random.rand_interval(0.0, 0.5),
                        0.0,
                    ));
                    world.push(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_interval(&*ctx.random, 0.5, 1.0);
                    let fuzz = ctx.random.rand_interval(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
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
    camera_builder.defocus_angle = 0.6;
    camera_builder.focus_distance = 10.0;
    camera_builder.background = Color::new(0.7, 0.8, 1.0);
    let camera = Arc::new(camera_builder.build());

    SceneResult {
        camera,
        world,
        lights: Arc::new(Group::new()),
    }
}
