#![allow(clippy::vec_init_then_push)]

pub mod scene;

use std::{
    env,
    sync::{Arc, Mutex, mpsc},
    thread,
};

use indicatif::{ProgressBar, ProgressStyle};
use rust_raytracer_core::{Camera, Color, Node, RenderContext, random_new};
use scene::Scene;

use crate::scene::get_scene;

const BLOCK_SIZE: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut scene = Scene::CornellBox;
    if let Some(scene_name) = args.get(1) {
        scene = if scene_name == "ThreeSpheres" {
            Scene::ThreeSpheres
        } else if scene_name == "RandomSpheres" {
            Scene::RandomSpheres
        } else if scene_name == "CheckeredSpheres" {
            Scene::CheckeredSpheres
        } else if scene_name == "Earth" {
            Scene::Earth
        } else if scene_name == "PerlinSpheres" {
            Scene::PerlinSpheres
        } else if scene_name == "Quads" {
            Scene::Quads
        } else if scene_name == "SimpleLight" {
            Scene::SimpleLight
        } else {
            panic!("invalid scene name")
        }
    }

    let ctx = Arc::new(RenderContext {
        random: random_new(),
    });

    let (camera, world) = get_scene(&ctx, scene);

    // render image
    let mut img: image::ImageBuffer<
        image::Rgb<u8>,
        Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>,
    > = image::ImageBuffer::new(camera.image_width(), camera.image_height());

    // generate work
    let mut work: Vec<Work> = vec![];
    let mut y = 0;
    loop {
        let mut x = 0;
        loop {
            work.push(Work {
                camera: camera.clone(),
                world: world.clone(),
                xmin: x,
                xmax: (x + BLOCK_SIZE).min(img.width()),
                ymin: y,
                ymax: (y + BLOCK_SIZE).min(img.height()),
            });
            if x > img.width() {
                break;
            }
            x += BLOCK_SIZE;
        }
        if y > img.height() {
            break;
        }
        y += BLOCK_SIZE;
    }
    let work_count = work.len();

    // Setup progress bar
    let pb = ProgressBar::new(work_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap(),
    );

    // start work
    let threads = num_cpus::get();
    let work = Arc::new(Mutex::new(work));
    let (results_send, results_recv) = mpsc::channel();
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let work = work.clone();
        let results_send = results_send.clone();
        let ctx = ctx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let item = { work.lock().unwrap().pop() };
                match item {
                    Some(item) => {
                        let mut pixels = vec![];
                        for y in item.ymin..item.ymax {
                            for x in item.xmin..item.xmax {
                                let pixel_color = item.camera.render(&ctx, x, y, &*item.world);
                                pixels.push(pixel_color);
                            }
                        }
                        results_send
                            .send(WorkResult {
                                xmin: item.xmin,
                                xmax: item.xmax,
                                ymin: item.ymin,
                                ymax: item.ymax,
                                pixels,
                            })
                            .unwrap();
                    }
                    None => break,
                }
            }
        }));
    }

    for _ in 0..work_count {
        let result = results_recv.recv().unwrap();
        let mut i = 0;
        for y in result.ymin..result.ymax {
            for x in result.xmin..result.xmax {
                if let Some(pixel) = img.get_pixel_mut_checked(x, y) {
                    let pixel_color = result.pixels[i];
                    *pixel = color_to_image_rgb(pixel_color);
                    i += 1;
                }
            }
        }
        pb.inc(1);
    }

    for h in handles {
        h.join().unwrap();
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

pub struct Work {
    pub camera: Arc<Camera>,
    pub world: Arc<dyn Node>,
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
}

pub struct WorkResult {
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
    pub pixels: Vec<Color>,
}
