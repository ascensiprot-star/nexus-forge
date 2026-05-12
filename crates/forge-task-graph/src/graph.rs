use std::collections::{HashMap, HashSet};

use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::*;

use crate::dependency::DependencyResolver;

pub struct TaskGraph {
    pub id: TaskGraphId,
    pub project_id: ProjectId,
    pub intent_text: String,
    tasks: HashMap<TaskId, Task>,
    dependencies: HashMap<TaskId, HashSet<TaskId>>,
    dependents: HashMap<TaskId, HashSet<TaskId>>,
}

impl TaskGraph {
    pub fn new(project_id: ProjectId, intent_text: String) -> Self {
        Self {
            id: TaskGraphId::new(),
            project_id,
            intent_text,
            tasks: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) -> ForgeResult<()> {
        let task_id = task.id.clone();

        for dep_id in &task.dependencies {
            if !self.tasks.contains_key(dep_id) {
                return Err(ForgeError::NotFound {
                    entity: "dependency task".to_string(),
                    id: dep_id.to_string(),
                });
            }

            self.dependencies
                .entry(task_id.clone())
                .or_default()
                .insert(dep_id.clone());

            self.dependents
                .entry(dep_id.clone())
                .or_default()
                .insert(task_id.clone());
        }

        self.tasks.insert(task_id, task);

        if DependencyResolver::has_cycle(&self.dependencies) {
            let last_task_id = self.tasks.keys().last().unwrap().clone();
            self.tasks.remove(&last_task_id);
            self.dependencies.remove(&last_task_id);
            return Err(ForgeError::CyclicDependency {
                task_id: last_task_id.to_string(),
            });
        }

        Ok(())
    }

    pub fn get_task(&self, task_id: &TaskId) -> ForgeResult<&Task> {
        self.tasks.get(task_id).ok_or_else(|| ForgeError::NotFound {
            entity: "task".to_string(),
            id: task_id.to_string(),
        })
    }

    pub fn get_task_mut(&mut self, task_id: &TaskId) -> ForgeResult<&mut Task> {
        self.tasks
            .get_mut(task_id)
            .ok_or_else(|| ForgeError::NotFound {
                entity: "task".to_string(),
                id: task_id.to_string(),
            })
    }

    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn get_ready_tasks(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.status == TaskStatus::Pending && self.all_deps_complete(&task.id))
            .collect()
    }

    pub fn all_deps_complete(&self, task_id: &TaskId) -> bool {
        match self.dependencies.get(task_id) {
            None => true,
            Some(deps) => deps.iter().all(|dep_id| {
                self.tasks
                    .get(dep_id)
                    .map(|t| t.status == TaskStatus::Complete || t.status == TaskStatus::Deployed)
                    .unwrap_or(false)
            }),
        }
    }

    pub fn get_dependents(&self, task_id: &TaskId) -> Vec<&TaskId> {
        self.dependents
            .get(task_id)
            .map(|deps| deps.iter().collect())
            .unwrap_or_default()
    }

    pub fn get_dependencies(&self, task_id: &TaskId) -> Vec<&TaskId> {
        self.dependencies
            .get(task_id)
            .map(|deps| deps.iter().collect())
            .unwrap_or_default()
    }

    pub fn critical_path(&self) -> Vec<TaskId> {
        DependencyResolver::critical_path(&self.tasks, &self.dependencies)
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn completed_count(&self) -> usize {
        self.tasks
            .values()
            .filter(|t| t.status.is_terminal())
            .count()
    }

    pub fn is_complete(&self) -> bool {
        self.tasks.values().all(|t| t.status.is_terminal())
    }

    pub fn topological_order(&self) -> ForgeResult<Vec<TaskId>> {
        DependencyResolver::topological_sort(&self.tasks, &self.dependencies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn make_task(id: &str, deps: Vec<&str>) -> Task {
        Task {
            id: TaskId::from_str(id),
            graph_id: TaskGraphId::from_str("graph-1"),
            parent_id: None,
            title: format!("Task {id}"),
            description: String::new(),
            task_type: TaskType::Task,
            status: TaskStatus::Pending,
            priority: 3,
            assigned_agent: None,
            estimated_effort: None,
            actual_effort_minutes: None,
            inputs: vec![],
            outputs: vec![],
            constraints: vec![],
            dependencies: deps.into_iter().map(|d| TaskId::from_str(d)).collect(),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        }
    }

    #[test]
    fn test_add_and_get_tasks() {
        let mut graph = TaskGraph::new(ProjectId::new(), "test".to_string());
        let task = make_task("t1", vec![]);
        graph.add_task(task).unwrap();

        assert_eq!(graph.task_count(), 1);
        assert_eq!(
            graph.get_task(&TaskId::from_str("t1")).unwrap().title,
            "Task t1"
        );
    }

    #[test]
    fn test_ready_tasks() {
        let mut graph = TaskGraph::new(ProjectId::new(), "test".to_string());
        graph.add_task(make_task("t1", vec![])).unwrap();
        graph.add_task(make_task("t2", vec!["t1"])).unwrap();
        graph.add_task(make_task("t3", vec![])).unwrap();

        let ready = graph.get_ready_tasks();
        let ready_ids: HashSet<_> = ready.iter().map(|t| t.id.as_str()).collect();
        assert!(ready_ids.contains("t1"));
        assert!(ready_ids.contains("t3"));
        assert!(!ready_ids.contains("t2"));
    }

    #[test]
    fn test_deps_complete_unlocks() {
        let mut graph = TaskGraph::new(ProjectId::new(), "test".to_string());
        graph.add_task(make_task("t1", vec![])).unwrap();
        graph.add_task(make_task("t2", vec!["t1"])).unwrap();

        assert!(!graph.all_deps_complete(&TaskId::from_str("t2")));

        graph.get_task_mut(&TaskId::from_str("t1")).unwrap().status = TaskStatus::Complete;

        assert!(graph.all_deps_complete(&TaskId::from_str("t2")));
    }
}
