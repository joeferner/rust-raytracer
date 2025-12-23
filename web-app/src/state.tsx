import { makeAutoObservable, runInAction } from 'mobx';
import { createContext, useContext, type JSX } from 'react';
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

class AppStore {
    public files: WorkingFile[] = [];
    public cameraInfo: CameraInfo | undefined = undefined;
    public renderOptions: Required<RenderOptions> = {
        blockSize: DEFAULT_RENDER_BLOCK_SIZE,
        threadCount: navigator.hardwareConcurrency ?? 4,
    };

    private renderWorkerPool = new RenderWorkerPool();
    private drawEventListeners = new Set<RenderCallbackFn>();

    public constructor() {
        makeAutoObservable(this);

        // Load initial project
        console.log('load initial project');
        setTimeout(() => {
            void this.loadExampleProject(Example.ThreeSpheres);
        });
    }

    public updateFile = (filename: string, newContents: string): void => {
        const file = this.files.find((f) => f.filename === filename);
        if (file) {
            file.contents = newContents;
        }
    };

    public getFile = (filename: string): WorkingFile | undefined => {
        return this.files.find((f) => f.filename === filename);
    };

    public render = async (): Promise<void> => {
        const input = this.files[0].contents;

        await initWasm();
        loadOpenscad(input);

        const cameraInfo = getCameraInfo();
        const { threadCount } = this.renderOptions;
        console.log(`Begin render ${cameraInfo.width}x${cameraInfo.height}`);

        runInAction(() => {
            this.cameraInfo = cameraInfo;
        });

        this.renderWorkerPool.render(threadCount, input, {
            ...cameraInfo,
            ...this.renderOptions,
            callback: (event) => {
                for (const listener of this.drawEventListeners) {
                    listener(event);
                }
            },
        });
    };

    public subscribeToDrawEvents = (listener: RenderCallbackFn): UnsubscribeFn => {
        this.drawEventListeners.add(listener);
        return () => this.drawEventListeners.delete(listener);
    };

    public loadExampleProject = async (example: Example): Promise<void> => {
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

        runInAction(() => {
            this.files = files;
        });
    };

    public setRenderOptions = (options: Partial<RenderOptions>): void => {
        this.renderOptions = {
            ...this.renderOptions,
            ...options,
        };
    };
}

const StoreContext = createContext<AppStore | undefined>(undefined);

export function StoreProvider({ children }: { children: React.ReactNode }): JSX.Element {
    const store = new AppStore();
    return <StoreContext value={store}>{children}</StoreContext>;
}

export function useStore(): AppStore {
    const context = useContext(StoreContext);
    if (!context) {
        throw new Error('useStore must be used within StoreProvider');
    }
    return context;
}
