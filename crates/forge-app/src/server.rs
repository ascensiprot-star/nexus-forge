use anyhow::Result;
use forge_orchestrator::Orchestrator;
use forge_project::ProjectStore;

pub async fn run(port: u16, project_store: ProjectStore, orchestrator: Orchestrator) -> Result<()> {
    tracing::info!("Nexus Forge server listening on port {port}");
    tracing::info!("Press Ctrl+C to stop");

    tokio::signal::ctrl_c().await?;
    tracing::info!("shutting down");

    Ok(())
}
