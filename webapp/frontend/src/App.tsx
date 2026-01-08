import classes from './App.module.scss';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { Render } from './components/Render';
import { Navbar } from './components/Navbar';
import { useEffect, type JSX } from 'react';
import { Header } from './components/Header';
import { ModalsProvider } from '@mantine/modals';
import { MantineProvider } from '@mantine/core';
import { projectsStore, projectStore, userStore } from './stores/store';
import '@mantine/notifications/styles.css';
import { Notifications } from '@mantine/notifications';

export function App(): JSX.Element {
    return (
        <MantineProvider>
            <ModalsProvider>
                <Notifications position="top-right" />
                <InnerApp />
            </ModalsProvider>
        </MantineProvider>
    );
}

function InnerApp(): JSX.Element {
    useEffect(() => {
        void (async (): Promise<void> => {
            // loadUserMe must come first
            await userStore.loadUserMe();
            await projectsStore.loadProjects();
            await projectStore.loadLastProject();
        })();
    }, []);

    return (
        <div className={classes.main}>
            <Header />
            <div className={classes.inner}>
                <Navbar />
                <PanelGroup autoSaveId="editRender" direction="horizontal">
                    <Panel defaultSize={50}>
                        <Files />
                    </Panel>
                    <PanelResizeHandle className="resizeHandle" />
                    <Panel>
                        <Render />
                    </Panel>
                </PanelGroup>
            </div>
        </div>
    );
}
