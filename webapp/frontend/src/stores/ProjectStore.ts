import { getCameraInfo, initWasm, loadOpenscad, type CameraInfo } from '../wasm';
import { RenderWorkerPool, type RenderCallbackFn } from '../RenderWorkerPool';
import type { WorkingFile } from '../types';
import { type Project } from '../api';
import * as R from 'radash';
import { computed, signal } from '@preact/signals-react';
import {
    DEFAULT_RENDER_BLOCK_SIZE,
    EXAMPLE_CAR_ID,
    projectsStore,
    rayTracerApi,
    type RenderOptions,
    type StoreProject,
    type UnsubscribeFn,
} from './store';

const renderWorkerPool = new RenderWorkerPool();

export class ProjectStore {
    private readonly drawEventListeners = new Set<RenderCallbackFn>();

    public readonly files = signal<WorkingFile[]>([]);
    public readonly cameraInfo = signal<CameraInfo | undefined>(undefined);
    public readonly renderOptions = signal<Required<RenderOptions>>({
        blockSize: DEFAULT_RENDER_BLOCK_SIZE,
        threadCount: typeof navigator !== 'undefined' ? (navigator.hardwareConcurrency ?? 4) : 4,
    });
    public readonly selectedTab = signal<string | undefined>(undefined);

    private readonly _project = signal<StoreProject | undefined>(undefined);
    // expose a read only copy of project, projects must be changed via setProject
    public readonly project = computed(() => this._project.value);

    public async loadLastProject(): Promise<void> {
        const lastLoadedProjectId = projectsStore.lastLoadedProjectId;
        const userProject = projectsStore.projects.value.find((p) => p.id === lastLoadedProjectId);
        await this.loadProject({ projectId: userProject?.id ?? EXAMPLE_CAR_ID });
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

    public async setProject(newProject: StoreProject): Promise<void> {
        if (this._project.value?.id !== newProject?.id && newProject?.id) {
            const files = await this.loadProjectFiles(newProject);
            document.title = `Caustic: ${newProject.name}`;
            this.files.value = files;
            projectsStore.lastLoadedProjectId = newProject.id;
            this.selectedTab.value = files[0].filename;
        }
        this._project.value = newProject;
    }

    public async loadProject({ projectId }: { projectId: string }): Promise<void> {
        const userProject = projectsStore.projects.value.find((project) => project.id === projectId);
        if (!userProject) {
            throw new Error(`project ${projectId} not found in user projects`);
        }
        console.log(`getting project (projectId: ${projectId})`);
        const project = await rayTracerApi.project.getProject(projectId);
        await this.setProject({ ...project, readOnly: userProject.readonly });
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

    private async loadProjectFiles(project: Project): Promise<WorkingFile[]> {
        return await Promise.all(
            project.files.map(async (f) => {
                console.log(`getting project file (projectId: ${project.id}, filename: ${f.filename})`);

                // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
                const contents = await rayTracerApi.project.getProjectFile(project.id, f.filename);
                console.log(contents);
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
