mod commands;
mod helpers;
mod trie;

use crate::helpers::ShellHelper;
use commands::{parse_command_line, CommandOutput, CommandRegistry};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::io::{self};
use std::process::Command as ProcessCommand;

fn main() {
    // Create command registry with all available commands
    let registry = CommandRegistry::new();

    let mut helper = ShellHelper::new();
    for command_name in registry.list_commands() {
        helper.trie.insert(command_name);
    }

    let mut rl = rustyline::Editor::<ShellHelper, DefaultHistory>::new().unwrap();
    rl.set_helper(Some(helper));

    loop {
        match rl.readline("$ ") {
            Ok(input) => {
                if let Some((command_name, args)) = parse_command_line(&input) {
                    match registry.execute(&command_name, &args) {
                        Ok(output) => match output {
                            CommandOutput::Success => {}
                            CommandOutput::Message(msg) => println!("{}", msg),
                            CommandOutput::Exit(code) => std::process::exit(code),
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
