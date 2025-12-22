use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, _args: &[String], _registry: &CommandRegistry) -> CommandResult {
        Ok(CommandOutput::Success)
    }

    fn execute_with_history(
        &self,
        _args: &[String],
        _registry: &CommandRegistry,
        _history: Option<&[String]>,
    ) -> CommandResult {
        let history = _history.unwrap_or(&[]);

        let output = history
            .iter()
            .enumerate()
            .map(|(i, cmd)| format!("{:>5}  {}", i + 1, cmd))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(CommandOutput::Message(output))
    }

    fn name(&self) -> &str {
        "history"
    }

    fn description(&self) -> &str {
        "Lists previously executed commands"
    }
}
