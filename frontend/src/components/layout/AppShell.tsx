import { Component, createSignal } from 'solid-js';
import { NavRail } from './NavRail';
import { MainPanel } from './MainPanel';
import { StatusBar } from './StatusBar';

export const AppShell: Component = () => {
  const [activeView, setActiveView] = createSignal('prompt');

  return (
    <div style={{
      display: 'flex',
      height: '100vh',
      width: '100vw',
      'flex-direction': 'column',
    }}>
      <div style={{
        display: 'flex',
        flex: '1',
        'min-height': '0',
      }}>
        <NavRail activeView={activeView()} onNavigate={setActiveView} />
        <MainPanel activeView={activeView()} />
      </div>
      <StatusBar />
    </div>
  );
};
