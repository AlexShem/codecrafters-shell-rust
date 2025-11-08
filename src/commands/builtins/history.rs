use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, args: &[String], _registry: &CommandRegistry) -> CommandResult {
        let output = args.join(" ");
        Ok(CommandOutput::Message(output))
    }

    fn name(&self) -> &str {
        "history"
    }

    fn description(&self) -> &str {
        "Lists previously executed commands"
    }
}
