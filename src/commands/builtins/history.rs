use crate::commands::{Command, CommandOutput, CommandRegistry, CommandResult};
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct HistoryCommand;

#[derive(Debug, PartialEq)]
enum HistoryAction {
    Display { limit: Option<usize> },
    Read { path: String },
    Write { path: String },
}

impl HistoryCommand {
    fn parse_args(args: &[String]) -> Result<HistoryAction, String> {
        if args.is_empty() {
            return Ok(HistoryAction::Display { limit: None });
        }

        match args[0].as_str() {
            "-r" => {
                if args.len() < 2 {
                    return Err("history: -r requires a file path argument".to_string());
                }
                Ok(HistoryAction::Read {
                    path: args[1].clone(),
                })
            }
            "-w" => {
                if args.len() < 2 {
                    return Err("history: -w requires a file path argument".to_string());
                }
                Ok(HistoryAction::Write {
                    path: args[1].clone(),
                })
            }
            arg => {
                // Try to parse as a number for limit
                match arg.parse::<usize>() {
                    Ok(n) => Ok(HistoryAction::Display { limit: Some(n) }),
                    Err(_) => Err(format!("history: invalid argument: '{}'", arg)),
                }
            }
        }
    }

    fn read_history_file(path: &str) -> Result<Vec<String>, String> {
        if !Path::new(path).exists() {
            return Err(format!(
                "history: cannot open {}: No such file or directory",
                path
            ));
        }

        fs::read_to_string(path)
            .map(|content| {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.to_string())
                    .collect()
            })
            .map_err(|e| format!("history: cannot read {}: {}", path, e))
    }

    fn write_history_file(path: &str, commands: &[String]) -> Result<(), String> {
        let mut file = fs::File::create(path)
            .map_err(|e| format!("history: cannot open {} for writing: {}", path, e))?;
        for cmd in commands {
            writeln!(file, "{}", cmd)
                .map_err(|e| format!("history: cannot write to {}: {}", path, e))?;
        }
        Ok(())
    }

    fn format_history(history: &[(usize, &String)]) -> String {
        history
            .iter()
            .map(|(i, cmd)| format!("{:>5}  {}", i + 1, cmd))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

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
        let action = Self::parse_args(args)?;

        match action {
            HistoryAction::Display { limit } => {
                let history = history
                    .unwrap_or(&[])
                    .iter()
                    .enumerate()
                    .collect::<Vec<(usize, &String)>>();

                let history = if let Some(lim) = limit {
                    let start = history.len().saturating_sub(lim);
                    &history[start..]
                } else {
                    &history
                };

                let output = Self::format_history(history);
                Ok(CommandOutput::Message(output))
            }
            HistoryAction::Read { path } => {
                let file_commands = Self::read_history_file(&path)?;

                // Return a special variant that signals file commands should be added
                Ok(CommandOutput::HistoryRead(file_commands))
            }
            HistoryAction::Write { path } => {
                let history = history.unwrap_or(&[]);

                Self::write_history_file(&path, history)?;

                Ok(CommandOutput::Success)
            }
        }
    }

    fn name(&self) -> &str {
        "history"
    }

    fn description(&self) -> &str {
        "Lists previously executed commands or reads from a history file"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_args_no_args() {
        let result = HistoryCommand::parse_args(&[]);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, HistoryAction::Display { limit: None });
    }

    #[test]
    fn test_parse_args_limit() {
        let result = HistoryCommand::parse_args(&["5".to_string()]);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, HistoryAction::Display { limit: Some(5) });
    }

    #[test]
    fn test_parse_args_read() {
        let result = HistoryCommand::parse_args(&["-r".to_string(), "/path/to/file".to_string()]);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            result,
            HistoryAction::Read {
                path: "/path/to/file".to_string()
            }
        );
    }

    #[test]
    fn test_parse_args_write() {
        let result = HistoryCommand::parse_args(&["-w".to_string(), "/path/to/file".to_string()]);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(
            result,
            HistoryAction::Write {
                path: "/path/to/file".to_string()
            }
        )
    }

    #[test]
    fn test_parse_args_read_missing_path() {
        let result = HistoryCommand::parse_args(&["-r".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_args_write_missing_path() {
        let result = HistoryCommand::parse_args(&["-w".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_args_invalid() {
        let result = HistoryCommand::parse_args(&["invalid".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_history_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "echo hello").unwrap();
        writeln!(temp_file, "echo world").unwrap();
        writeln!(temp_file).unwrap(); // Empty line
        writeln!(temp_file, "ls -la").unwrap();
        temp_file.flush().unwrap();

        let path = temp_file.path().to_str().unwrap();
        let result = HistoryCommand::read_history_file(path).unwrap();

        assert_eq!(result, vec!["echo hello", "echo world", "ls -la"]);
    }

    #[test]
    fn test_write_history_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let commands = vec![
            "cd /home".to_string(),
            "ls -la".to_string(),
            "echo Hello".to_string(),
        ];

        HistoryCommand::write_history_file(path, &commands).unwrap();

        let content = fs::read_to_string(path).unwrap();
        let expected = "cd /home\nls -la\necho Hello\n";
        assert_eq!(content, expected);
    }

    #[test]
    fn test_read_history_file_not_found() {
        let result = HistoryCommand::read_history_file("/nonexistent/file");
        assert!(result.is_err());
    }

    #[test]
    fn test_history_without_limit() {
        let command = HistoryCommand;
        let registry = CommandRegistry::new();
        let history = vec![
            "ls -la".to_string(),
            "cd /home".to_string(),
            "echo Hello".to_string(),
        ];

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

    #[test]
    fn test_history_with_repeated_commands() {
        let command = HistoryCommand;
        let registry = CommandRegistry::new();
        let history = vec![
            "echo Hello".to_string(),
            "echo Hello".to_string(),
            "ls -la".to_string(),
        ];

        let result = command
            .execute_with_history(&[], &registry, Some(&history))
            .unwrap();
        if let CommandOutput::Message(output) = result {
            let expected = "    1  echo Hello\n    2  echo Hello\n    3  ls -la";
            assert_eq!(output, expected);
        } else {
            panic!("Expected CommandOutput::Message");
        }
    }
}
