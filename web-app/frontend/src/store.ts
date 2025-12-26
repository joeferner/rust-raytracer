import { atom } from 'jotai';
import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from './wasm';
import { RenderWorkerPool, type RenderCallbackFn } from './RenderWorkerPool';
import { Example, getExampleProject } from './utils/examples';
import type { WorkingFile } from './types';
import { RayTracerApi, type Settings, type User } from './api';
import { atomWithStorage } from 'jotai/utils';
import type { GoogleCredentialResponse } from './components/GoogleLogin';

export const rayTracerApi = new RayTracerApi();

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export const DEFAULT_RENDER_BLOCK_SIZE = 50;

// Singleton worker pool and draw event listeners
const renderWorkerPool = new RenderWorkerPool();
const drawEventListeners = new Set<RenderCallbackFn>();

// Base atoms
export const jwtTokenAtom = atomWithStorage<string | undefined>('jwtToken', undefined, undefined, { getOnInit: true });
export const userAtom = atom<User | undefined>(undefined);
export const settingsAtom = atom<Settings | undefined>(undefined);
export const filesAtom = atom<WorkingFile[]>([]);
export const cameraInfoAtom = atom<CameraInfo | undefined>(undefined);
export const renderOptionsAtom = atom<Required<RenderOptions>>({
    blockSize: DEFAULT_RENDER_BLOCK_SIZE,
    threadCount: typeof navigator !== 'undefined' ? (navigator.hardwareConcurrency ?? 4) : 4,
});

// Write-only atom for updateFile
export const updateFileAtom = atom(null, (get, set, update: { filename: string; content: string }) => {
    const files = get(filesAtom);
    const newFiles = files.map((f) => {
        if (f.filename === update.filename) {
            return {
                ...f,
                contents: update.content,
            };
        }
        return f;
    });
    set(filesAtom, newFiles);
});

// Read-only atom for getFile (returns a function)
export const getFileAtom = atom((get) => {
    const files = get(filesAtom);
    return (filename: string): WorkingFile | undefined => {
        return files.find((f) => f.filename === filename);
    };
});

// Write-only atom for render
export const renderAtom = atom(null, async (get, set) => {
    const files = get(filesAtom);
    const renderOptions = get(renderOptionsAtom);

    if (files.length === 0) return;

    const input = files[0].contents;

    await initWasm();
    loadOpenscad(input);

    const cameraInfo = getCameraInfo();
    const { threadCount } = renderOptions;
    console.log(`Begin render ${cameraInfo.width}x${cameraInfo.height}`);
    set(cameraInfoAtom, cameraInfo);

    renderWorkerPool.render(threadCount, input, {
        ...cameraInfo,
        ...renderOptions,
        callback: (event) => {
            for (const listener of drawEventListeners) {
                listener(event);
            }
        },
    });
});

export const handleGoogleCredentialResponseAtom = atom(
    null,
    async (_get, set, { response }: { response: GoogleCredentialResponse }) => {
        try {
            const data = await rayTracerApi.user.googleTokenVerify({
                token: response.credential,
            });

            rayTracerApi.request.config.TOKEN = data.token;

            const resp = await rayTracerApi.user.getUserMe();
            set(settingsAtom, resp.settings);
            set(userAtom, resp.user ?? undefined);
            set(jwtTokenAtom, data.token);
        } catch (err) {
            console.error('onGoogleCredentialResponse', err instanceof Error ? err : new Error('Unknown error'));
        }
    }
);

export const handleLogOutAtom = atom(null, (_get, set) => {
    set(userAtom, undefined);
    set(jwtTokenAtom, undefined);
});

export const loadExampleProjectAtom = atom(null, async (_get, set, example: Example) => {
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
    set(filesAtom, files);
});

export const loadUserMeAtom = atom(null, async (get, set) => {
    const jwtToken = get(jwtTokenAtom);
    if (jwtToken) {
        rayTracerApi.request.config.TOKEN = jwtToken;
    }

    const resp = await rayTracerApi.user.getUserMe();
    set(settingsAtom, resp.settings);
    set(userAtom, resp.user ?? undefined);
});

export function subscribeToDrawEvents(listener: RenderCallbackFn): UnsubscribeFn {
    drawEventListeners.add(listener);
    return () => drawEventListeners.delete(listener);
}
