pub mod builtins;

use crate::commands::builtins::{echo, exit, pwd, typec};
use std::collections::HashMap;

/// Result type for command execution
pub type CommandResult = Result<CommandOutput, String>;

/// Output from a command execution
#[derive(Debug)]
pub enum CommandOutput {
    /// Command executed successfully, continue shell loop
    Success,
    /// Command executed successfully with output
    Message(String),
    /// Exit the shell with given code
    Exit(i32),
}

/// Trait that all commands must implement
pub trait Command {
    /// Execute the command with given arguments and optional registry access
    fn execute(&self, args: &[String], registry: &CommandRegistry) -> CommandResult;

    /// Get command name
    fn name(&self) -> &str;

    /// Get command description (for help)
    fn description(&self) -> &str {
        ""
    }
}

/// Registry for all available commands
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    /// Create a new command registry with all built-in commands
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };

        // Register all commands here
        registry.register(Box::new(exit::ExitCommand));
        registry.register(Box::new(echo::EchoCommand));
        registry.register(Box::new(typec::TypeCommand));
        registry.register(Box::new(pwd::PwdCommand));

        registry
    }

    /// Register a new command
    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name().to_string(), command);
    }

    /// Execute a command by name with arguments
    pub fn execute(&self, command_name: &str, args: &[String]) -> CommandResult {
        match self.commands.get(command_name) {
            Some(command) => command.execute(args, self),
            None => Err(format!("{}: command not found", command_name)),
        }
    }

    /// Check if a command exists
    pub fn has_command(&self, command_name: &str) -> bool {
        self.commands.contains_key(command_name)
    }

    /// Get all registered command names
    pub fn command_names(&self) -> Vec<&str> {
        self.commands.keys().map(|s| s.as_str()).collect()
    }
}

/// Parse a command line into command name and arguments
pub fn parse_command_line(input: &str) -> Option<(String, Vec<String>)> {
    let parts: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if parts.is_empty() {
        return None;
    }

    let command = parts[0].clone();
    let args = parts[1..].to_vec();

    Some((command, args))
}
