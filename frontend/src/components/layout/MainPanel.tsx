import { Component, Match, Switch } from 'solid-js';
import { ForgePrompt } from '../forge-prompt/ForgePrompt';
import { BoardView } from '../board/BoardView';
import { CrewView } from '../crew/CrewView';

interface MainPanelProps {
  activeView: string;
}

export const MainPanel: Component<MainPanelProps> = (props) => {
  return (
    <main style={{
      flex: '1',
      display: 'flex',
      'flex-direction': 'column',
      'min-width': '0',
      background: '#0e0e16',
    }}>
      <Switch fallback={<ForgePrompt />}>
        <Match when={props.activeView === 'prompt'}>
          <ForgePrompt />
        </Match>
        <Match when={props.activeView === 'board'}>
          <BoardView />
        </Match>
        <Match when={props.activeView === 'crew'}>
          <CrewView />
        </Match>
        <Match when={props.activeView === 'editor'}>
          <PlaceholderView name="Editor" />
        </Match>
        <Match when={props.activeView === 'memory'}>
          <PlaceholderView name="Memory" />
        </Match>
        <Match when={props.activeView === 'vault'}>
          <PlaceholderView name="Vault" />
        </Match>
        <Match when={props.activeView === 'ship'}>
          <PlaceholderView name="Ship" />
        </Match>
        <Match when={props.activeView === 'lens'}>
          <PlaceholderView name="Lens" />
        </Match>
      </Switch>
    </main>
  );
};

const PlaceholderView: Component<{ name: string }> = (props) => {
  return (
    <div style={{
      display: 'flex',
      'align-items': 'center',
      'justify-content': 'center',
      height: '100%',
      color: '#555',
      'font-size': '24px',
    }}>
      {props.name}
    </div>
  );
};
