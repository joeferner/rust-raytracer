import { modals } from '@mantine/modals';
import {
    ActionIcon,
    Button,
    Divider,
    Group,
    Loader,
    Modal,
    Paper,
    Stack,
    Text,
    TextInput,
    Tooltip,
    UnstyledButton,
} from '@mantine/core';
import { useCallback, useEffect, useState, type JSX, type MouseEvent } from 'react';
import classes from './OpenProjectDialog.module.scss';
import { useAtomValue, useSetAtom } from 'jotai';
import {
    copyProjectAtom,
    createProjectAtom,
    deleteProjectAtom,
    loadProjectAtom,
    loadProjectsAtom,
    projectsAtom,
    userAtom,
} from '../store';
import { ErrorMessage, type ErrorMessageProps } from './ErrorMessage';
import { Copy as CopyIcon, Trash as DeleteIcon } from 'react-bootstrap-icons';
import type { UserDataProject } from '../api';

export function OpenProjectDialog({ opened, onClose }: { opened: boolean; onClose: () => void }): JSX.Element {
    const user = useAtomValue(userAtom);
    const projects = useAtomValue(projectsAtom);
    const storeLoadProjects = useSetAtom(loadProjectsAtom);
    const storeLoadProject = useSetAtom(loadProjectAtom);
    const storeCopyProject = useSetAtom(copyProjectAtom);
    const storeDeleteProject = useSetAtom(deleteProjectAtom);
    const createProject = useSetAtom(createProjectAtom);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<ErrorMessageProps | undefined>(undefined);
    const [newProjectName, setNewProjectName] = useState('');
    const [canSubmit, setCanSubmit] = useState(false);

    useEffect(() => {
        const loadProjects = async (): Promise<void> => {
            try {
                setError(undefined);
                setLoading(true);
                await storeLoadProjects();
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                setError({
                    title: 'Error Loading Projects',
                    message,
                });
            } finally {
                setLoading(false);
            }
        };

        if (opened) {
            setNewProjectName('');
            void loadProjects();
        }
    }, [opened, setNewProjectName, setError, storeLoadProjects]);

    useEffect(() => {
        setCanSubmit(newProjectName.trim().length > 0 && !loading);
    }, [newProjectName, loading]);

    const loadProject = useCallback(
        (projectId: string): void => {
            void (async (): Promise<void> => {
                try {
                    setLoading(true);
                    setError(undefined);
                    await storeLoadProject({ projectId });
                    onClose();
                } catch (err) {
                    const message = err instanceof Error ? err.message : 'Unknown error';
                    setError({
                        title: 'Error Loading Project',
                        message,
                    });
                } finally {
                    setLoading(false);
                }
            })();
        },
        [storeLoadProject, onClose, setError, setLoading]
    );

    const copyProject = useCallback(
        (projectId: string): void => {
            void (async (): Promise<void> => {
                try {
                    setLoading(true);
                    setError(undefined);
                    await storeCopyProject({ projectId });
                    onClose();
                } catch (err) {
                    const message = err instanceof Error ? err.message : 'Unknown error';
                    setError({
                        title: 'Error Coping Project',
                        message,
                    });
                } finally {
                    setLoading(false);
                }
            })();
        },
        [storeCopyProject, onClose, setError, setLoading]
    );

    const deleteProject = useCallback(
        (projectId: string): void => {
            void (async (): Promise<void> => {
                try {
                    setLoading(true);
                    setError(undefined);
                    await storeDeleteProject({ projectId });
                } catch (err) {
                    const message = err instanceof Error ? err.message : 'Unknown error';
                    setError({
                        title: 'Error Deleting Project',
                        message,
                    });
                } finally {
                    setLoading(false);
                }
            })();
        },
        [storeDeleteProject, setError, setLoading]
    );

    const onCancelClick = useCallback((): void => {
        onClose();
    }, [onClose]);

    const onCreateProjectClick = useCallback((): void => {
        void (async (): Promise<void> => {
            try {
                setLoading(true);
                setError(undefined);
                await createProject({ name: newProjectName });
                onClose();
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                setError({
                    title: 'Error Creating Project',
                    message,
                });
            } finally {
                setLoading(false);
            }
        })();
    }, [createProject, newProjectName, onClose, setError, setLoading]);

    return (
        <Modal opened={opened} onClose={onClose} title="Open Project" zIndex={2000}>
            <Stack className={classes.group} align="start">
                {error ? <ErrorMessage {...error} width="100%" /> : null}
                <div className={classes.item}>
                    <div>
                        <TextInput
                            placeholder="New Project Name"
                            inputSize="100"
                            label="New Project Name"
                            description={user ? null : 'To create a new project you must be logged in'}
                            value={newProjectName}
                            onChange={(event) => {
                                setNewProjectName(event.target.value);
                            }}
                        />
                    </div>
                </div>
                <Divider
                    my="xs"
                    label="Previous Projects"
                    labelPosition="center"
                    style={{ width: `100%`, margin: 0 }}
                />
                <div className={classes.item}>
                    <Stack className={classes.existingProjects}>
                        {projects?.map((project) => (
                            <ProjectButton
                                key={project.id}
                                project={project}
                                onClick={() => {
                                    loadProject(project.id);
                                }}
                                onCopyProject={() => {
                                    copyProject(project.id);
                                }}
                                onDeleteProject={() => {
                                    deleteProject(project.id);
                                }}
                            />
                        ))}
                    </Stack>
                </div>

                <Group justify="flex-end" className={classes.footer}>
                    {loading ? <Loader color="blue" size="xs" type="bars" /> : null}
                    <UnstyledButton onClick={onCancelClick}>Cancel</UnstyledButton>
                    <Button onClick={onCreateProjectClick} disabled={!canSubmit}>
                        Create Project
                    </Button>
                </Group>
            </Stack>
        </Modal>
    );
}

function ProjectButton({
    project,
    onClick,
    onCopyProject,
    onDeleteProject,
}: {
    project: UserDataProject;
    onClick: () => void;
    onCopyProject: () => void;
    onDeleteProject: () => void;
}): JSX.Element {
    const onCopyProjectClick = useCallback(
        (event: MouseEvent) => {
            event.stopPropagation();
            onCopyProject();
        },
        [onCopyProject]
    );

    const onDeleteProjectClick = useCallback(
        (event: MouseEvent) => {
            event.stopPropagation();
            modals.openConfirmModal({
                title: 'Delete Project',
                children: (
                    <Text size="sm">
                        Are you sure you want to delete project "{project.name}"? This action cannot be undone.
                    </Text>
                ),
                labels: { confirm: 'Delete', cancel: 'Cancel' },
                confirmProps: { color: 'red' },
                onConfirm: () => {
                    onDeleteProject();
                },
                zIndex: 5000,
            });
        },
        [onDeleteProject, project]
    );

    return (
        <Paper key={project.id} onClick={onClick}>
            <div className={classes.projectName}>{project.name}</div>
            <div className={classes.projectActions}>
                <Tooltip label="Clone Project" zIndex={5000}>
                    <ActionIcon
                        onClick={(event) => {
                            onCopyProjectClick(event);
                        }}
                        variant="filled"
                        size="sm"
                    >
                        <CopyIcon />
                    </ActionIcon>
                </Tooltip>
                {project.readonly ? null : (
                    <Tooltip label="Delete Project" zIndex={5000}>
                        <ActionIcon
                            onClick={(event) => {
                                onDeleteProjectClick(event);
                            }}
                            variant="filled"
                            size="sm"
                        >
                            <DeleteIcon />
                        </ActionIcon>
                    </Tooltip>
                )}
            </div>
        </Paper>
    );
}
