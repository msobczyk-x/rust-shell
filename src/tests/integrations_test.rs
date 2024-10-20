#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_runs() {
        let result = run();
        assert!(result.is_ok());
    }
}
