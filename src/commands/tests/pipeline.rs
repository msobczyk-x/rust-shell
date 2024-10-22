use std::process::{Command, Stdio};

#[test]
fn test_pipeline_ls_grep() {
    let ls_output = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute ls");

    let grep_output = Command::new("grep")
        .arg("main")
        .stdin(ls_output.stdout.unwrap())
        .output()
        .expect("Failed to execute grep");

    let stdout = String::from_utf8_lossy(&grep_output.stdout);
    assert!(stdout.contains("main.rs"));
}

#[test]
fn test_pipeline_echo_wc() {
    let echo_output = Command::new("echo")
        .arg("Hello World")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute echo");

    let wc_output = Command::new("wc")
        .arg("-w")
        .stdin(echo_output.stdout.unwrap())
        .output()
        .expect("Failed to execute wc");

    let stdout = String::from_utf8_lossy(&wc_output.stdout);
    assert!(stdout.trim() == "2");
}
