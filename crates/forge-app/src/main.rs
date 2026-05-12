use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use forge_core::config::ForgeConfig;
use forge_eventbus::EventBusImpl;
use forge_memory::MemoryStore;
use forge_model_router::ModelRouterService;
use forge_observability::init_logging;
use forge_orchestrator::Orchestrator;
use forge_project::ProjectStore;

mod cli;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse_args();

    let config = load_config(&args.config_path)?;
    init_logging(
        &config.general.log_level,
        config.observability.log_format == "json",
    );

    tracing::info!("Nexus Forge v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("data directory: {}", config.general.data_dir);

    let data_dir = PathBuf::from(&config.general.data_dir);
    std::fs::create_dir_all(&data_dir)?;

    let project_store = ProjectStore::open(&data_dir.join("forge.db"))?;

    let memory_store = MemoryStore::open(&data_dir.join("memory.db"))?;

    let event_bus = EventBusImpl::in_memory();

    let model_router = ModelRouterService::new();

    let orchestrator = Orchestrator::new(
        Arc::new(event_bus),
        Arc::new(model_router),
        Arc::new(memory_store),
    );

    match args.command {
        cli::Command::Serve { port } => {
            tracing::info!("starting server on port {port}");
            server::run(port, project_store, orchestrator).await?;
        }
        cli::Command::Init { path } => {
            tracing::info!("initializing project at {}", path.display());
            let project = project_store
                .create_project(
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("untitled"),
                    path.to_str().unwrap_or("."),
                )
                .await?;
            println!("Created project: {} ({})", project.name, project.id);
        }
        cli::Command::Status => {
            let projects = project_store.list_projects().await?;
            if projects.is_empty() {
                println!("No projects found.");
            } else {
                for p in &projects {
                    println!("{} | {} | {} | {}", p.id, p.name, p.status, p.root_path);
                }
            }
        }
        cli::Command::Version => {
            println!("nexus-forge {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

fn load_config(path: &Option<PathBuf>) -> Result<ForgeConfig> {
    if let Some(path) = path {
        let content = std::fs::read_to_string(path)?;
        let config: ForgeConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(ForgeConfig::default())
    }
}
