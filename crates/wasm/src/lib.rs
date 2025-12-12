#![allow(clippy::vec_init_then_push)]
use std::sync::Arc;

use rust_raytracer_core::{Color, RenderContext, random_new};
use rust_raytracer_openscad::openscad_string_to_scene_data;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_openscad(input: &str) -> Result<JsValue, JsValue> {
    let scene_data =
        openscad_string_to_scene_data(input).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    serde_wasm_bindgen::to_value(&scene_data).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[wasm_bindgen]
pub fn render_openscad(
    input: &str,
    xmin: u32,
    xmax: u32,
    ymin: u32,
    ymax: u32,
) -> Result<JsValue, JsValue> {
    let scene_data =
        openscad_string_to_scene_data(input).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let ctx = Arc::new(RenderContext {
        random: random_new(),
    });
    let mut results: Vec<WasmColor> = vec![];

    for y in ymin..ymax {
        for x in xmin..xmax {
            let pixel_color =
                scene_data
                    .camera
                    .render(&ctx, x, y, &*scene_data.world, scene_data.lights.clone());
            let color = WasmColor::from(pixel_color);
            results.push(color);
        }
    }

    serde_wasm_bindgen::to_value(&results).map_err(|e| JsValue::from_str(&format!("{}", e)))
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
