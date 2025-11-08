use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};
use std::env;
use std::path::PathBuf;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &[String], _registry: &CommandRegistry) -> CommandResult {
        if args.is_empty() {
            return Err("cd: missing argument".to_string());
        }

        let path = if &args[0] == "~" {
            if let Some(home) = env::home_dir() {
                home
            } else {
                return Ok(CommandOutput::Message(
                    "Home directory is not set".to_string(),
                ));
            }
        } else {
            PathBuf::from(&args[0])
        };

        if !path.exists() {
            return Ok(CommandOutput::Message(format!(
                "cd: {}: No such file or directory",
                path.display()
            )));
        }

        match env::set_current_dir(&path) {
            Ok(_) => Ok(CommandOutput::Success),
            Err(e) => Ok(CommandOutput::Message(format!(
                "Failed to change working directory: {}. Reason: {}",
                path.display(),
                e
            ))),
        }
    }

    fn name(&self) -> &str {
        "cd"
    }

    fn description(&self) -> &str {
        "Change directory"
    }
}
