import { Component, createSignal } from 'solid-js';

export const ForgePrompt: Component = () => {
  const [input, setInput] = createSignal('');
  const [isProcessing, setIsProcessing] = createSignal(false);

  const handleSubmit = async () => {
    const text = input().trim();
    if (!text || isProcessing()) return;

    setIsProcessing(true);
    // TODO: send to backend via Tauri IPC
    console.log('Intent submitted:', text);
    setInput('');
    setIsProcessing(false);
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div style={{
      display: 'flex',
      'flex-direction': 'column',
      'align-items': 'center',
      'justify-content': 'center',
      height: '100%',
      padding: '24px',
    }}>
      <h1 style={{
        'font-size': '32px',
        'font-weight': '700',
        color: '#e0e0e8',
        'margin-bottom': '8px',
        'letter-spacing': '-0.5px',
      }}>
        Nexus Forge
      </h1>
      <p style={{
        color: '#666',
        'margin-bottom': '32px',
        'font-size': '14px',
      }}>
        Describe what you want to build
      </p>
      <div style={{
        width: '100%',
        'max-width': '640px',
        position: 'relative',
      }}>
        <textarea
          value={input()}
          onInput={(e) => setInput(e.currentTarget.value)}
          onKeyDown={handleKeyDown}
          placeholder="Build a REST API for a todo app with user authentication..."
          disabled={isProcessing()}
          style={{
            width: '100%',
            'min-height': '120px',
            background: '#16161e',
            border: '1px solid #2a2a3a',
            'border-radius': '12px',
            padding: '16px',
            color: '#e0e0e8',
            'font-size': '14px',
            'font-family': 'inherit',
            resize: 'vertical',
            outline: 'none',
          }}
        />
        <button
          onClick={handleSubmit}
          disabled={!input().trim() || isProcessing()}
          style={{
            position: 'absolute',
            bottom: '12px',
            right: '12px',
            background: input().trim() ? '#7c3aed' : '#2a2a3a',
            color: input().trim() ? '#fff' : '#555',
            border: 'none',
            'border-radius': '8px',
            padding: '8px 20px',
            cursor: input().trim() ? 'pointer' : 'default',
            'font-size': '13px',
            'font-weight': '600',
          }}
        >
          {isProcessing() ? 'Processing...' : 'Forge'}
        </button>
      </div>
    </div>
  );
};
