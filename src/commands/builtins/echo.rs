use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};

/// Echo command - prints arguments to stdout
pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String], _registry: &CommandRegistry) -> CommandResult {
        let output = args.join(" ");
        Ok(CommandOutput::Message(output))
    }

    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Display a line of text"
    }
}
