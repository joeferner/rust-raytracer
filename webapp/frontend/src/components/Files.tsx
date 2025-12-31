import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import classes from './Files.module.scss';
import { filesAtom, updateFileAtom } from '../store';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';
import { useAtomValue, useSetAtom } from 'jotai';

export function Files(): JSX.Element | null {
    const files = useAtomValue(filesAtom);
    const updateFile = useSetAtom(updateFileAtom);

    if (files.length === 0) {
        return null;
    }

    return (
        <Tabs value={files[0].filename} className={classes.tabs}>
            <Tabs.List>
                {files.map((file) => {
                    return (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            <div className={classes.tabFilename}>
                                {file.filename}
                                <div className={classes.unsavedIndicator}>
                                    {file.contents != file.originalContents ? '*' : ' '}
                                </div>
                            </div>
                        </Tabs.Tab>
                    );
                })}
            </Tabs.List>

            {files.map((file) => {
                return (
                    <Tabs.Panel key={file.filename} value={file.filename} className={classes.tabPanel}>
                        <Editor
                            height="100%"
                            language="openscad"
                            beforeMount={(monaco) => {
                                registerOpenscadLanguage(monaco);
                            }}
                            theme="vs-dark"
                            value={file.contents}
                            onChange={(code) => {
                                updateFile({ filename: file.filename, content: code ?? '' });
                            }}
                            options={{ minimap: { enabled: false } }}
                        />
                    </Tabs.Panel>
                );
            })}
        </Tabs>
    );
}
