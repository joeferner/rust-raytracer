import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from './wasm';
import { RenderWorkerPool, type RenderCallbackFn } from './RenderWorkerPool';
import type { WorkingFile } from './types';
import { RayTracerApi, type Project, type Settings, type User, type UserDataProject } from './api';
import type { GoogleCredentialResponse } from './components/GoogleLogin';
import * as R from 'radash';
import { computed, signal } from '@preact/signals-react';

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
const LOCAL_STORAGE_JWT_TOKEN_KEY = 'jwtToken';
const LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY = 'lastLoadedProjectId';

const renderWorkerPool = new RenderWorkerPool();

export class Store {
    private readonly drawEventListeners = new Set<RenderCallbackFn>();

    public readonly user = signal<User | undefined>();
    public readonly settings = signal<Settings | undefined>(undefined);
    public readonly projects = signal<UserDataProject[] | undefined>(undefined);
    public readonly files = signal<WorkingFile[]>([]);
    public readonly cameraInfo = signal<CameraInfo | undefined>(undefined);
    public readonly renderOptions = signal<Required<RenderOptions>>({
        blockSize: DEFAULT_RENDER_BLOCK_SIZE,
        threadCount: typeof navigator !== 'undefined' ? (navigator.hardwareConcurrency ?? 4) : 4,
    });
    private readonly _project = signal<StoreProject | undefined>(undefined);
    public readonly project = computed(() => this._project.value);

    public async initialize(): Promise<void> {
        console.log('load initial project');
        await this.loadUserMe();
        await this.loadProjects();
        const lastLoadedProjectId = this.lastLoadedProjectId;
        const userProject = this.projects.value?.find((p) => p.id === lastLoadedProjectId);
        await this.loadProject({ projectId: userProject?.id ?? EXAMPLE_CAR_ID });
    }

    private set jwtToken(value: string | undefined) {
        if (value) {
            window.localStorage.setItem(LOCAL_STORAGE_JWT_TOKEN_KEY, value);
        } else {
            window.localStorage.removeItem(LOCAL_STORAGE_JWT_TOKEN_KEY);
        }
    }

    private get jwtToken(): string | undefined {
        return window.localStorage.getItem(LOCAL_STORAGE_JWT_TOKEN_KEY) ?? undefined;
    }

    private set lastLoadedProjectId(value: string | undefined) {
        if (value) {
            window.localStorage.setItem(LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY, value);
        } else {
            window.localStorage.removeItem(LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY);
        }
    }

    private get lastLoadedProjectId(): string | undefined {
        return window.localStorage.getItem(LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY) ?? undefined;
    }

    public updateFile(update: { filename: string; content: string }): void {
        this.files.value = this.files.value.map((f) => {
            if (f.filename === update.filename) {
                return {
                    ...f,
                    contents: update.content,
                };
            }
            return f;
        });
    }

    private async loadUserMe(): Promise<void> {
        const jwtToken = this.jwtToken;
        if (jwtToken) {
            rayTracerApi.request.config.TOKEN = jwtToken;
        }

        const resp = await rayTracerApi.user.getUserMe();
        this.settings.value = resp.settings;
        this.user.value = resp.user ?? undefined;
    }

    public async loadProjects(): Promise<void> {
        const response = await rayTracerApi.project.getProjects();
        this.projects.value = response.projects;
    }

    private async updateProject(newProject: StoreProject): Promise<void> {
        if (this._project.value?.id !== newProject?.id && newProject?.id) {
            const files = await this.loadProjectFiles(newProject);
            document.title = `Caustic: ${newProject.name}`;
            this.files.value = files;
            this.lastLoadedProjectId = newProject.id;
        }
        this._project.value = newProject;
    }

    public async createProject({ name }: { name: string }): Promise<void> {
        console.log('creating new project', name);
        const project = await rayTracerApi.project.createProject({ name });
        await this.updateProject({ ...project, readOnly: false });
        // TODO add project to projects list
    }

    public async deleteProject({ projectId }: { projectId: string }): Promise<void> {
        await rayTracerApi.project.deleteProject({ projectId });
        this.projects.value = this.projects.value?.filter((p) => p.id !== projectId);

        if (projectId == this.project.value?.id) {
            const newestProject = this.projects.value?.[0];
            if (newestProject) {
                await this.loadProject({ projectId: newestProject.id });
            }
        }
    }

    public async copyProject({ projectId }: { projectId: string }): Promise<void> {
        const newProject = await rayTracerApi.project.copyProject({ projectId });
        await this.updateProject({ ...newProject, readOnly: false });
    }

    public async loadProject({ projectId }: { projectId: string }): Promise<void> {
        if (!this.projects.value) {
            throw new Error('cannot load project until user projects are loaded');
        }
        const userProject = this.projects.value?.find((project) => project.id === projectId);
        if (!userProject) {
            throw new Error(`project ${projectId} not found in user projects`);
        }
        console.log(`getting project (projectId: ${projectId})`);
        const project = await rayTracerApi.project.getProject(projectId);
        await this.updateProject({ ...project, readOnly: userProject.readonly });
    }

    public async render(): Promise<void> {
        if (this.files.value.length === 0) {
            return;
        }

        const input = this.files.value[0].contents;

        await initWasm();
        loadOpenscad(input);

        const cameraInfo = getCameraInfo();
        const { threadCount } = this.renderOptions.value;
        console.log(`Begin render ${cameraInfo.width}x${cameraInfo.height}`);
        this.cameraInfo.value = cameraInfo;

        renderWorkerPool.render(threadCount, input, {
            ...cameraInfo,
            ...this.renderOptions.value,
            callback: (event) => {
                for (const listener of this.drawEventListeners) {
                    listener(event);
                }
            },
        });
    }

    public logOut(): void {
        this.user.value = undefined;
        this.jwtToken = undefined;
    }

    public async handleGoogleCredentialResponse({ response }: { response: GoogleCredentialResponse }): Promise<void> {
        try {
            const data = await rayTracerApi.user.googleTokenVerify({
                token: response.credential,
            });

            rayTracerApi.request.config.TOKEN = data.token;

            const resp = await rayTracerApi.user.getUserMe();
            this.settings.value = resp.settings;
            this.user.value = resp.user ?? undefined;
            this.jwtToken = data.token;
        } catch (err) {
            console.error('onGoogleCredentialResponse', err instanceof Error ? err : new Error('Unknown error'));
        }
    }

    private async loadProjectFiles(project: Project): Promise<WorkingFile[]> {
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

    public subscribeToDrawEvents(listener: RenderCallbackFn): UnsubscribeFn {
        this.drawEventListeners.add(listener);
        return () => this.drawEventListeners.delete(listener);
    }
}

export const store = new Store();
