#![allow(clippy::vec_init_then_push)]
use std::{env, sync::Arc};

use indicatif::{ProgressBar, ProgressStyle};
use rust_raytracer_core::{
    Camera, Color, RenderContext, Vector3,
    camera::CameraBuilder,
    material::{Lambertian, Metal, Refractive},
    object::{BoundingVolumeHierarchy, Node, Sphere},
    random_new,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut scene = Scene::ThreeSpheres;
    if let Some(scene_name) = args.get(1) {
        scene = if scene_name == "ThreeSpheres" {
            Scene::ThreeSpheres
        } else if scene_name == "RandomSpheres" {
            Scene::RandomSpheres
        } else {
            Scene::ThreeSpheres
        }
    }

    let ctx = RenderContext {
        random: &random_new(),
    };

    let (camera, world) = get_scene(&ctx, scene);

    // Setup progress bar
    let pb = ProgressBar::new(camera.image_height() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap(),
    );

    // render image
    let mut img: image::ImageBuffer<
        image::Rgb<u8>,
        Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>,
    > = image::ImageBuffer::new(camera.image_width(), camera.image_height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                let pixel_color = camera.render(&ctx, x, y, &*world);
                *pixel = color_to_image_rgb(pixel_color);
            }
        }
        pb.inc(1);
    }

    img.save("../../target/out.png").unwrap();
    pb.finish_with_message("Done!");
}

fn color_to_image_rgb(color: Color) -> image::Rgb<u8> {
    let r = (color.r * 255.999) as u8;
    let g = (color.g * 255.999) as u8;
    let b = (color.b * 255.999) as u8;
    image::Rgb([r, g, b])
}

enum Scene {
    ThreeSpheres,
    RandomSpheres,
}

fn get_scene(ctx: &RenderContext, scene: Scene) -> (Camera, Arc<dyn Node>) {
    match scene {
        Scene::ThreeSpheres => get_scene_three_spheres(ctx),
        Scene::RandomSpheres => get_scene_random_spheres(ctx),
    }
}

fn get_scene_three_spheres(_ctx: &RenderContext) -> (Camera, Arc<dyn Node>) {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
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
    camera_builder.samples_per_pixel = 50;
    camera_builder.max_depth = 50;
    camera_builder.defocus_angle = 0.6;
    camera_builder.focus_distance = 1.0;
    let camera = camera_builder.build();

    (camera, world)
}

fn get_scene_random_spheres(ctx: &RenderContext) -> (Camera, Arc<dyn Node>) {
    let mut world: Vec<Arc<dyn Node>> = vec![];

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
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
                    let albedo = Color::random(ctx.random) * Color::random(ctx.random);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let mut sphere = Sphere::new(center, 0.2, sphere_material);
                    sphere.set_direction(Vector3::new(
                        0.0,
                        ctx.random.rand_interval(0.0, 0.5),
                        0.0,
                    ));
                    world.push(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_interval(ctx.random, 0.5, 1.0);
                    let fuzz = ctx.random.rand_interval(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Refractive::new(1.5));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Refractive::new(1.5));
    world.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
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
    let camera = camera_builder.build();

    (camera, world)
}
