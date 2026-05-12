use forge_core::types::{AgentRole, Task, TaskType};

pub struct TaskScheduler;

impl TaskScheduler {
    pub fn assign_agent_role(task: &Task) -> AgentRole {
        match task.task_type {
            TaskType::Epic | TaskType::Feature => AgentRole::BackendArchitect,
            TaskType::Task | TaskType::Subtask => AgentRole::SoftwareEngineer,
            TaskType::BugFix => AgentRole::SoftwareEngineer,
            TaskType::Refactor => AgentRole::SoftwareEngineer,
            TaskType::Investigation => AgentRole::SoftwareEngineer,
            TaskType::Review => AgentRole::CodeReviewer,
            TaskType::Test => AgentRole::QaEngineer,
            TaskType::Documentation => AgentRole::DocumentationEngineer,
            TaskType::Deployment => AgentRole::DevopsEngineer,
        }
    }

    pub fn priority_score(task: &Task) -> i32 {
        let base = (6 - task.priority as i32) * 100;

        let type_bonus = match task.task_type {
            TaskType::BugFix => 50,
            TaskType::Deployment => 30,
            TaskType::Feature => 20,
            _ => 0,
        };

        base + type_bonus
    }

    pub fn sort_by_priority(tasks: &mut [&Task]) {
        tasks.sort_by(|a, b| Self::priority_score(b).cmp(&Self::priority_score(a)));
    }
}
