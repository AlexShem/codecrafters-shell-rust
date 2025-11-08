use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};
use crate::path_utils::find_in_path;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry) -> CommandResult {
        if args.is_empty() {
            return Err("type: missing argument".to_string());
        }

        let command = &args[0];

        // Step 1: Check if it's a builtin
        if registry.has_command(command) {
            return Ok(CommandOutput::Message(format!(
                "{} is a shell builtin",
                command
            )));
        }

        // Step 2: Search in PATH
        if let Some(path) = find_in_path(command) {
            return Ok(CommandOutput::Message(format!(
                "{} is {}",
                command,
                path.display()
            )));
        }

        // Step 3: Not found
        Ok(CommandOutput::Message(format!("{}: not found", command)))
    }
    fn name(&self) -> &str {
        "type"
    }
    fn description(&self) -> &str {
        "Checks whether a command is a builtin, an executable file, or unrecognized"
    }
}
