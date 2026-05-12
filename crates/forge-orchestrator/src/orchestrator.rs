use std::collections::HashMap;
use std::sync::Arc;

use forge_core::error::{ForgeError, ForgeResult};
use forge_core::event::{self, EventMetadata, ForgeEvent};
use forge_core::traits::{EventBus, MemoryService, ModelRouter};
use forge_core::types::*;
use forge_task_graph::planner::TaskPlanner;
use forge_task_graph::{TaskGraph, TaskScheduler, TaskStateMachine};
use tokio::sync::RwLock;

use crate::agent_manager::AgentManager;

pub struct Orchestrator {
    event_bus: Arc<dyn EventBus>,
    model_router: Arc<dyn ModelRouter>,
    memory: Arc<dyn MemoryService>,
    agent_manager: Arc<RwLock<AgentManager>>,
    active_graphs: Arc<RwLock<HashMap<TaskGraphId, TaskGraph>>>,
}

impl Orchestrator {
    pub fn new(
        event_bus: Arc<dyn EventBus>,
        model_router: Arc<dyn ModelRouter>,
        memory: Arc<dyn MemoryService>,
    ) -> Self {
        Self {
            event_bus,
            model_router,
            memory,
            agent_manager: Arc::new(RwLock::new(AgentManager::new())),
            active_graphs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn submit_intent(&self, intent: IntentRequest) -> ForgeResult<IntentResponse> {
        tracing::info!(
            project_id = %intent.project_id,
            "processing intent: {}",
            &intent.intent_text[..intent.intent_text.len().min(100)]
        );

        let plan = self.generate_plan(&intent).await?;

        let graph = TaskPlanner::plan_to_graph(intent.project_id.clone(), &plan)?;
        let graph_id = graph.id.clone();

        {
            let mut graphs = self.active_graphs.write().await;
            graphs.insert(graph_id.clone(), graph);
        }

        self.event_bus
            .publish(
                event::event_types::TASK_CREATED,
                ForgeEvent::new(
                    event::event_types::TASK_CREATED,
                    "orchestrator",
                    serde_json::to_value(&plan)?,
                    EventMetadata {
                        project_id: intent.project_id.clone(),
                        user_id: Some(intent.user_id.clone()),
                        agent_id: None,
                        task_id: None,
                    },
                ),
            )
            .await?;

        Ok(IntentResponse {
            plan_id: graph_id.to_string(),
            plan,
            questions: vec![],
            requires_approval: true,
        })
    }

    pub async fn execute_plan(&self, graph_id: &TaskGraphId) -> ForgeResult<()> {
        loop {
            let ready_tasks = {
                let graphs = self.active_graphs.read().await;
                let graph = graphs.get(graph_id).ok_or_else(|| ForgeError::NotFound {
                    entity: "task_graph".to_string(),
                    id: graph_id.to_string(),
                })?;

                if graph.is_complete() {
                    tracing::info!(graph_id = %graph_id, "all tasks complete");
                    return Ok(());
                }

                graph
                    .get_ready_tasks()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>()
            };

            if ready_tasks.is_empty() {
                let graphs = self.active_graphs.read().await;
                let graph = graphs.get(graph_id).unwrap();
                if !graph.is_complete() {
                    tracing::warn!("no ready tasks but graph not complete - possible deadlock");
                    return Err(ForgeError::Internal(
                        "task graph deadlock: no ready tasks".to_string(),
                    ));
                }
                return Ok(());
            }

            for task in &ready_tasks {
                self.assign_and_execute_task(graph_id, task).await?;
            }
        }
    }

    async fn assign_and_execute_task(
        &self,
        graph_id: &TaskGraphId,
        task: &Task,
    ) -> ForgeResult<()> {
        let role = TaskScheduler::assign_agent_role(task);

        {
            let mut graphs = self.active_graphs.write().await;
            if let Some(graph) = graphs.get_mut(graph_id) {
                if let Ok(t) = graph.get_task_mut(&task.id) {
                    t.status = TaskStatus::Assigned;
                    t.assigned_agent = Some(AgentId::from_str(&role.to_string()));
                }
            }
        }

        tracing::info!(
            task_id = %task.id,
            role = %role,
            "assigned task: {}",
            task.title
        );

        {
            let mut graphs = self.active_graphs.write().await;
            if let Some(graph) = graphs.get_mut(graph_id) {
                if let Ok(t) = graph.get_task_mut(&task.id) {
                    t.status = TaskStatus::Complete;
                    t.completed_at = Some(chrono::Utc::now());
                }
            }
        }

        Ok(())
    }

    async fn generate_plan(&self, intent: &IntentRequest) -> ForgeResult<Plan> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a project planner. Break down the user's request into concrete implementation tasks. Return a JSON plan.".to_string(),
            },
            Message {
                role: MessageRole::User,
                content: intent.intent_text.clone(),
            },
        ];

        let request = CompletionRequest {
            task_type: "architecture_design".to_string(),
            messages,
            max_tokens: Some(4096),
            temperature: Some(0.3),
            preferred_model: None,
            budget: None,
            metadata: serde_json::json!({}),
        };

        match self.model_router.complete(request).await {
            Ok(response) => {
                if let Ok(plan) = serde_json::from_str::<Plan>(&response.content) {
                    Ok(plan)
                } else {
                    Ok(Plan {
                        id: uuid::Uuid::new_v4().to_string(),
                        title: "Implementation Plan".to_string(),
                        description: intent.intent_text.clone(),
                        tasks: vec![PlannedTask {
                            title: "Implement requested feature".to_string(),
                            description: intent.intent_text.clone(),
                            task_type: TaskType::Feature,
                            priority: 2,
                            agent_role: AgentRole::SoftwareEngineer,
                            dependencies: vec![],
                            estimated_minutes: 30,
                        }],
                        estimated_total_minutes: 30,
                    })
                }
            }
            Err(_) => Ok(Plan {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Implementation Plan".to_string(),
                description: intent.intent_text.clone(),
                tasks: vec![PlannedTask {
                    title: "Implement requested feature".to_string(),
                    description: intent.intent_text.clone(),
                    task_type: TaskType::Feature,
                    priority: 2,
                    agent_role: AgentRole::SoftwareEngineer,
                    dependencies: vec![],
                    estimated_minutes: 30,
                }],
                estimated_total_minutes: 30,
            }),
        }
    }

    pub async fn get_graph_status(&self, graph_id: &TaskGraphId) -> ForgeResult<GraphStatus> {
        let graphs = self.active_graphs.read().await;
        let graph = graphs.get(graph_id).ok_or_else(|| ForgeError::NotFound {
            entity: "task_graph".to_string(),
            id: graph_id.to_string(),
        })?;

        Ok(GraphStatus {
            total_tasks: graph.task_count(),
            completed_tasks: graph.completed_count(),
            is_complete: graph.is_complete(),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GraphStatus {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub is_complete: bool,
}
