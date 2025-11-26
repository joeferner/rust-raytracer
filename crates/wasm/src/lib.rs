use std::sync::Arc;

use rust_raytracer_core::{
    Color, RenderContext, Vector3,
    camera::CameraBuilder,
    material::{Lambertian, Metal},
    object::{Group, Sphere},
    random_new,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(aspect_ratio: f64, image_width: u32, x: u32, y: u32) -> Result<JsValue, JsValue> {
    let ctx = RenderContext {
        random: &random_new(),
    };

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut group = Group::new();
    group.push(Arc::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    group.push(Arc::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    group.push(Arc::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    group.push(Arc::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = aspect_ratio;
    camera_builder.image_width = image_width;
    let camera = camera_builder.build();

    let pixel_color = camera.render(&ctx, x, y, &group);
    let color = WasmColor::from(pixel_color);

    serde_wasm_bindgen::to_value(&color).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct WasmColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl WasmColor {
    pub fn from(color: Color) -> Self {
        WasmColor {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        }
    }
}
