use std::process::Stdio;
use std::time::Duration;

use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::*;

pub struct ProcessRunner;

impl ProcessRunner {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(
        &self,
        sandbox_id: &SandboxId,
        command: &str,
        working_dir: Option<&str>,
        timeout: Duration,
    ) -> ForgeResult<ExecutionResult> {
        let start = std::time::Instant::now();

        let mut cmd = tokio::process::Command::new("sh");
        cmd.arg("-c").arg(command);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        let child = cmd.spawn().map_err(|e| ForgeError::ExecutionError {
            message: format!("failed to spawn process: {e}"),
        })?;

        let output = match tokio::time::timeout(timeout, child.wait_with_output()).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Err(ForgeError::ExecutionError {
                    message: format!("process error: {e}"),
                });
            }
            Err(_) => {
                return Err(ForgeError::Timeout {
                    operation: format!("sandbox {sandbox_id} command execution"),
                    timeout_ms: timeout.as_millis() as u64,
                });
            }
        };

        Ok(ExecutionResult {
            sandbox_id: sandbox_id.clone(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
            artifacts: vec![],
        })
    }
}

impl Default for ProcessRunner {
    fn default() -> Self {
        Self::new()
    }
}
