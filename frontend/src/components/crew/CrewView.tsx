import { Component, For } from 'solid-js';

const agents = [
  { name: 'AI Engineer', role: 'ai_engineer', status: 'idle' },
  { name: 'Software Engineer', role: 'software_engineer', status: 'idle' },
  { name: 'Data Engineer', role: 'data_engineer', status: 'idle' },
  { name: 'Backend Architect', role: 'backend_architect', status: 'idle' },
  { name: 'Frontend Engineer', role: 'frontend_engineer', status: 'idle' },
  { name: 'DevOps Engineer', role: 'devops_engineer', status: 'idle' },
  { name: 'Security Engineer', role: 'security_engineer', status: 'idle' },
  { name: 'QA Engineer', role: 'qa_engineer', status: 'idle' },
  { name: 'Database Architect', role: 'database_architect', status: 'idle' },
  { name: 'Performance Engineer', role: 'performance_engineer', status: 'idle' },
  { name: 'Documentation Engineer', role: 'documentation_engineer', status: 'idle' },
  { name: 'Code Reviewer', role: 'code_reviewer', status: 'idle' },
];

export const CrewView: Component = () => {
  return (
    <div style={{ padding: '24px', height: '100%', overflow: 'auto' }}>
      <h2 style={{ color: '#e0e0e8', 'margin-bottom': '16px', 'font-size': '18px' }}>
        Agent Crew
      </h2>
      <div style={{
        display: 'grid',
        'grid-template-columns': 'repeat(auto-fill, minmax(240px, 1fr))',
        gap: '12px',
      }}>
        <For each={agents}>
          {(agent) => (
            <div style={{
              background: '#12121a',
              'border-radius': '8px',
              padding: '16px',
              border: '1px solid #1e1e2e',
            }}>
              <div style={{
                display: 'flex',
                'justify-content': 'space-between',
                'align-items': 'center',
              }}>
                <span style={{ color: '#e0e0e8', 'font-size': '13px', 'font-weight': '600' }}>
                  {agent.name}
                </span>
                <span style={{
                  color: '#4ade80',
                  'font-size': '11px',
                  background: '#16331e',
                  padding: '2px 8px',
                  'border-radius': '4px',
                }}>
                  {agent.status}
                </span>
              </div>
              <div style={{ color: '#555', 'font-size': '11px', 'margin-top': '4px' }}>
                {agent.role}
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  );
};
