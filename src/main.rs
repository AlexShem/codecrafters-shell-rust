mod commands;
mod helpers;
mod path_utils;
mod trie;
mod parser;

use crate::commands::builtins::history::HistoryCommand;
use crate::helpers::ShellHelper;
use commands::{CommandOutput, CommandRegistry};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::Config;
use std::collections::HashMap;
use std::io::{self};
use std::process::Command as ProcessCommand;
use crate::parser::parse_command_line;

fn main() {
    // Create command registry with all available commands
    let registry = CommandRegistry::new();

    let mut helper = ShellHelper::new();

    // Load builtin commands
    for command_name in registry.list_commands() {
        helper.trie.insert(command_name);
    }

    // Load PATH executables
    helper.load_path_executables();

    let config = Config::builder().auto_add_history(true).build();
    let history = rustyline::history::MemHistory::new();

    let mut rl = rustyline::Editor::<ShellHelper, _>::with_history(config, history).unwrap();
    rl.set_helper(Some(helper));
    rl.set_history_ignore_dups(false).unwrap();

    // Load HISTFILE environment variable (if provided) and store it for saving on exit
    let histfile_path = std::env::var("HISTFILE").ok();
    if let Some(ref path) = histfile_path {
        if std::path::Path::new(path).exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                for line in content.lines() {
                    if !line.trim().is_empty() {
                        rl.add_history_entry(line).ok();
                    }
                }
            }
        }
    }

    // Track last appended index per file
    let mut last_appended: HashMap<String, usize> = HashMap::new();

    loop {
        match rl.readline("$ ") {
            Ok(input) => {
                if let Ok(command_line) = parse_command_line(&input) {
                    let command_name = &command_line[0];
                    let args = &command_line[1..];
                    let result = if command_name == "history" {
                        let history_items: Vec<String> = rl
                            .history()
                            .into_iter()
                            .map(|entry| entry.to_string())
                            .collect();
                        registry.execute_with_history(&command_name, &args, Some(&history_items))
                    } else {
                        registry.execute(&command_name, &args)
                    };

                    match result {
                        Ok(output) => {
                            match output {
                                CommandOutput::Success => {}
                                CommandOutput::Message(msg) => println!("{}", msg),
                                CommandOutput::Exit(code) => {
                                    // Save history to HISTFILE before exiting
                                    if let Some(ref path) = histfile_path {
                                        if let Err(e) = save_history_to_file(&rl, path) {
                                            eprintln!("Error saving history: {}", e);
                                        }
                                    }
                                    std::process::exit(code);
                                }
                                CommandOutput::HistoryRead(commands) => {
                                    // Add commands from file to history
                                    for cmd in commands {
                                        rl.add_history_entry(&cmd).ok();
                                    }
                                }
                                CommandOutput::HistoryAppend { path } => {
                                    let history_items: Vec<String> = rl
                                        .history()
                                        .into_iter()
                                        .map(|entry| entry.to_string())
                                        .collect();

                                    let last_idx = last_appended.get(&path).copied().unwrap_or(0);
                                    let commands_to_append = &history_items[last_idx..];

                                    if let Err(e) = HistoryCommand::append_history_file(
                                        &path,
                                        commands_to_append,
                                    ) {
                                        eprintln!("Error appending history to {}: {}", path, e);
                                    } else {
                                        last_appended.insert(path, history_items.len());
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            if let Err(e) = execute_external_program(&command_name, &args) {
                                eprintln!("{}", e);
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => {
                // Save history to HISTFILE before exiting
                if let Some(ref path) = histfile_path {
                    if let Err(e) = save_history_to_file(&rl, path) {
                        eprintln!("Error saving history: {}", e);
                    }
                }
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn save_history_to_file(
    rl: &rustyline::Editor<ShellHelper, rustyline::history::MemHistory>,
    histfile_path: &str,
) -> Result<(), String> {
    let history_items: Vec<String> = rl
        .history()
        .into_iter()
        .map(|entry| entry.to_string())
        .collect();

    HistoryCommand::write_history_file(histfile_path, &history_items)
}

fn execute_external_program(program: &str, args: &[String]) -> Result<(), String> {
    let status = ProcessCommand::new(program).args(args).status();
    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                //
            }
            Ok(())
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Err(format!("{}: command not found", program))
            } else {
                Err(format!("{}: {}", program, e))
            }
        }
    }
}
