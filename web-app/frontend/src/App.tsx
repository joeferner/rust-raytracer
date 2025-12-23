import styles from './App.module.scss';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { Provider as JotaiProvider, useSetAtom } from 'jotai';
import { Render } from './components/Render';
import { Navbar } from './components/Navbar';
import { useEffect, type JSX } from 'react';
import { loadExampleProjectAtom } from './store';
import { Example } from './utils/examples';

export function App(): JSX.Element {
    return (
        <JotaiProvider>
            <InnerApp />
        </JotaiProvider>
    );
}

function InnerApp(): JSX.Element {
    const loadExampleProject = useSetAtom(loadExampleProjectAtom);

    useEffect(() => {
        console.log('load initial project');
        void loadExampleProject(Example.ThreeSpheres);
    }, [loadExampleProject]);

    return (
        <div className={styles.main}>
            <Navbar />
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
    );
}
