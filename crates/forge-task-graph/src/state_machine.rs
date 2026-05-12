use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::TaskStatus;

pub struct TaskStateMachine;

impl TaskStateMachine {
    pub fn validate_transition(from: TaskStatus, to: TaskStatus) -> ForgeResult<()> {
        if from.can_transition_to(&to) {
            Ok(())
        } else {
            Err(ForgeError::InvalidTransition {
                entity: "task".to_string(),
                from: from.to_string(),
                to: to.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(
            TaskStateMachine::validate_transition(TaskStatus::Pending, TaskStatus::Assigned)
                .is_ok()
        );
        assert!(TaskStateMachine::validate_transition(
            TaskStatus::Assigned,
            TaskStatus::InProgress
        )
        .is_ok());
        assert!(TaskStateMachine::validate_transition(
            TaskStatus::InProgress,
            TaskStatus::Validating
        )
        .is_ok());
        assert!(TaskStateMachine::validate_transition(
            TaskStatus::Validating,
            TaskStatus::Reviewing
        )
        .is_ok());
        assert!(
            TaskStateMachine::validate_transition(TaskStatus::Reviewing, TaskStatus::Complete)
                .is_ok()
        );
    }

    #[test]
    fn test_invalid_transition() {
        assert!(
            TaskStateMachine::validate_transition(TaskStatus::Pending, TaskStatus::Complete)
                .is_err()
        );
        assert!(
            TaskStateMachine::validate_transition(TaskStatus::Complete, TaskStatus::Pending)
                .is_err()
        );
    }
}
