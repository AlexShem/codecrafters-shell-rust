use super::{Command, CommandOutput, CommandResult};

/// Exit command - exits the shell
pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        let exit_code = if args.is_empty() {
            0
        } else {
            // Parse exit code from first argument
            args[0].parse::<i32>().unwrap_or(0)
        };

        Ok(CommandOutput::Exit(exit_code))
    }

    fn name(&self) -> &str {
        "exit"
    }

    fn description(&self) -> &str {
        "Exit the shell"
    }
}
