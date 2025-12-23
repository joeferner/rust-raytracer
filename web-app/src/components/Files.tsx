import { Tabs } from '@mantine/core';
import { Editor } from '@monaco-editor/react';
import styles from './Files.module.scss';
import { useStore } from '../state';
import { observer } from 'mobx-react-lite';
import { registerOpenscadLanguage } from '../monaco-openscad';

export const Files = observer(() => {
    const store = useStore();

    return (
        <Tabs defaultValue="main.scad" className={styles.tabs}>
            <Tabs.List>
                {store.files.map((file) => {
                    return (
                        <Tabs.Tab key={file.filename} value={file.filename}>
                            {file.filename}
                        </Tabs.Tab>
                    );
                })}
            </Tabs.List>

            {store.files.map((file) => {
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
                                store.updateFile(file.filename, code ?? '');
                            }}
                            options={{ minimap: { enabled: false } }}
                        />
                    </Tabs.Panel>
                );
            })}
        </Tabs>
    );
});
