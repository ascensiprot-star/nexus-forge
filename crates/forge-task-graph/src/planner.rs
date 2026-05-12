use chrono::Utc;
use forge_core::error::ForgeResult;
use forge_core::types::*;

use crate::graph::TaskGraph;

pub struct TaskPlanner;

impl TaskPlanner {
    pub fn plan_to_graph(project_id: ProjectId, plan: &Plan) -> ForgeResult<TaskGraph> {
        let mut graph = TaskGraph::new(project_id, plan.description.clone());

        let mut title_to_id: std::collections::HashMap<String, TaskId> =
            std::collections::HashMap::new();

        for planned in &plan.tasks {
            let task_id = TaskId::new();
            title_to_id.insert(planned.title.clone(), task_id.clone());
        }

        for planned in &plan.tasks {
            let task_id = title_to_id.get(&planned.title).unwrap().clone();

            let dep_ids: Vec<TaskId> = planned
                .dependencies
                .iter()
                .filter_map(|dep_title| title_to_id.get(dep_title).cloned())
                .collect();

            let task = Task {
                id: task_id,
                graph_id: graph.id.clone(),
                parent_id: None,
                title: planned.title.clone(),
                description: planned.description.clone(),
                task_type: planned.task_type,
                status: TaskStatus::Pending,
                priority: planned.priority,
                assigned_agent: None,
                estimated_effort: Some(EffortEstimate {
                    optimistic_minutes: (planned.estimated_minutes as f64 * 0.7) as u32,
                    expected_minutes: planned.estimated_minutes,
                    pessimistic_minutes: (planned.estimated_minutes as f64 * 1.5) as u32,
                    confidence: 0.7,
                }),
                actual_effort_minutes: None,
                inputs: vec![],
                outputs: vec![],
                constraints: vec![],
                dependencies: dep_ids,
                metadata: serde_json::json!({"agent_role": planned.agent_role.to_string()}),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                completed_at: None,
            };

            graph.add_task(task)?;
        }

        Ok(graph)
    }
}
