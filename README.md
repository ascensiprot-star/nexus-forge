# Nexus Forge

**AI-native developer workspace that turns ideas into production-ready software.**

Nexus Forge is an autonomous software engineering platform. A single natural-language prompt is transformed into a complete, production-ready software project through coordinated specialized agents, multi-model orchestration, persistent project memory, and intelligent execution.

## Architecture

```
┌─────────────────────────────────────────────┐
│                 Nexus Forge                  │
├──────────┬──────────┬───────────┬───────────┤
│ Frontend │   Core   │  Agents   │  Storage  │
│ SolidJS  │   Rust   │  Python   │  SQLite   │
│  Tauri   │  Tokio   │  httpx    │  DuckDB   │
│          │  gRPC    │  pydantic │  usearch  │
└──────────┴──────────┴───────────┴───────────┘
```

### Core Services (Rust)

| Crate | Purpose |
|-------|---------|
| `forge-core` | Shared types, traits, error handling |
| `forge-eventbus` | Pub/sub event bus (in-memory + NATS) |
| `forge-project` | Project CRUD + SQLite persistence |
| `forge-task-graph` | DAG-based task scheduling |
| `forge-memory` | 7-layer persistent memory |
| `forge-model-router` | Multi-model routing + budget tracking |
| `forge-orchestrator` | Agent lifecycle + task coordination |
| `forge-code-index` | Tree-sitter symbol extraction |
| `forge-search` | Full-text + semantic search |
| `forge-auth` | RBAC, secrets vault, audit trail |
| `forge-execution` | Sandboxed code execution |
| `forge-validation` | Lint, typecheck, test pipeline |
| `forge-observability` | Structured logging + metrics |
| `forge-plugin` | Plugin runtime + SDK |
| `forge-app` | CLI binary + server |

### Agent Crew (Python)

12 specialized agents following the Observe → Think → Act → Reflect loop:
- AI Engineer, Software Engineer, Data Engineer
- Backend Architect, Frontend Engineer, DevOps Engineer
- Security Engineer, QA Engineer, Database Architect
- Performance Engineer, Documentation Engineer, Code Reviewer

### Frontend (SolidJS + Tauri)

- ForgePrompt: Natural language command input
- TaskBoard: Kanban-style task visualization
- CrewView: Agent activity dashboard
- Editor, Memory, Vault, Ship, Lens views

## Quick Start

### Prerequisites

- Rust 1.80+ (with cargo)
- Python 3.11+
- Node.js 20+ (with pnpm)

### Build

```bash
# Rust workspace
cargo build --workspace
cargo test --workspace

# Python agents
cd agents
pip install -e ".[dev]"
pytest tests/ -v

# Frontend
cd frontend
pnpm install
pnpm dev
```

### Run

```bash
# Start the server
cargo run --bin nexus-forge -- serve

# Initialize a project
cargo run --bin nexus-forge -- init ./my-project

# Check status
cargo run --bin nexus-forge -- status
```

## Project Structure

```
nexus-forge/
├── crates/              # Rust workspace (15 crates)
├── agents/              # Python agent runtime
│   └── forge_agents/    # Agent implementations
├── frontend/            # SolidJS + Tauri UI
├── proto/forge/         # gRPC protobuf definitions
├── schemas/
│   └── migrations/      # SQLite migrations
├── docs/                # Architecture & API docs
└── .github/workflows/   # CI pipeline
```

## License

MIT
