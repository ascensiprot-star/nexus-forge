use forge_core::types::ValidationType;

pub fn default_check_order() -> Vec<ValidationType> {
    vec![
        ValidationType::Lint,
        ValidationType::TypeCheck,
        ValidationType::UnitTest,
        ValidationType::SecurityScan,
    ]
}

pub fn full_check_order() -> Vec<ValidationType> {
    vec![
        ValidationType::Lint,
        ValidationType::TypeCheck,
        ValidationType::UnitTest,
        ValidationType::IntegrationTest,
        ValidationType::SecurityScan,
        ValidationType::AcceptanceCheck,
    ]
}
