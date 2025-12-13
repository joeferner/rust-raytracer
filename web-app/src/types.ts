import type { Color } from "./wasm";

export interface RenderDataInit {
    type: 'init';
    workerId: number,
    input: string;
}

export interface RenderDataWork {
    type: 'work';
    xmin: number;
    xmax: number;
    ymin: number;
    ymax: number;
}

export type RenderData = RenderDataInit | RenderDataWork;

export interface RenderResponseInit {
    type: 'init',
    workerId: number;
}

export interface DrawEvent {
    xmin: number;
    xmax: number;
    ymin: number;
    ymax: number;
    data: Color[];
}

export interface RenderResponseData extends DrawEvent {
    type: 'data';
    workerId: number;
}

export type RenderResponse = RenderResponseInit | RenderResponseData;
