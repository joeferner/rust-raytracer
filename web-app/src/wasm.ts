import type { CameraInfo, Color } from './wasm/rust_raytracer_wasm';
import init, { load_openscad, get_camera_info, render } from './wasm/rust_raytracer_wasm.js';

export type { CameraInfo, Color };

export const initWasm = init;

export const loadOpenscad = load_openscad;
export const getCameraInfo = get_camera_info;
export const renderBlock = render;
