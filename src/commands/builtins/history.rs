use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, _args: &[String], _registry: &CommandRegistry) -> CommandResult {
        Ok(CommandOutput::Success)
    }

    fn execute_with_history(
        &self,
        args: &[String],
        _registry: &CommandRegistry,
        history: Option<&[String]>,
    ) -> CommandResult {
        // Parse optional limit argument
        let limit = if args.len() >= 1 {
            match args[0].parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => return Err(format!("history: invalid number of entries: '{}'", args[0])),
            }
        } else {
            None
        };

        let history = history
            .unwrap_or(&[])
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &String)>>();

        // Apply limit if provided (the last 'limit' entries)
        let history = if let Some(lim) = limit {
            let start = history.len().saturating_sub(lim);
            &history[start..]
        } else {
            &history
        };

        let output = history
            .iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_without_limit() {
        let command = HistoryCommand;
        let registry = CommandRegistry::new();

        let history = vec![
            "ls -la".to_string(),
            "cd /home".to_string(),
            "echo Hello".to_string(),
        ];

        // Test without limit
        let result = command
            .execute_with_history(&[], &registry, Some(&history))
            .unwrap();
        if let CommandOutput::Message(output) = result {
            let expected = "    1  ls -la\n    2  cd /home\n    3  echo Hello";
            assert_eq!(output, expected);
        } else {
            panic!("Expected CommandOutput::Message");
        }
    }

    #[test]
    fn test_history_with_limit() {
        let command = HistoryCommand;
        let registry = CommandRegistry::new();
        let history = vec![
            "ls -la".to_string(),
            "cd /home".to_string(),
            "echo Hello".to_string(),
        ];

        // Test with limit of 2
        let result = command
            .execute_with_history(&["2".to_string()], &registry, Some(&history))
            .unwrap();
        if let CommandOutput::Message(output) = result {
            let expected = "    2  cd /home\n    3  echo Hello";
            assert_eq!(output, expected);
        } else {
            panic!("Expected CommandOutput::Message");
        }
    }
}
