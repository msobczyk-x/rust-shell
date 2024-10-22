use std::process::{Command, Stdio};
use std::io::{Write, Read};

#[test]
fn test_single_command_echo() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shell")) // This runs your compiled shell binary
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start shell");

    // Simulate user input: Run the echo command
    let input = b"echo Hello, World!\nexit\n";
    child.stdin.as_mut().unwrap().write_all(input).expect("Failed to write to stdin");

    // Capture the output
    let mut output = String::new();
    child.stdout.unwrap().read_to_string(&mut output).expect("Failed to read stdout");

    // Ensure the command output is as expected
    assert!(output.contains("Hello, World!"));
}

#[test]
fn test_cd_command() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shell"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start shell");

    // Simulate user input: Change directory and print current directory
    let input = b"cd /\npwd\nexit\n";
    child.stdin.as_mut().unwrap().write_all(input).expect("Failed to write to stdin");

    // Capture the output
    let mut output = String::new();
    child.stdout.unwrap().read_to_string(&mut output).expect("Failed to read stdout");

    // Check that the directory was changed to "/"
    assert!(output.contains("/"));
}

#[test]
fn test_pipeline_commands() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shell"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start shell");

    // Simulate a piped command: echo and pipe into wc -w (word count)
    let input = b"echo Hello World | wc -w\nexit\n";
    child.stdin.as_mut().unwrap().write_all(input).expect("Failed to write to stdin");

    // Capture the output
    let mut output = String::new();
    child.stdout.unwrap().read_to_string(&mut output).expect("Failed to read stdout");

    // Ensure the piped command output is correct ("2" words)
    assert!(output.contains("2"));
}

#[test]
fn test_invalid_command() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shell"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start shell");

    // Simulate user input with an invalid command
    let input = b"invalid_command\nexit\n";
    child.stdin.as_mut().unwrap().write_all(input).expect("Failed to write to stdin");

    // Capture the output
    let mut output = String::new();
    child.stdout.unwrap().read_to_string(&mut output).expect("Failed to read stdout");

    // Check that the shell handled the invalid command
    assert!(output.contains("Failed to start command"));
}

#[test]
fn test_exit_command() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shell"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start shell");

    // Simulate the 'exit' command
    let input = b"exit\n";
    child.stdin.as_mut().unwrap().write_all(input).expect("Failed to write to stdin");

    // Capture the output (expecting no output since we're exiting)
    let mut output = String::new();
    let exit_status = child.wait().expect("Failed to wait on child");

    // Check that the exit status is successful
    assert!(exit_status.success());
}
