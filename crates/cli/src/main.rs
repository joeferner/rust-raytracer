use std::sync::Arc;

use image;
use indicatif::{ProgressBar, ProgressStyle};
use rust_raytracer_core::{
    Camera, Color, Vector3, object::{Group, Sphere},
};

fn main() {
    // World
    let mut group = Group::new();
    group.push(Arc::new(Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    group.push(Arc::new(Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400);

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
                let pixel_color = camera.render(x, y, &group);
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
