#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command_ls() {
        let output = run_command("echo", vec!["Hello", "World"].into_iter()).unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Hello World"));
    }

    #[test]
    fn test_run_command_fail() {
        let result = run_command("nonexistent_command", vec![].into_iter());
        assert!(result.is_err());
    }
}
