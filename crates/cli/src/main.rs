use thread_priority::ThreadBuilderExt;
use thread_priority::*;

pub mod scene;

use std::{
    env,
    process::ExitCode,
    sync::{Arc, Mutex, mpsc},
};

use indicatif::{ProgressBar, ProgressStyle};
use rust_raytracer_core::{Camera, Color, Node, RenderContext, random_new};
use scene::Scene;

use crate::scene::get_scene;

const BLOCK_SIZE: u32 = 10;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let mut scene = Scene::ThreeSpheres;
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
        } else if scene_name == "CornellBox" {
            Scene::CornellBox
        } else if scene_name == "CornellBoxSmoke" {
            Scene::CornellBoxSmoke
        } else if scene_name == "Final" {
            Scene::Final
        } else if scene_name.to_lowercase().ends_with(".scad") {
            Scene::OpenScad(scene_name.to_owned())
        } else {
            eprintln!("invalid scene name: {scene_name}");
            return ExitCode::from(1);
        }
    }

    let ctx = Arc::new(RenderContext {
        random: random_new(),
    });

    let scene = match get_scene(&ctx, scene) {
        Ok(scene) => scene,
        Err(err) => {
            eprintln!("failed to get scene: {err}");
            return ExitCode::from(1);
        }
    };

    // render image
    let mut img: image::ImageBuffer<
        image::Rgb<u8>,
        Vec<<image::Rgb<u8> as image::Pixel>::Subpixel>,
    > = image::ImageBuffer::new(scene.camera.image_width(), scene.camera.image_height());

    // generate work
    let mut work: Vec<Work> = vec![];
    let mut y = 0;
    loop {
        let mut x = 0;
        loop {
            work.push(Work {
                camera: scene.camera.clone(),
                world: scene.world.clone(),
                lights: scene.lights.clone(),
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
    for i in 0..threads {
        let work = work.clone();
        let results_send = results_send.clone();
        let ctx = ctx.clone();
        let thread = std::thread::Builder::new()
            .name(format!("RenderThread-{i}"))
            .spawn_with_priority(ThreadPriority::Min, move |_| {
                loop {
                    let item = { work.lock().unwrap().pop() };
                    match item {
                        Some(item) => {
                            let mut pixels = vec![];
                            for y in item.ymin..item.ymax {
                                for x in item.xmin..item.xmax {
                                    let pixel_color = item.camera.render(
                                        &ctx,
                                        x,
                                        y,
                                        &*item.world,
                                        item.lights.clone(),
                                    );
                                    pixels.push(pixel_color);
                                }
                            }
                            results_send
                                .send(WorkResult::DataWorkResult(DataWorkResult {
                                    xmin: item.xmin,
                                    xmax: item.xmax,
                                    ymin: item.ymin,
                                    ymax: item.ymax,
                                    pixels,
                                }))
                                .unwrap();
                        }
                        None => break,
                    }
                }
            });
        handles.push(thread.unwrap());
    }

    for _ in 0..work_count {
        let result = results_recv.recv().unwrap();
        match result {
            WorkResult::DataWorkResult(result) => {
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
        }
    }

    for h in handles {
        h.join().unwrap();
    }

    img.save("../../target/out.png").unwrap();
    pb.finish_with_message("Done!");
    ExitCode::SUCCESS
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
    pub lights: Option<Arc<dyn Node>>,
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
}

pub enum WorkResult {
    DataWorkResult(DataWorkResult),
}

pub struct DataWorkResult {
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
    pub pixels: Vec<Color>,
}
