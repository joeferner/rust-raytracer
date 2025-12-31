import classes from './App.module.scss';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { Provider as JotaiProvider, useSetAtom } from 'jotai';
import { Render } from './components/Render';
import { Navbar } from './components/Navbar';
import { useEffect, type JSX } from 'react';
import { initializeAtom } from './store';
import { Header } from './components/Header';
import { ModalsProvider } from '@mantine/modals';
import { MantineProvider } from '@mantine/core';

export function App(): JSX.Element {
    return (
        <JotaiProvider>
            <MantineProvider>
                <ModalsProvider>
                    <InnerApp />
                </ModalsProvider>
            </MantineProvider>
        </JotaiProvider>
    );
}

function InnerApp(): JSX.Element {
    const initialize = useSetAtom(initializeAtom);

    useEffect(() => {
        void initialize();
    }, [initialize]);

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
