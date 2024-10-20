pub mod commands;

use std::io::{self, stdin, stdout, Write};

use colored::Colorize;

pub fn run() -> io::Result<()> {
    loop {
        print!("{}", "> ".blue());
        stdout().flush()?; // Flush stdout and propagate error

        let input = get_input()?;
        let (command, args) = parse_input(&input);

        match command {
            Some("cd") => {
                commands::cd::change_directory(args);
            }
            Some("exit") => {
                break;
            }
            Some(command) => match commands::run::run_command(command, args) {
                Ok(exit_status) if exit_status.success() => {}
                Ok(_) => eprintln!("{}", "Error: command failed".red()),
                Err(e) => eprintln!("{}", format!("Error running command: {}", e).red()),
            },
            None => {
                eprintln!("{}", "Please enter a command.".red());
            }
        }
    }
    Ok(())
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

fn parse_input(input: &str) -> (Option<&str>, impl Iterator<Item = &str>) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next();
    let args = parts;
    (command, args)
}
