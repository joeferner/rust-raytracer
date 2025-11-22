import init, { render } from '../pkg/rust_raytracer_wasm.js';

let wasmReady = init();

onmessage = async (ev) => {
    const { width, height, x, y } = ev.data;
    await wasmReady;
    const color = render(width / height, width, x, y);
    postMessage({ x, y, color });
};
