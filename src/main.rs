use std::io::{stdin, stdout, Write};
use shell::commands::pipeline::execute_pipeline;
use shell::commands::pwd::get_current_directory;
use shell::commands::git::get_git_status;
use colored::*;
fn main() {
    loop {
        let current_dir = get_current_directory();
        print!("\n{} ", current_dir.blue());

        if let Some(git_status) = get_git_status() {
            print!("{} ", git_status.green());
        }

        print!("\n> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command_input = input.trim();
        let mut commands = command_input.split(" | ").peekable();
        if execute_pipeline(&mut commands) {
            break;
        }
    }
}
