import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import styles from './Files.module.scss';
import { filesAtom, updateFileAtom } from '../store';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';
import { useAtomValue, useSetAtom } from 'jotai';

export function Files(): JSX.Element {
    const files = useAtomValue(filesAtom);
    const updateFile = useSetAtom(updateFileAtom);

    return (
        <Tabs defaultValue="main.scad" className={styles.tabs}>
            <Tabs.List>
                {files.map((file) => {
                    return (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            {file.filename}
                        </Tabs.Tab>
                    );
                })}
            </Tabs.List>

            {files.map((file) => {
                return (
                    <Tabs.Panel key={file.filename} value={file.filename} className={styles.tabPanel}>
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
