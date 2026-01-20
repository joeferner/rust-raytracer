import type { ImageWorkingFile, TextWorkingFile, WorkingFile } from './types.js';
import type {
    CameraInfo,
    Color,
    InitOutput,
    LoadResults,
    WasmImage,
    WasmSource,
    WasmMessage,
} from './wasm/debug/caustic_wasm';
import init, { load_openscad, get_camera_info, render } from './wasm/debug/caustic_wasm.js';
export { WasmLspServer } from './wasm/debug/caustic_wasm.js';

export type { CameraInfo, Color, WasmMessage };

export function initWasm(): Promise<InitOutput> {
    return init();
}

export function loadOpenscad(source: Source): LoadResults {
    return load_openscad(source);
}

export function getCameraInfo(): CameraInfo {
    return get_camera_info();
}

export function renderBlock(xmin: number, xmax: number, ymin: number, ymax: number): Color[] {
    return render(xmin, xmax, ymin, ymax);
}

export class Source implements WasmSource {
    public constructor(
        private readonly main: TextWorkingFile,
        private readonly files: WorkingFile[]
    ) {}

    public get_filename(): string {
        return this.main.filename;
    }

    public get_code(): string {
        return this.main.contents;
    }

    public get_image(filename: string): WasmImage {
        const file = this.files.find((f) => f.filename === filename);
        if (!file) {
            throw new Error('file not found');
        }
        if (file.type !== 'image') {
            throw new Error('expected file of type image');
        }
        return new Image(file);
    }
}

export class Image implements WasmImage {
    public constructor(private readonly file: ImageWorkingFile) {}

    public get_width(): number {
        return this.file.width;
    }

    public get_height(): number {
        return this.file.height;
    }

    public get_data(): ImageDataArray {
        return this.file.pixels;
    }
}
