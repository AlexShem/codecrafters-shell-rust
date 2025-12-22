mod commands;
mod helpers;
mod path_utils;
mod trie;

use crate::helpers::ShellHelper;
use commands::{parse_command_line, CommandOutput, CommandRegistry};
use rustyline::error::ReadlineError;
use rustyline::Config;
use std::io::{self};
use std::process::Command as ProcessCommand;

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

    loop {
        match rl.readline("$ ") {
            Ok(input) => {
                if let Some((command_name, args)) = parse_command_line(&input) {
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
                        Ok(output) => match output {
                            CommandOutput::Success => {}
                            CommandOutput::Message(msg) => println!("{}", msg),
                            CommandOutput::Exit(code) => std::process::exit(code),
                            CommandOutput::HistoryRead(commands) => {
                                // Add commands from file to history
                                for cmd in commands {
                                    rl.add_history_entry(&cmd).ok();
                                }
                            }
                        },
                        Err(_) => {
                            if let Err(e) = execute_external_program(&command_name, &args) {
                                eprintln!("{}", e);
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
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
