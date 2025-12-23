import type { Color } from './wasm';

export interface RenderResult {
    xmin: number;
    xmax: number;
    ymin: number;
    ymax: number;
    data: Color[];
}

export interface RenderRequestInit {
    type: 'init';
    workerId: number;
    input: string;
}

export interface RenderRequestWork {
    type: 'work';
    xmin: number;
    xmax: number;
    ymin: number;
    ymax: number;
}

export type RenderRequest = RenderRequestInit | RenderRequestWork;

export interface RenderResponseInit {
    type: 'init';
    workerId: number;
}

export interface RenderResponseData extends RenderResult {
    type: 'data';
    workerId: number;
}

export type RenderResponse = RenderResponseInit | RenderResponseData;

export interface Project {
    files: ProjectFile[];
}

export interface ProjectFile {
    filename: string;
    url: string;
}

export interface WorkingFile extends ProjectFile {
    contents: string;
}
