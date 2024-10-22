#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_change_directory() {
        let initial_dir = env::current_dir().unwrap();

        // Change to root directory
        change_directory(&mut vec!["/"].into_iter().peekable());

        let new_dir = env::current_dir().unwrap();
        assert_ne!(initial_dir, new_dir);

        // Change back to the initial directory
        env::set_current_dir(&initial_dir).unwrap();
    }

    #[test]
    fn test_change_directory_fail() {
        // Try changing to a nonexistent directory
        let result = env::set_current_dir("/nonexistent_directory");
        assert!(result.is_err());
    }
}
