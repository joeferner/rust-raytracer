/* eslint-disable react-refresh/only-export-components */

import { createContext, use, useEffect, useRef, useState, type JSX, type ReactNode } from 'react';
import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from './wasm';
import { RenderWorkerPool, type RenderCallbackFn } from './RenderWorkerPool';
import { Example, getExampleProject } from './utils/examples';
import type { WorkingFile } from './types';

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export const DEFAULT_RENDER_BLOCK_SIZE = 50;

interface MyContextType {
    files: WorkingFile[];
    cameraInfo: CameraInfo | undefined;
    renderOptions: Required<RenderOptions>;
    render: () => Promise<void>;
    updateFile: (filename: string, content: string) => void;
    getFile: (filename: string) => WorkingFile | undefined;
    subscribeToDrawEvents: (listener: RenderCallbackFn) => UnsubscribeFn;
    loadExampleProject: (example: Example) => Promise<void>;
}

const MyContext = createContext<MyContextType | undefined>(undefined);

interface MyProviderProps {
    children: ReactNode;
}

const renderWorkerPool = new RenderWorkerPool();

export function MyProvider({ children }: MyProviderProps): JSX.Element {
    const [renderOptions, _setRenderOptions] = useState<Required<RenderOptions>>({
        blockSize: DEFAULT_RENDER_BLOCK_SIZE,
        threadCount: navigator.hardwareConcurrency ?? 4,
    });
    const [files, setFiles] = useState<WorkingFile[]>([]);
    const [cameraInfo, setCameraInfo] = useState<CameraInfo | undefined>(undefined);
    const drawEventListeners = useRef(new Set<RenderCallbackFn>());

    const updateFile = (filename: string, newContents: string): void => {
        setFiles((prev) => {
            return prev.map((f) => {
                if (f.filename === filename) {
                    return {
                        ...f,
                        contents: newContents,
                    };
                }
                return f;
            });
        });
    };

    const getFile = (filename: string): WorkingFile | undefined => {
        return files.find((f) => f.filename == filename);
    };

    const render = async (): Promise<void> => {
        const input = files[0].contents;

        await initWasm();
        const results = loadOpenscad(input);
        console.log(results.output);

        const cameraInfo = getCameraInfo();
        const { threadCount } = renderOptions;
        console.log(`Begin render ${cameraInfo.width}x${cameraInfo.height}`);
        setCameraInfo(cameraInfo);

        const localDrawEventListeners = drawEventListeners.current;
        renderWorkerPool.render(threadCount, input, {
            ...cameraInfo,
            ...renderOptions,
            callback: (event) => {
                for (const localDrawEventListener of localDrawEventListeners) {
                    localDrawEventListener(event);
                }
            },
        });
    };

    const subscribeToDrawEvents = (listener: RenderCallbackFn): UnsubscribeFn => {
        drawEventListeners.current.add(listener);
        return () => drawEventListeners.current.delete(listener);
    };

    const loadExampleProject = async (example: Example): Promise<void> => {
        console.log('loadExampleProject', example);
        const project = getExampleProject(example);
        const files = await Promise.all(
            project.files.map(async (f) => {
                const contents = await (await fetch(f.url)).text();
                return {
                    ...f,
                    contents,
                } satisfies WorkingFile;
            })
        );
        setFiles(files);
    };

    useEffect(() => {
        console.log('load initial project');
        setTimeout(() => {
            void loadExampleProject(Example.RandomSpheres);
        });
    }, []);

    const value: MyContextType = {
        files,
        cameraInfo,
        renderOptions,
        updateFile,
        getFile,
        render,
        subscribeToDrawEvents,
        loadExampleProject,
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
