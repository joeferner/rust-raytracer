import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import styles from './Files.module.scss';
import { useMyContext } from '../state';
import type { JSX } from 'react';
import { registerOpenscadLanguage } from '../monaco-openscad';

export function Files(): JSX.Element {
    const { updateFile, getFile } = useMyContext();

    return (
        <Tabs defaultValue="main.scad" className={styles.tabs}>
            <Tabs.List>
                <Tabs.Tab value="main.scad">main.scad</Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="main.scad" className={styles.tabPanel}>
                <Editor
                    height="100%"
                    language="openscad"
                    beforeMount={(monaco) => {
                        registerOpenscadLanguage(monaco);
                    }}
                    theme="vs-dark"
                    value={getFile('main.scad')}
                    onChange={(code) => {
                        updateFile('main.scad', code ?? '');
                    }}
                    options={{ minimap: { enabled: false } }}
                />
            </Tabs.Panel>
        </Tabs>
    );
}
