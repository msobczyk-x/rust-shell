use std::{process::Child, str::Split, iter::Peekable};
use crate::commands::cd::change_directory;
use crate::commands::runner::run_piped_command;

/// Executes a pipeline of commands connected by pipes.
pub fn execute_pipeline(commands: &mut Peekable<Split<'_, &str>>) -> bool {
    let mut previous_command: Option<Child> = None;

    // Flag to indicate if we should exit
    let mut should_exit = false;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command_name = parts.next().unwrap();
        let args = parts;

        match command_name {
            "cd" => {
                change_directory(&mut args.peekable());
                previous_command = None; // Reset the pipeline
            }
            "exit" => {
                should_exit = true; // Set the flag to indicate exit
                break;
            }
            command => {
                match run_piped_command(command, args, previous_command, commands.peek().is_some()) {
                    Ok(child) => previous_command = Some(child), // Save for next command in pipeline
                    Err(e) => {
                        eprintln!("Error running command: {}", e);
                        previous_command = None; // Reset on failure
                    }
                }
            }
        }
    }

    if let Some(mut final_command) = previous_command {
        final_command.wait().unwrap();
    }

    should_exit // Return the exit flag
}
