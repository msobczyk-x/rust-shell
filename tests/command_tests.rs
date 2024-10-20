use shell::commands::{cd, run};

#[test]
fn test_cd() {
    let old_dir = std::env::current_dir().unwrap();
    cd::change_directory(vec!["/tmp"].into_iter());
    let new_dir = std::env::current_dir().unwrap();
    assert_eq!(new_dir, std::path::Path::new("/tmp"));
    std::env::set_current_dir(old_dir).unwrap();
}

#[test]
fn test_run_command_success() {
    let result = run::run_command("echo", vec!["Hello"].into_iter());
    assert!(result.is_ok());
}

#[test]
fn test_run_command_failure() {
    let result = run::run_command("non_existent_command", vec![].into_iter());
    assert!(result.is_err());
}
