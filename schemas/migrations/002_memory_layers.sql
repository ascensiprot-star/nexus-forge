-- Nexus Forge: Memory layers

CREATE TABLE IF NOT EXISTS memory_items (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    layer TEXT NOT NULL,
    item_type TEXT NOT NULL,
    key TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    source TEXT NOT NULL,
    confidence REAL DEFAULT 1.0,
    tags TEXT DEFAULT '[]',
    metadata TEXT DEFAULT '{}',
    embedding_id TEXT,
    access_count INTEGER DEFAULT 0,
    last_accessed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    archived_at TEXT,
    UNIQUE(project_id, layer, key)
);

CREATE TABLE IF NOT EXISTS decisions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    decision_type TEXT NOT NULL,
    title TEXT NOT NULL,
    question TEXT NOT NULL,
    chosen_option TEXT NOT NULL,
    rejected_options TEXT NOT NULL DEFAULT '[]',
    reasoning TEXT NOT NULL,
    tradeoffs TEXT NOT NULL DEFAULT '[]',
    constraints TEXT DEFAULT '[]',
    decided_by TEXT NOT NULL,
    task_id TEXT,
    supersedes TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS conventions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    category TEXT NOT NULL,
    rule TEXT NOT NULL,
    examples TEXT DEFAULT '[]',
    source TEXT NOT NULL DEFAULT 'auto_detected',
    confidence REAL DEFAULT 1.0,
    active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_memory_project_layer ON memory_items(project_id, layer);
CREATE INDEX IF NOT EXISTS idx_memory_type ON memory_items(item_type);
CREATE INDEX IF NOT EXISTS idx_decisions_project ON decisions(project_id);
CREATE INDEX IF NOT EXISTS idx_decisions_type ON decisions(decision_type);
CREATE INDEX IF NOT EXISTS idx_conventions_project ON conventions(project_id);
