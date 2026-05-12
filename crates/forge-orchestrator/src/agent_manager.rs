use std::collections::HashMap;

use forge_core::types::*;

pub struct AgentManager {
    agents: HashMap<AgentId, AgentInfo>,
}

impl AgentManager {
    pub fn new() -> Self {
        let mut agents = HashMap::new();

        let roles = vec![
            (AgentRole::AiEngineer, "AI Engineer"),
            (AgentRole::SoftwareEngineer, "Software Engineer"),
            (AgentRole::DataEngineer, "Data Engineer"),
            (AgentRole::BackendArchitect, "Backend Architect"),
            (AgentRole::FrontendEngineer, "Frontend Engineer"),
            (AgentRole::DevopsEngineer, "DevOps Engineer"),
            (AgentRole::SecurityEngineer, "Security Engineer"),
            (AgentRole::QaEngineer, "QA Engineer"),
            (AgentRole::DatabaseArchitect, "Database Architect"),
            (AgentRole::PerformanceEngineer, "Performance Engineer"),
            (AgentRole::DocumentationEngineer, "Documentation Engineer"),
            (AgentRole::CodeReviewer, "Code Reviewer"),
        ];

        for (role, name) in roles {
            let id = AgentId::from_str(&role.to_string());
            agents.insert(
                id.clone(),
                AgentInfo {
                    id: id.clone(),
                    name: name.to_string(),
                    role,
                    status: AgentStatus::Idle,
                    current_task: None,
                    capabilities: vec![],
                },
            );
        }

        Self { agents }
    }

    pub fn get_agent(&self, id: &AgentId) -> Option<&AgentInfo> {
        self.agents.get(id)
    }

    pub fn get_agent_by_role(&self, role: AgentRole) -> Option<&AgentInfo> {
        self.agents.values().find(|a| a.role == role)
    }

    pub fn set_status(&mut self, id: &AgentId, status: AgentStatus) {
        if let Some(agent) = self.agents.get_mut(id) {
            agent.status = status;
        }
    }

    pub fn assign_task(&mut self, agent_id: &AgentId, task_id: TaskId) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = AgentStatus::Preparing;
            agent.current_task = Some(task_id);
        }
    }

    pub fn complete_task(&mut self, agent_id: &AgentId) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.status = AgentStatus::Idle;
            agent.current_task = None;
        }
    }

    pub fn list_agents(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    pub fn idle_agents(&self) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|a| a.status == AgentStatus::Idle)
            .collect()
    }

    pub fn active_agents(&self) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|a| a.status != AgentStatus::Idle)
            .collect()
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}
