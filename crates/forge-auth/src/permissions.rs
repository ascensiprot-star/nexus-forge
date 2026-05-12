use forge_core::types::{AgentRole, RiskLevel};

pub fn requires_approval(agent_role: AgentRole, action: &str, risk: RiskLevel) -> bool {
    match risk {
        RiskLevel::Critical => true,
        RiskLevel::High => true,
        RiskLevel::Medium => matches!(
            action,
            "delete_file" | "modify_schema" | "add_dependency" | "deploy"
        ),
        RiskLevel::Low => false,
    }
}

pub fn agent_allowed_actions(role: AgentRole) -> &'static [&'static str] {
    match role {
        AgentRole::SoftwareEngineer => &[
            "read_file",
            "write_file",
            "create_file",
            "run_tests",
            "search_code",
        ],
        AgentRole::CodeReviewer => &["read_file", "comment", "approve", "reject", "search_code"],
        AgentRole::DevopsEngineer => &[
            "read_file",
            "write_file",
            "create_file",
            "run_command",
            "deploy",
        ],
        AgentRole::SecurityEngineer => &["read_file", "scan", "search_code", "review"],
        AgentRole::QaEngineer => &["read_file", "run_tests", "write_file", "create_file"],
        _ => &["read_file", "write_file", "create_file", "search_code"],
    }
}
