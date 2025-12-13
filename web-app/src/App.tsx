import './App.module.scss'
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { MyProvider } from './state';
import { Render } from './components/Render';
import { Toolbar } from './components/Toolbar';

export function App() {
  return (
    <MyProvider>
      <Toolbar />
      <PanelGroup autoSaveId="example" direction="horizontal">
        <Panel defaultSize={25}>
          <Files />
        </Panel>
        <PanelResizeHandle />
        <Panel>
          <Render />
        </Panel>
      </PanelGroup>
    </MyProvider>
  );
}
