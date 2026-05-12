use forge_core::types::ProjectStatus;

pub struct ProjectStateMachine;

impl ProjectStateMachine {
    pub fn can_transition(from: ProjectStatus, to: ProjectStatus) -> bool {
        use ProjectStatus::*;
        matches!(
            (from, to),
            (Empty, Planning)
                | (Planning, Building)
                | (Planning, Paused)
                | (Building, Review)
                | (Building, Paused)
                | (Building, Error)
                | (Review, Ready)
                | (Review, Building)
                | (Review, Paused)
                | (Ready, Deployed)
                | (Ready, Building)
                | (Paused, Planning)
                | (Paused, Building)
                | (Paused, Review)
                | (Error, Planning)
                | (Error, Building)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::types::ProjectStatus::*;

    #[test]
    fn test_valid_transitions() {
        assert!(ProjectStateMachine::can_transition(Empty, Planning));
        assert!(ProjectStateMachine::can_transition(Planning, Building));
        assert!(ProjectStateMachine::can_transition(Building, Review));
        assert!(ProjectStateMachine::can_transition(Review, Ready));
        assert!(ProjectStateMachine::can_transition(Ready, Deployed));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!ProjectStateMachine::can_transition(Empty, Deployed));
        assert!(!ProjectStateMachine::can_transition(Deployed, Empty));
        assert!(!ProjectStateMachine::can_transition(Ready, Planning));
    }
}
