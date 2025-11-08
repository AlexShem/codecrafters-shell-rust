use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};
use std::env;
use std::path::Path;

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
            return Ok(CommandOutput::Message(format!("{} is {}", command, path)));
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

fn find_in_path(command: &str) -> Option<String> {
    let path_var = env::var("PATH").ok()?;
    let path_separator = if cfg!(windows) { ";" } else { ":" };

    for dir in path_var.split(path_separator) {
        let full_path = Path::new(dir).join(command);
        if !full_path.exists() {
            continue;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&full_path) {
                let permissions = metadata.permissions();
                if permissions.mode() & 0o111 != 0 {
                    // 1111
                    return full_path.to_str().map(|s| s.to_string());
                }
            }
        }

        #[cfg(not(unix))]
        {
            return full_path.to_str().map(|s| s.to_string());
        }
    }

    None
}
