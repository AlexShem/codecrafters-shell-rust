use super::{Command, CommandOutput, CommandResult};

/// Echo command - prints arguments to stdout
pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
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
