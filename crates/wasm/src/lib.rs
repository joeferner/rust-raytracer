use std::sync::Arc;

use rust_raytracer_core::{
    Camera, Random, RenderContext, Vector3,
    object::{Group, Sphere},
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(aspect_ratio: f64, image_width: u32, x: u32, y: u32) -> Result<JsValue, JsValue> {
    let ctx = RenderContext {
        random: &WasmRandom::new(),
    };

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
    let camera = Camera::new(aspect_ratio, image_width);

    let pixel_color = camera.render(&ctx, x, y, &group);
    let color = Color::from(pixel_color);

    serde_wasm_bindgen::to_value(&color).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

pub struct WasmRandom {}

impl WasmRandom {
    fn new() -> Self {
        Self {}
    }
}

impl Random for WasmRandom {
    fn rand(&self) -> f64 {
        random()
    }

    fn rand_interval(&self, min: f64, max: f64) -> f64 {
        let delta = max - min;
        (random() * delta) + min
    }
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from(color: rust_raytracer_core::Color) -> Self {
        Color {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        }
    }
}
