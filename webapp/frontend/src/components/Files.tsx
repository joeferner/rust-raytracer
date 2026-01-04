import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import classes from './Files.module.scss';
import { projectStore } from '../stores/store';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';
import type { WorkingFile } from '../types';
import { For } from '@preact/signals-react/utils';

export function Files(): JSX.Element | null {
    const handleTabChange = (newValue: string | null): void => {
        projectStore.selectedTab.value = newValue ?? projectStore.files.value[0].filename;
    };

    if (projectStore.files.value.length === 0) {
        return null;
    }

    return (
        <Tabs value={projectStore.selectedTab.value ?? projectStore.files.value[0].filename} onChange={handleTabChange} className={classes.tabs}>
            <Tabs.List>
                <For each={projectStore.files}>
                    {(file) => (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            <div className={classes.tabFilename}>
                                {file.filename}
                                <div className={classes.unsavedIndicator}>
                                    {file.contents != file.originalContents ? '*' : ' '}
                                </div>
                            </div>
                        </Tabs.Tab>
                    )}
                </For>
            </Tabs.List>

            <For each={projectStore.files}>
                {(file) => (
                    <Tabs.Panel key={file.filename} value={file.filename} className={classes.tabPanel}>
                        <File file={file} />
                    </Tabs.Panel>
                )}
            </For>
        </Tabs>
    );
}

interface FileProps {
    file: WorkingFile
}

function File({ file }: FileProps): JSX.Element {
    if (file.contentType.startsWith('image/')) {
        return (<div>Image</div>);
    } else if (file.contentType.startsWith('text/') || file.contentType === 'application/x-openscad') {
        return (<FileEditor file={file} />);
    } else {
        return (<div>Unsupported: {file.contentType}</div>)
    }
}

function FileEditor({ file }: FileProps): JSX.Element {
    const handleCodeChange = (code: string | undefined): void => {
        projectStore.updateFile({ filename: file.filename, content: code ?? '' });
    };

    if (file.contentType.startsWith('image/')) {
        return (<div>Image</div>);
    } else if (file.contentType.startsWith('text/') || file.contentType === 'application/x-openscad') {
        return (
            <Editor
                height="100%"
                language="openscad"
                beforeMount={(monaco) => {
                    registerOpenscadLanguage(monaco);
                }}
                theme="vs-dark"
                value={file.contents}
                onChange={handleCodeChange}
                options={{ minimap: { enabled: false } }}
            />
        );
    } else {
        return (<div>Unsupported: {file.contentType}</div>)
    }
}
