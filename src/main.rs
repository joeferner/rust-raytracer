use indicatif::{ProgressBar, ProgressStyle};

pub mod vector;

fn main() {
    let pb = ProgressBar::new(800 * 600);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap(),
    );

    let mut img_buf = image::ImageBuffer::new(800, 600);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
        pb.inc(1);
    }

    let pixel = img_buf.get_pixel_mut(10, 10);
    *pixel = image::Rgb([255, 255, 255]);

    img_buf.save("target/out.png").unwrap();
    pb.finish_with_message("Done!");
}
