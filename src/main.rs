mod commands;

use commands::{parse_command_line, CommandOutput, CommandRegistry};
use std::io::{self, Write};
use std::process::Command as ProcessCommand;

fn main() {
    // Create command registry with all available commands
    let registry = CommandRegistry::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Parse command line
        let parsed = parse_command_line(&input);

        if let Some((command_name, args)) = parsed {
            // Execute command through registry
            match registry.execute(&command_name, &args) {
                Ok(output) => match output {
                    CommandOutput::Exit(code) => {
                        std::process::exit(code);
                    }
                    CommandOutput::Success => {
                        // Command succeeded, continue
                    }
                    CommandOutput::Message(msg) => {
                        println!("{}", msg);
                    }
                },
                Err(_) => {
                    if let Err(e) = execute_external_program(&command_name, &args) {
                        eprintln!("{}", e);
                    }
                }
            }
        }
        // Empty input - just show prompt again
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
