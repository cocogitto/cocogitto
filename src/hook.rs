use anyhow::Result;
use colored::*;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hook {
    pub command: String,
}

impl Hook {
    pub(crate) fn run(&self) -> Result<()> {
        let command_display = format!("`{}`", &self.command.green());
        println!("Running pre-version hook : {}", command_display);

        Command::new(self.command.clone())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .output()
            .map_err(|err| anyhow!(err))
            .map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use crate::hook::Hook;

    #[test]
    fn should_run_command() {
        // Arrange
        let hook = Hook {
            command: "echo hello world".to_string(),
        };

        // Act
        let result = hook.run();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_to_run_invalid_command() {
        // Arrange
        let hook = Hook {
            command: "azmroih".to_string(),
        };

        // Act
        let result = hook.run();

        // Assert
        assert!(result.is_err());
    }
}
