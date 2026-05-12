use forge_core::types::*;

pub struct EscalationPolicy {
    pub max_revision_cycles: u32,
    pub auto_approve_low_risk: bool,
    pub escalation_chain: Vec<EscalationStep>,
}

pub struct EscalationStep {
    pub trigger: EscalationTrigger,
    pub action: EscalationAction,
}

pub enum EscalationTrigger {
    MaxRetriesExceeded,
    BudgetExceeded,
    QualityBelowThreshold,
    TaskTimeout,
}

pub enum EscalationAction {
    UpgradeModel,
    ReassignAgent(AgentRole),
    RequestHumanApproval,
    FailTask,
}

impl Default for EscalationPolicy {
    fn default() -> Self {
        Self {
            max_revision_cycles: 5,
            auto_approve_low_risk: false,
            escalation_chain: vec![
                EscalationStep {
                    trigger: EscalationTrigger::MaxRetriesExceeded,
                    action: EscalationAction::UpgradeModel,
                },
                EscalationStep {
                    trigger: EscalationTrigger::BudgetExceeded,
                    action: EscalationAction::RequestHumanApproval,
                },
                EscalationStep {
                    trigger: EscalationTrigger::QualityBelowThreshold,
                    action: EscalationAction::ReassignAgent(AgentRole::CodeReviewer),
                },
                EscalationStep {
                    trigger: EscalationTrigger::TaskTimeout,
                    action: EscalationAction::FailTask,
                },
            ],
        }
    }
}
