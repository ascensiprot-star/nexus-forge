import { Component } from 'solid-js';

export const StatusBar: Component = () => {
  return (
    <footer style={{
      height: '24px',
      background: '#12121a',
      'border-top': '1px solid #1e1e2e',
      display: 'flex',
      'align-items': 'center',
      'justify-content': 'space-between',
      padding: '0 12px',
      'font-size': '11px',
      color: '#555',
    }}>
      <span>Nexus Forge v0.1.0</span>
      <span>Ready</span>
    </footer>
  );
};
