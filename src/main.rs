use core::f64;

use image;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    color::Color,
    object::{Node, Sphere},
    ray::Ray,
    vector::Vector3,
};

pub mod color;
pub mod object;
pub mod ray;
pub mod vector;

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height: u32 = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Setup progress bar
    let pb = ProgressBar::new(image_height as u64);
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
    > = image::ImageBuffer::new(image_width, image_height);

    for y in 0..img.height() {
        for x in 0..img.width() {
            if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                let pixel_center =
                    pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;
                let r = Ray::new(camera_center, ray_direction);
                let pixel_color = ray_color(r);
                *pixel = pixel_color.into();
            }
        }
        pb.inc(1);
    }

    img.save("target/out.png").unwrap();
    pb.finish_with_message("Done!");
}

fn ray_color(ray: Ray) -> Color {
    let sphere = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    if let Some(rec) = sphere.hit(&ray, 0.0, f64::INFINITY) {
        let n = (ray.at(rec.t) - Vector3::new(0.0, 0.0, -1.0)).unit();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
