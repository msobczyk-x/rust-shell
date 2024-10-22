use std::process::{Child, Command, Stdio};
use std::io;
use std::iter::Iterator;

pub fn run_piped_command<'a>(
    command: &str,
    args: impl Iterator<Item = &'a str>,
    previous_command: Option<Child>,
    more_commands: bool,
) -> io::Result<Child> {
    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
        Stdio::from(output.stdout.unwrap()) // Pipe previous command's output
    });

    let stdout = if more_commands {
        Stdio::piped() // Pipe to next command if necessary
    } else {
        Stdio::inherit() // Otherwise, inherit stdout
    };

    Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
}
