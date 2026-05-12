import { Component } from 'solid-js';

export const BoardView: Component = () => {
  return (
    <div style={{
      padding: '24px',
      height: '100%',
      overflow: 'auto',
    }}>
      <h2 style={{ color: '#e0e0e8', 'margin-bottom': '16px', 'font-size': '18px' }}>
        Task Board
      </h2>
      <div style={{
        display: 'grid',
        'grid-template-columns': 'repeat(4, 1fr)',
        gap: '16px',
        height: 'calc(100% - 50px)',
      }}>
        <BoardColumn title="Pending" count={0} />
        <BoardColumn title="In Progress" count={0} />
        <BoardColumn title="Reviewing" count={0} />
        <BoardColumn title="Complete" count={0} />
      </div>
    </div>
  );
};

const BoardColumn: Component<{ title: string; count: number }> = (props) => {
  return (
    <div style={{
      background: '#12121a',
      'border-radius': '8px',
      padding: '12px',
      border: '1px solid #1e1e2e',
    }}>
      <div style={{
        display: 'flex',
        'justify-content': 'space-between',
        'margin-bottom': '12px',
      }}>
        <span style={{ color: '#888', 'font-size': '13px', 'font-weight': '600' }}>
          {props.title}
        </span>
        <span style={{ color: '#555', 'font-size': '12px' }}>
          {props.count}
        </span>
      </div>
      <div style={{ color: '#444', 'font-size': '12px', 'text-align': 'center', padding: '24px 0' }}>
        No tasks
      </div>
    </div>
  );
};
