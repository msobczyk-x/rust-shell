use std::process::{Command, ExitStatus};

pub fn run_command<'a>(
    command: &str,
    args: impl Iterator<Item = &'a str>,
) -> std::io::Result<ExitStatus> {
    let mut child = Command::new(command).args(args).spawn()?;
    child.wait()
}
