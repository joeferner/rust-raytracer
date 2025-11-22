use std::sync::Arc;

use indicatif::{ProgressBar, ProgressStyle};
use rust_raytracer_core::{
    Camera, Color, Random, RenderContext, Vector3,
    material::{Lambertian, Metal},
    object::{Group, Sphere},
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_height = 600;

    let ctx = RenderContext {
        random: &RandRandom::new(),
    };

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut group = Group::new();
    group.push(Arc::new(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));
    group.push(Arc::new(Sphere {
        center: Vector3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: material_center,
    }));
    group.push(Arc::new(Sphere {
        center: Vector3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left,
    }));
    group.push(Arc::new(Sphere {
        center: Vector3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    }));

    // Camera
    let camera = Camera::new(aspect_ratio, image_height);

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
                let pixel_color = camera.render(&ctx, x, y, &group);
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

pub struct RandRandom {}

impl RandRandom {
    fn new() -> Self {
        Self {}
    }
}

impl Random for RandRandom {
    fn rand(&self) -> f64 {
        rand::random()
    }

    fn rand_interval(&self, min: f64, max: f64) -> f64 {
        rand::random_range(min..max)
    }
}
