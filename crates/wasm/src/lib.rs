#![allow(clippy::vec_init_then_push)]

pub mod types;

use std::{any::Any, cell::RefCell, fmt::Debug, sync::Arc};

use caustic_core::{
    Color as CoreColor, Image, RenderContext, SceneData, image::ImageError, random_new,
};
use caustic_openscad::{run_openscad, source::Source};
use js_sys::Uint8ClampedArray;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::types::message::WasmMessage;

thread_local! {
static LOADED_SCENE_DATA: RefCell<Option<SceneData>> = const { RefCell::new(None) };
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_CODE_RESOURCE_INTERFACE: &'static str = r#"
export interface WasmSource {
    get_filename(): string;
    get_code(): string;
    get_image(filename: string): WasmImage;
}
"#;

#[wasm_bindgen]
extern "C" {
    pub type WasmSource;

    #[wasm_bindgen(method, catch)]
    pub fn get_filename(this: &WasmSource) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn get_code(this: &WasmSource) -> Result<String, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn get_image(this: &WasmSource, filename: &str) -> Result<WasmImage, JsValue>;
}

// Add this wrapper struct
struct SendSyncWasmSource(WasmSource);

// SAFETY: In WASM, all JS interactions happen on the main thread.
// The wasm-bindgen runtime ensures thread safety.
unsafe impl Send for SendSyncWasmSource {}
unsafe impl Sync for SendSyncWasmSource {}

impl std::ops::Deref for SendSyncWasmSource {
    type Target = WasmSource;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct WasmSourceAdapter {
    filename: String,
    wasm_source: SendSyncWasmSource,
    code: String,
}

impl WasmSourceAdapter {
    pub fn new(wasm_source: WasmSource) -> Result<Self, JsValue> {
        let code = wasm_source.get_code()?;
        Ok(Self {
            filename: wasm_source.get_filename()?,
            wasm_source: SendSyncWasmSource(wasm_source),
            code,
        })
    }
}

impl Source for WasmSourceAdapter {
    fn get_code(&self) -> &str {
        &self.code
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_image(&self, filename: &str) -> Result<Arc<dyn Image>, ImageError> {
        let image = self.wasm_source.get_image(filename).map_err(|err| {
            ImageError::Other(format!("getting image from JavaScript failed: {err:?}"))
        })?;
        let image_adapter = WasmImageAdapter::new(image).map_err(|err| {
            ImageError::Other(format!("converting image from JavaScript failed: {err:?}"))
        })?;
        Ok(Arc::new(image_adapter))
    }

    fn get_filename(&self) -> &str {
        &self.filename
    }
}

impl Debug for WasmSourceAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmCodeResourceAdapter").finish()
    }
}

#[wasm_bindgen(typescript_custom_section)]
const WASM_IMAGE_INTERFACE: &'static str = r#"
export interface WasmImage {
    get_width(): number;
    get_height(): number;
    get_data(): ImageDataArray;
}
"#;

#[wasm_bindgen]
extern "C" {
    pub type WasmImage;

    #[wasm_bindgen(method, catch)]
    pub fn get_width(this: &WasmImage) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn get_height(this: &WasmImage) -> Result<u32, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn get_data(this: &WasmImage) -> Result<Uint8ClampedArray, JsValue>;
}

struct WasmImageAdapter {
    width: u32,
    height: u32,
    data: Vec<CoreColor>,
}

impl WasmImageAdapter {
    pub fn new(wasm_image: WasmImage) -> Result<Self, JsValue> {
        Ok(Self {
            width: wasm_image.get_width()?,
            height: wasm_image.get_height()?,
            data: wasm_image
                .get_data()?
                .to_vec()
                .chunks_exact(4)
                .map(|chunk| {
                    CoreColor {
                        r: (chunk[0] as f64) / 255.0,
                        g: (chunk[1] as f64) / 255.0,
                        b: (chunk[2] as f64) / 255.0,
                        // chunk[3] is alpha, which we ignore
                    }
                })
                .collect(),
        })
    }
}

impl Image for WasmImageAdapter {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get_pixel(&self, x: u32, y: u32) -> Option<CoreColor> {
        let index = ((y * self.width) + x) as usize;
        self.data.get(index).copied()
    }
}

impl Debug for WasmImageAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmImageAdapter").finish()
    }
}

#[wasm_bindgen]
pub fn load_openscad(wasm_source: WasmSource) -> Result<LoadResults, JsValue> {
    let source: Arc<Box<dyn Source>> = Arc::new(Box::new(WasmSourceAdapter::new(wasm_source)?));
    let random = random_new();
    let results = run_openscad(source, random);
    let messages = results.messages.iter().map(|m| m.into()).collect();

    let loaded = match results.scene_data {
        Some(scene_data) => {
            LOADED_SCENE_DATA.with(|data| *data.borrow_mut() = Some(scene_data));
            true
        }
        None => false,
    };

    Ok(LoadResults { messages, loaded })
}

#[wasm_bindgen]
pub fn get_camera_info() -> Result<CameraInfo, JsValue> {
    LOADED_SCENE_DATA.with(|data| {
        if let Some(scene_data) = data.borrow().as_ref() {
            let width = scene_data.camera.image_width();
            let height = scene_data.camera.image_height();
            Ok(CameraInfo { width, height })
        } else {
            Err(JsValue::from_str("Scene data not loaded"))
        }
    })
}

#[wasm_bindgen]
pub fn render(xmin: u32, xmax: u32, ymin: u32, ymax: u32) -> Result<Vec<Color>, JsValue> {
    LOADED_SCENE_DATA.with(|data| {
        if let Some(scene_data) = data.borrow().as_ref() {
            let ctx = Arc::new(RenderContext {
                random: random_new(),
            });
            let mut results: Vec<Color> = vec![];

            for y in ymin..ymax {
                for x in xmin..xmax {
                    let pixel_color = scene_data.camera.render(
                        &ctx,
                        x,
                        y,
                        &*scene_data.world,
                        scene_data.lights.clone(),
                    );
                    let color = Color::from(pixel_color);
                    results.push(color);
                }
            }

            Ok(results)
        } else {
            Err(JsValue::from_str("Scene data not loaded"))
        }
    })
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LoadResults {
    pub messages: Vec<WasmMessage>,
    pub loaded: bool,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CameraInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from(color: CoreColor) -> Self {
        Color {
            r: (color.r * 255.0) as u8,
            g: (color.g * 255.0) as u8,
            b: (color.b * 255.0) as u8,
        }
    }
}
