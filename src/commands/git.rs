use std::process::Command;
use colored::*;

/// Check if the current directory is a Git repository and return its status
pub fn get_git_status() -> Option<String> {
    // Check if it's a Git repository
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output();

    if output.is_ok() && output.unwrap().status.success() {
        let branch_output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output();

        let branch_name = match branch_output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            }
            _ => "unknown".to_string(),
        };

        // Get the status to determine changes
        let status_output = Command::new("git")
            .arg("status")
            .arg("--porcelain") // Get the status in a simple format
            .output();

        match status_output {
            Ok(status_output) if status_output.status.success() => {
                let status_str = String::from_utf8_lossy(&status_output.stdout);
                let changes: Vec<&str> = status_str.lines().collect();

                // Count added and modified files
                let added_count = changes.iter().filter(|line| line.starts_with("A")).count();
                let modified_count = changes.iter().filter(|line| line.starts_with("M")).count();
                let deleted_count = changes.iter().filter(|line| line.starts_with("D")).count();

                let total_changes = added_count + modified_count + deleted_count;
                Some(format!("git:({})+{}", branch_name.yellow(), total_changes))
            }
            Ok(status_output) => {
                let error_str = String::from_utf8_lossy(&status_output.stderr);
                eprintln!("{}", error_str);
                None
            }
            Err(e) => {
                eprintln!("Failed to run git status: {}", e);
                None
            }
        }
    } else {
        None // Not a Git repository
    }
}
