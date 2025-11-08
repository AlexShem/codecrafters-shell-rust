use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};
use std::env;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &[String], _registry: &CommandRegistry) -> CommandResult {
        match env::current_dir() {
            Ok(current_dir) => Ok(CommandOutput::Message(format!("{}", current_dir.display()))),
            Err(e) => Err(e.to_string()),
        }
    }

    fn name(&self) -> &str {
        "pwd"
    }

    fn description(&self) -> &str {
        "Prints current working directory"
    }
}
