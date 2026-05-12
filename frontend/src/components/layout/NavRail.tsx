import { Component, For } from 'solid-js';

interface NavRailProps {
  activeView: string;
  onNavigate: (view: string) => void;
}

const navItems = [
  { id: 'prompt', label: 'Forge', icon: '⚡' },
  { id: 'board', label: 'Board', icon: '📋' },
  { id: 'editor', label: 'Editor', icon: '📝' },
  { id: 'crew', label: 'Crew', icon: '🤖' },
  { id: 'memory', label: 'Memory', icon: '🧠' },
  { id: 'vault', label: 'Vault', icon: '🔐' },
  { id: 'ship', label: 'Ship', icon: '🚀' },
  { id: 'lens', label: 'Lens', icon: '🔍' },
];

export const NavRail: Component<NavRailProps> = (props) => {
  return (
    <nav style={{
      width: '52px',
      background: '#12121a',
      'border-right': '1px solid #1e1e2e',
      display: 'flex',
      'flex-direction': 'column',
      'align-items': 'center',
      'padding-top': '8px',
      gap: '2px',
    }}>
      <For each={navItems}>
        {(item) => (
          <button
            onClick={() => props.onNavigate(item.id)}
            title={item.label}
            style={{
              width: '40px',
              height: '40px',
              border: 'none',
              background: props.activeView === item.id ? '#1e1e2e' : 'transparent',
              color: props.activeView === item.id ? '#a78bfa' : '#666',
              'border-radius': '8px',
              cursor: 'pointer',
              'font-size': '18px',
              display: 'flex',
              'align-items': 'center',
              'justify-content': 'center',
            }}
          >
            {item.icon}
          </button>
        )}
      </For>
    </nav>
  );
};
