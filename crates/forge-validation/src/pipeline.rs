use async_trait::async_trait;
use forge_core::error::ForgeResult;
use forge_core::traits::ValidationService;
use forge_core::types::*;

use crate::runners::CheckRunner;

pub struct ValidationPipeline {
    runner: CheckRunner,
}

impl ValidationPipeline {
    pub fn new() -> Self {
        Self {
            runner: CheckRunner::new(),
        }
    }
}

impl Default for ValidationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ValidationService for ValidationPipeline {
    async fn run_validation(
        &self,
        project_path: &str,
        checks: &[ValidationType],
    ) -> ForgeResult<ValidationResult> {
        let mut results = Vec::new();

        for check_type in checks {
            let check = self.runner.run_check(project_path, *check_type).await?;
            results.push(check);
        }

        let all_passed = results.iter().all(|c| c.passed);
        let summary = if all_passed {
            format!("All {} checks passed", results.len())
        } else {
            let failed: Vec<_> = results
                .iter()
                .filter(|c| !c.passed)
                .map(|c| c.name.clone())
                .collect();
            format!("Failed checks: {}", failed.join(", "))
        };

        Ok(ValidationResult {
            passed: all_passed,
            checks: results,
            summary,
        })
    }
}
