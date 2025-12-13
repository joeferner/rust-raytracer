import styles from './App.module.scss';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { MyProvider } from './state';
import { Render } from './components/Render';
import { Toolbar } from './components/Toolbar';
import type { JSX } from 'react';

export function App(): JSX.Element {
    return (
        <MyProvider>
            <div className={styles.main}>
                <Toolbar />
                <PanelGroup autoSaveId="example" direction="horizontal">
                    <Panel defaultSize={50}>
                        <Files />
                    </Panel>
                    <PanelResizeHandle className="resizeHandle" />
                    <Panel>
                        <Render />
                    </Panel>
                </PanelGroup>
            </div>
        </MyProvider>
    );
}
