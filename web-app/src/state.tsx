/* eslint-disable react-refresh/only-export-components */

import { createContext, use, useRef, useState, type JSX, type ReactNode } from 'react';
import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from './wasm';
import { RenderWorkerPool } from './RenderWorkerPool';
import type { RenderResponse } from './types';

const code = `
// camera
camera(
    // aspect_ratio = 1.0,
    image_width = 400,
    image_height = 400,
    samples_per_pixel = 10,
    max_depth = 10,
    vertical_fov = 90.0,
    look_from = [50.0, -50.0, 70.0],
    look_at = [0.0, 0.0, 0.0],
    up = [0.0, 0.0, 1.0],
    defocus_angle = 0.0,
    focus_distance = 10.0,
    background = [0.7, 0.8, 1.0]
);

color([0,125,255]/255)
    scale([1.2,1,1])
    cube([60,20,10],center=true);
`;

export type DrawEventListener = (event: RenderResponse) => void;

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export const DEFAULT_RENDER_BLOCK_SIZE = 50;

interface MyContextType {
    files: Record<string, string>;
    cameraInfo: CameraInfo | undefined;
    renderOptions: Required<RenderOptions>;
    render: () => Promise<void>;
    updateFile: (filename: string, content: string) => void;
    getFile: (filename: string) => string | undefined;
    subscribeToDrawEvents: (listener: DrawEventListener) => UnsubscribeFn;
}

const MyContext = createContext<MyContextType | undefined>(undefined);

interface MyProviderProps {
    children: ReactNode;
}

const renderWorkerPool = new RenderWorkerPool();

export function MyProvider({ children }: MyProviderProps): JSX.Element {
    const [renderOptions, _setRenderOptions] = useState<Required<RenderOptions>>({
        blockSize: DEFAULT_RENDER_BLOCK_SIZE,
        threadCount: 2,
    });
    const [files, setFiles] = useState<Record<string, string>>({
        'main.scad': code,
    });
    const [cameraInfo, setCameraInfo] = useState<CameraInfo | undefined>(undefined);
    const drawEventListeners = useRef(new Set<DrawEventListener>());

    const updateFile = (filename: string, content: string): void => {
        setFiles((prev) => ({
            ...prev,
            [filename]: content,
        }));
    };

    const getFile = (filename: string): string | undefined => {
        return files[filename];
    };

    const render = async (): Promise<void> => {
        const input = files['main.scad'];

        await initWasm();
        loadOpenscad(input);

        const cameraInfo = getCameraInfo();
        const { threadCount } = renderOptions;
        console.log(`Begin render ${cameraInfo.width}x${cameraInfo.height}`);
        setCameraInfo(cameraInfo);

        const localDrawEventListeners = drawEventListeners.current;
        renderWorkerPool.render(threadCount, input, {
            ...cameraInfo,
            ...renderOptions,
            callback: (data) => {
                for (const localDrawEventListener of localDrawEventListeners) {
                    localDrawEventListener(data);
                }
            },
        });
    };

    const subscribeToDrawEvents = (listener: DrawEventListener): UnsubscribeFn => {
        drawEventListeners.current.add(listener);
        return () => drawEventListeners.current.delete(listener);
    };

    const value: MyContextType = {
        files,
        cameraInfo,
        renderOptions,
        updateFile,
        getFile,
        render,
        subscribeToDrawEvents,
    };

    return <MyContext value={value}>{children}</MyContext>;
}

export function useMyContext(): MyContextType {
    const context = use(MyContext);
    if (!context) {
        throw new Error('useMyContext must be used within MyProvider');
    }
    return context;
}
