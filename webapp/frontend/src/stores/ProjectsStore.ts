import { type UserDataProject } from '../api';
import { signal } from '@preact/signals-react';
import { projectStore, rayTracerApi } from './store';

const LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY = 'lastLoadedProjectId';

export class ProjectsStore {
    public readonly projects = signal<UserDataProject[]>([]);

    public async loadProjects(): Promise<void> {
        const response = await rayTracerApi.project.getProjects();
        this.projects.value = response.projects;
    }

    public set lastLoadedProjectId(value: string | undefined) {
        if (value) {
            window.localStorage.setItem(LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY, value);
        } else {
            window.localStorage.removeItem(LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY);
        }
    }

    public get lastLoadedProjectId(): string | undefined {
        return window.localStorage.getItem(LOCAL_STORAGE_LAST_LOADED_PROJECT_ID_KEY) ?? undefined;
    }

    public async createProject({ name }: { name: string }): Promise<void> {
        console.log('creating new project', name);
        const project = await rayTracerApi.project.createProject({ name });
        await projectStore.setProject({ ...project, readOnly: false });
        this.projects.value = [...this.projects.value, { ...project, readonly: false }];
    }

    public async deleteProject({ projectId }: { projectId: string }): Promise<void> {
        await rayTracerApi.project.deleteProject({ projectId });
        this.projects.value = this.projects.value.filter((p) => p.id !== projectId);

        if (projectId == projectStore.project.value?.id) {
            const newestProject = this.projects.value[0];
            if (newestProject) {
                await projectStore.loadProject({ projectId: newestProject.id });
            }
        }
    }

    public async copyProject({ projectId }: { projectId: string }): Promise<void> {
        const newProject = await rayTracerApi.project.copyProject({ projectId });
        await projectStore.setProject({ ...newProject, readOnly: false });
        this.projects.value = [...this.projects.value, { ...newProject, readonly: false }];
    }
}
