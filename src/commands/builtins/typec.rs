use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry) -> CommandResult {
        let command = &args[0];
        if registry.has_command(command) {
            Ok(CommandOutput::Message(format!("{} is a shell builtin", command)))
        } else {
            Ok(CommandOutput::Message(format!("{}: not found", command)))
        }
    }
    fn name(&self) -> &str {
        "type"
    }
    fn description(&self) -> &str {
        "Checks whether a command is a builtin, an executable file, or unrecognized"
    }
}
