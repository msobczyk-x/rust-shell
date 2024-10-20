use colored::*;
use std::{
    env,
    io::{self, stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() -> io::Result<()> {
    loop {
        print!("{}", "> ".blue());
        stdout().flush()?; // Flush stdout and propagate error

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut parts = input.trim().split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => {
                eprintln!("{}", "Please enter a command.".red());
                continue;
            }
        };
        let args = parts; // This is an iterator over the command arguments

        match command {
            "cd" => {
                // Default to '/' as new directory if one was not provided
                let new_dir = args.clone().next().unwrap_or("/");
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", format!("Failed to change directory: {}", e).red());
                }
            }
            "exit" => {
                break;
            }
            _ => match run_command(command, args) {
                Ok(exit_status) if exit_status.success() => {}
                Ok(_) => eprintln!("{}", "Error: command failed".red()),
                Err(e) => eprintln!("{}", format!("Error running command: {}", e).red()),
            },
        }
    }

    Ok(())
}

fn run_command<'a>(
    command: &str,
    args: impl Iterator<Item = &'a str>,
) -> io::Result<std::process::ExitStatus> {
    let mut child = match Command::new(command).args(args).spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("{}", format!("Failed to start command: {}", e).red());
            return Err(e);
        }
    };

    child.wait()
}
