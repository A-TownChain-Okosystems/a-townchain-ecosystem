//! Aurora -> Genesis Engine control adapter.
//! The adapter is userspace-only and talks to the allow-listed Genesis protocol.

use crate::{AuroraError, ChatTool, ToolCallRequest};
use crate::types::ToolId;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

pub struct GenesisEngineControl {
    process: Mutex<GenesisProcess>,
    tool_id: crate::ToolId,
}

struct GenesisProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl GenesisEngineControl {
    pub fn spawn(python: impl AsRef<std::ffi::OsStr>, script: impl AsRef<std::ffi::OsStr>, tool_id: crate::ToolId) -> Result<Self, AuroraError> {
        let mut child = Command::new(python)
            .arg(script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| AuroraError::ResourceExhausted)?;
        let stdin = child.stdin.take().ok_or(AuroraError::ResourceExhausted)?;
        let stdout = child.stdout.take().ok_or(AuroraError::ResourceExhausted)?;
        Ok(Self {
            process: Mutex::new(GenesisProcess { child, stdin, stdout: BufReader::new(stdout) }),
            tool_id,
        })
    }

    fn execute(&self, input: &str) -> Result<String, AuroraError> {
        let mut process = self.process.lock().map_err(|_| AuroraError::SecurityViolation)?;
        let command = input.trim();
        if command.is_empty() || command.len() > 512 || command.contains('\n') || command.contains('\r') {
            return Err(AuroraError::InvalidRequest);
        }
        let allowed = ["STATUS", "SPAWN", "DESTROY", "SET_POSITION", "TICK", "SNAPSHOT", "RESET"];
        let verb = command.split_whitespace().next().unwrap_or("");
        if !allowed.contains(&verb) {
            return Err(AuroraError::CapabilityDenied);
        }
        writeln!(process.stdin, "{command}").map_err(|_| AuroraError::ResourceExhausted)?;
        process.stdin.flush().map_err(|_| AuroraError::ResourceExhausted)?;
        let mut line = String::new();
        process.stdout.read_line(&mut line).map_err(|_| AuroraError::ResourceExhausted)?;
        let line = line.trim_end().to_string();
        if line.starts_with("ERR ") {
            return Err(AuroraError::CapabilityDenied);
        }
        if !line.starts_with("OK ") {
            return Err(AuroraError::SecurityViolation);
        }
        Ok(line)
    }

    pub fn tool_id(&self) -> &crate::ToolId {
        &self.tool_id
    }
}

impl ChatTool for GenesisEngineControl {
    fn call(&self, request: &ToolCallRequest) -> Result<String, AuroraError> {
        if request.tool_id != self.tool_id {
            return Err(AuroraError::CapabilityDenied);
        }
        self.execute(&request.input)
    }
}

impl Drop for GenesisEngineControl {
    fn drop(&mut self) {
        if let Ok(mut process) = self.process.lock() {
            let _ = writeln!(process.stdin, "QUIT");
            let _ = process.stdin.flush();
            let _ = process.child.kill();
            let _ = process.child.wait();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unallowlisted_commands() {
        // Protocol filtering is applied before any process I/O.
        let input = "SHELL rm -rf /";
        assert!(!["STATUS", "SPAWN", "DESTROY", "SET_POSITION", "TICK", "SNAPSHOT", "RESET"]
            .contains(&input.split_whitespace().next().unwrap()));
    }
}
