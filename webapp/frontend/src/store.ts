import { atom } from 'jotai';
import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from './wasm';
import { RenderWorkerPool, type RenderCallbackFn } from './RenderWorkerPool';
import type { WorkingFile } from './types';
import { RayTracerApi, type Project, type Settings, type User, type UserDataProject } from './api';
import { atomWithStorage } from 'jotai/utils';
import type { GoogleCredentialResponse } from './components/GoogleLogin';
import * as R from 'radash';

export const rayTracerApi = new RayTracerApi();

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export interface StoreProject extends Project {
    readOnly: boolean;
}

export const DEFAULT_RENDER_BLOCK_SIZE = 50;
export const EXAMPLE_CAR_ID = 'cad84577-c808-41a9-8d77-25a4626fe65f';

// Singleton worker pool and draw event listeners
const renderWorkerPool = new RenderWorkerPool();
const drawEventListeners = new Set<RenderCallbackFn>();

// Base atoms
export const jwtTokenAtom = atomWithStorage<string | undefined>('jwtToken', undefined, undefined, { getOnInit: true });
export const lastLoadedProjectIdAtom = atomWithStorage<string | undefined>(
    'lastLoadedProjectId',
    undefined,
    undefined,
    { getOnInit: true }
);
export const userAtom = atom<User | undefined>(undefined);
export const settingsAtom = atom<Settings | undefined>(undefined);
export const projectsAtom = atom<UserDataProject[] | undefined>(undefined);
export const projectAtom = atom<StoreProject | undefined>(undefined);
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

export const initializeAtom = atom(null, async (get, set) => {
    console.log('load initial project');
    await set(loadUserMeAtom);
    await set(loadProjectsAtom);
    const lastLoadedProjectId = get(lastLoadedProjectIdAtom);
    const userProject = get(projectsAtom)?.find((p) => p.id === lastLoadedProjectId);
    await set(loadProjectAtom, { projectId: userProject?.id ?? EXAMPLE_CAR_ID });
});

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

async function loadProjectFiles(project: Project): Promise<WorkingFile[]> {
    return await Promise.all(
        project.files.map(async (f) => {
            console.log(`getting project file (projectId: ${project.id}, filename: ${f.filename})`);

            // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
            const contents = await rayTracerApi.project.getProjectFile(project.id, f.filename);
            if (R.isString(contents)) {
                return {
                    ...f,
                    originalContents: contents,
                    contents,
                } satisfies WorkingFile;
            } else {
                console.log('unhandled file contents', contents);
                throw new Error('todo');
            }
        })
    );
}

export const loadProjectsAtom = atom(null, async (_get, set) => {
    const response = await rayTracerApi.project.getProjects();
    set(projectsAtom, response.projects);
});

export const loadProjectAtom = atom(null, async (get, set, { projectId }: { projectId: string }) => {
    const projects = get(projectsAtom);
    if (!projects) {
        throw new Error('cannot load project until user projects are loaded');
    }
    const userProject = projects?.find((project) => project.id === projectId);
    if (!userProject) {
        throw new Error(`project ${projectId} not found in user projects`);
    }
    console.log(`getting project (projectId: ${projectId})`);
    const project = await rayTracerApi.project.getProject(projectId);
    const files = await loadProjectFiles(project);
    set(projectAtom, { ...project, readOnly: userProject.readonly });
    set(filesAtom, files);
    set(lastLoadedProjectIdAtom, project.id);
});

export const createProjectAtom = atom(null, async (_get, set, { name }: { name: string }) => {
    console.log('creating new project', name);
    const project = await rayTracerApi.project.createProject({ name });
    const files = await loadProjectFiles(project);
    set(projectAtom, { ...project, readOnly: false });
    set(filesAtom, files);
    set(lastLoadedProjectIdAtom, project.id);
});

export const copyProjectAtom = atom(null, async (_get, set, { projectId }: { projectId: string }) => {
    const newProject = await rayTracerApi.project.copyProject({ projectId });
    const files = await loadProjectFiles(newProject);
    set(projectAtom, { ...newProject, readOnly: false });
    set(filesAtom, files);
    set(lastLoadedProjectIdAtom, newProject.id);
});

export const deleteProjectAtom = atom(null, async (get, set, { projectId }: { projectId: string }) => {
    await rayTracerApi.project.deleteProject({ projectId });
    set(
        projectsAtom,
        get(projectsAtom)?.filter((p) => p.id !== projectId)
    );

    if (projectId == get(projectAtom)?.id) {
        const newestProject = get(projectsAtom)?.[0];
        if (newestProject) {
            await set(loadProjectAtom, { projectId: newestProject.id });
        }
    }
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
